#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//pub mod weights;
//pub use weights::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::IsType, sp_runtime::RuntimeDebug, BoundedVec};
use orml_traits::{CombineData, OnNewData};
use scale_info::TypeInfo;
// use crate::{Config, MomentOf, TimestampedValueOf};
use cyborg_primitives::{
    oracle::{ProcessStatus, TimestampedValue},
    worker::{WorkerId, WorkerInfoHandler, WorkerStatusType},
};
use frame_support::{traits::Get, LOG_TARGET};
use pallet_timestamp;

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct StatusInstance<BlockNumber> {
    pub is_online: bool,
    pub is_available: bool,
    pub block: BlockNumber,
}

#[derive(
    Default,
    Encode,
    Decode,
    MaxEncodedLen,
    Clone,
    Copy,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    TypeInfo,
)]
pub struct ProcessStatusPercentages<BlockNumber> {
    pub online: u8,
    pub available: u8,
    pub last_block_processed: BlockNumber,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    use frame_system::WeightInfo; //remove later

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html>
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;

        /// Maximum number of blocks or block range used to calculate average status
        #[pallet::constant]
        type MaxBlockRangePeriod: Get<BlockNumberFor<Self>>;

        /// The percentage of active oracle entries needed to determine online status for worker
        #[pallet::constant]
        type ThresholdUptimeStatus: Get<u8>;

        /// Maximum number of status entries by unique oracle feeders for a worker per period
        #[pallet::constant]
        type MaxAggregateParamLength: Get<u32>;

        /// Updates Worker Status for Edge Connect
        type WorkerInfoHandler: WorkerInfoHandler<
            Self::AccountId,
            WorkerId,
            BlockNumberFor<Self>,
            Self::Moment,
        >;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn last_updated_block)]
    pub type LastClearedBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    /// Stores the amount of status entries for a given worker provided for this period
    #[pallet::storage]
    #[pallet::getter(fn worker_status_entries)]
    pub type WorkerStatusEntriesPerPeriod<T: Config> = StorageMap<
        _,
        Twox64Concat,
        (T::AccountId, WorkerId),
        BoundedVec<StatusInstance<BlockNumberFor<T>>, T::MaxAggregateParamLength>,
        ValueQuery,
    >;

    /// Tracks whether an oracle provider has submitted status for a given worker during the current period
    #[pallet::storage]
    #[pallet::getter(fn submitted_per_period)]
    pub type SubmittedPerPeriod<T: Config> =
        StorageMap<_, Twox64Concat, (T::AccountId, (T::AccountId, WorkerId)), bool, ValueQuery>;

    /// Resulting percentages for status uptimes
    #[pallet::storage]
    #[pallet::getter(fn worker_status_resulting_percentages)]
    pub type ResultingWorkerStatusPercentages<T: Config> = StorageMap<
        _,
        Twox64Concat,
        (T::AccountId, WorkerId),
        ProcessStatusPercentages<BlockNumberFor<T>>,
        ValueQuery,
    >;

    /// Resulting status determined by given percentage threshold
    #[pallet::storage]
    #[pallet::getter(fn worker_status_result)]
    pub type ResultingWorkerStatus<T: Config> =
        StorageMap<_, Twox64Concat, (T::AccountId, WorkerId), ProcessStatus, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UpdateFromAggregatedWorkerInfo {
            worker: (T::AccountId, WorkerId),
            online: bool,
            available: bool,
            last_block_processed: BlockNumberFor<T>,
        },
        LastBlockUpdated {
            block_number: BlockNumberFor<T>,
        },
    }

    /// Pallet Errors
    pub enum Error {}

    /// This hook calculates storage values in this pallet updated by the oracle per MaxBlockRangePeriod
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(now: BlockNumberFor<T>) {
            if LastClearedBlock::<T>::get() + T::MaxBlockRangePeriod::get() <= now {
                Self::process_aggregate_data_for_period();
                let clear_result_a = SubmittedPerPeriod::<T>::clear(500, None);
                let clear_result_b = WorkerStatusEntriesPerPeriod::<T>::clear(500, None);
                if clear_result_a.maybe_cursor.is_none() && clear_result_b.maybe_cursor.is_none() {
                    LastClearedBlock::<T>::set(now);
                    Self::deposit_event(Event::LastBlockUpdated { block_number: now });
                }
                log::info!(
                        target: LOG_TARGET,
                                "Clearing map result for SubmittedPerPeriod: {:?}",
                                clear_result_a.deconstruct()
                );
                log::info!(
                        target: LOG_TARGET,
                                "Clearing map result for WorkerStatusEntriesPerPeriod: {:?}",
                                clear_result_b.deconstruct()
                );
            }
        }
    }

    impl<T: Config> Pallet<T> {
        fn process_aggregate_data_for_period() {
            for (key_worker, value_status_vec) in WorkerStatusEntriesPerPeriod::<T>::iter() {
                let mut total_online: u32 = 0;
                let mut total_available: u32 = 0;
                value_status_vec
                    .iter()
                    .for_each(|value: &StatusInstance<BlockNumberFor<T>>| {
                        total_online += if value.is_online { 100 } else { 0 };
                        total_available += if value.is_available { 100 } else { 0 };
                    });
                let online = (total_online / value_status_vec.len() as u32) as u8;
                let available = (total_available / value_status_vec.len() as u32) as u8;
                let current_block = <frame_system::Pallet<T>>::block_number();
                let process_status_percentages = ProcessStatusPercentages {
                    online,
                    available,
                    last_block_processed: current_block,
                };
                ResultingWorkerStatusPercentages::<T>::set(&key_worker, process_status_percentages);

                // Update worker statuses
                let online_status = online >= T::ThresholdUptimeStatus::get();
                let available_status = available >= T::ThresholdUptimeStatus::get();
                ResultingWorkerStatus::<T>::set(
                    key_worker.clone(),
                    ProcessStatus {
                        online: online_status,
                        available: available_status,
                    },
                );
                Self::update_worker_clusters(
                    key_worker,
                    online_status,
                    available_status,
                    current_block,
                );
            }
        }
        /// sends updated worker info to pallets that implement T::WorkerClusterHandler and emits an event
        fn update_worker_clusters(
            key_worker: (T::AccountId, WorkerId),
            online: bool,
            available: bool,
            last_block_processed: BlockNumberFor<T>,
        ) {
            if let Some(mut worker_cluster) = T::WorkerInfoHandler::get_worker_cluster(&key_worker)
            {
                let status = if online {
                    if available {
                        WorkerStatusType::Active
                    } else {
                        WorkerStatusType::Busy
                    }
                } else {
                    WorkerStatusType::Inactive
                };
                worker_cluster.status = status;
                worker_cluster.status_last_updated = last_block_processed;

                T::WorkerInfoHandler::update_worker_cluster(&key_worker, worker_cluster);

                Self::deposit_event(Event::UpdateFromAggregatedWorkerInfo {
                    worker: key_worker,
                    online,
                    available,
                    last_block_processed,
                });
            } else {
                log::warn!("Worker cluster not found for the given account and worker_id.");
            }
        }

        // This public wrapper function is exposed only when the `runtime-benchmarks` feature is enabled.
        // It allows access to the private `derive_status_percentages_for_period` function for benchmarking purposes.
        // The feature gate ensures that this function is only available in benchmarking builds and not in normal runtime builds,
        // keeping the core functionality private while still enabling performance measurements.
        #[cfg(feature = "runtime-benchmarks")]
        pub fn benchmark_derive_status_percentages_for_period() {
            Self::derive_status_percentages_for_period();
        }
    }

    /// Data from the oracle first enters into this pallet through this trait implmentation and updates this pallet's storage
    impl<T: Config> OnNewData<T::AccountId, (T::AccountId, u64), ProcessStatus> for Pallet<T> {
        fn on_new_data(who: &T::AccountId, key: &(T::AccountId, u64), value: &ProcessStatus) {
            if T::WorkerInfoHandler::get_worker_cluster(key).is_none() {
                log::error!(
                    target: LOG_TARGET,
                    "No worker registed by this key: {:?}",
                    key
                );
                return;
            }
            if SubmittedPerPeriod::<T>::get((who, key)) {
                log::error!(
                    target: LOG_TARGET,
                    "A value for this period was already submitted by: {:?}",
                    who
                );
                return;
            }
            WorkerStatusEntriesPerPeriod::<T>::mutate(key, |status_vec| {
                match status_vec.try_push(StatusInstance {
                    is_online: value.online,
                    is_available: value.available,
                    block: <frame_system::Pallet<T>>::block_number(),
                }) {
                    Ok(()) => {
                        log::info!(
                            target: LOG_TARGET,
                            "Successfully push status instance value for period. \
                            Value was submitted by: {:?}",
                            who
                        );
                    }
                    Err(_) => {
                        log::error!(
                        target: LOG_TARGET,
                                "Failed to push status instance value due to exceeded capacity. \
                                Value was submitted by: {:?}",
                                who
                        );
                    }
                }
            });
            SubmittedPerPeriod::<T>::set((who, key), true);
        }
    }

    impl<T: Config + orml_oracle::Config> CombineData<(T::AccountId, WorkerId), TimestampedValue<T>>
        for Pallet<T>
    {
        fn combine_data(
            _key: &(T::AccountId, WorkerId),
            _values: Vec<TimestampedValue<T>>,
            _prev_value: Option<TimestampedValue<T>>,
        ) -> Option<TimestampedValue<T>> {
            None
        }
    }
}
