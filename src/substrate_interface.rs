#[allow(dead_code, unused_imports, non_camel_case_types, unreachable_patterns)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 23usize] = [
		"System",
		"ParachainSystem",
		"Timestamp",
		"ParachainInfo",
		"Balances",
		"TransactionPayment",
		"Sudo",
		"Authorship",
		"CollatorSelection",
		"Session",
		"Aura",
		"AuraExt",
		"XcmpQueue",
		"PolkadotXcm",
		"CumulusXcm",
		"MessageQueue",
		"Oracle",
		"OracleMembership",
		"EdgeConnect",
		"TaskManagement",
		"StatusAggregator",
		"Payment",
		"ZKVerifier",
	];
	pub static RUNTIME_APIS: [&str; 0usize] = [];
	#[doc = r" The error type that is returned when there is a runtime issue."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	#[doc = r" The outer event enum."]
	pub type Event = runtime_types::cyborg_runtime::RuntimeEvent;
	#[doc = r" The outer extrinsic enum."]
	pub type Call = runtime_types::cyborg_runtime::RuntimeCall;
	#[doc = r" The outer error enum represents the DispatchError's Module variant."]
	pub type Error = runtime_types::cyborg_runtime::RuntimeError;
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
	}
	pub fn apis() -> runtime_apis::RuntimeApi {
		runtime_apis::RuntimeApi
	}
	pub mod runtime_apis {
		use super::root_mod;
		use super::runtime_types;
		use subxt::ext::subxt_core::ext::codec::Encode;
		pub struct RuntimeApi;
		impl RuntimeApi {}
	}
	pub fn custom() -> CustomValuesApi {
		CustomValuesApi
	}
	pub struct CustomValuesApi;
	impl CustomValuesApi {}
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi {
			system::constants::ConstantsApi
		}
		pub fn parachain_system(&self) -> parachain_system::constants::ConstantsApi {
			parachain_system::constants::ConstantsApi
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn balances(&self) -> balances::constants::ConstantsApi {
			balances::constants::ConstantsApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}
		pub fn aura(&self) -> aura::constants::ConstantsApi {
			aura::constants::ConstantsApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::constants::ConstantsApi {
			xcmp_queue::constants::ConstantsApi
		}
		pub fn message_queue(&self) -> message_queue::constants::ConstantsApi {
			message_queue::constants::ConstantsApi
		}
		pub fn oracle(&self) -> oracle::constants::ConstantsApi {
			oracle::constants::ConstantsApi
		}
		pub fn status_aggregator(&self) -> status_aggregator::constants::ConstantsApi {
			status_aggregator::constants::ConstantsApi
		}
		pub fn zk_verifier(&self) -> zk_verifier::constants::ConstantsApi {
			zk_verifier::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn parachain_system(&self) -> parachain_system::storage::StorageApi {
			parachain_system::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn parachain_info(&self) -> parachain_info::storage::StorageApi {
			parachain_info::storage::StorageApi
		}
		pub fn balances(&self) -> balances::storage::StorageApi {
			balances::storage::StorageApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}
		pub fn collator_selection(&self) -> collator_selection::storage::StorageApi {
			collator_selection::storage::StorageApi
		}
		pub fn session(&self) -> session::storage::StorageApi {
			session::storage::StorageApi
		}
		pub fn aura(&self) -> aura::storage::StorageApi {
			aura::storage::StorageApi
		}
		pub fn aura_ext(&self) -> aura_ext::storage::StorageApi {
			aura_ext::storage::StorageApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi {
			xcmp_queue::storage::StorageApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::storage::StorageApi {
			polkadot_xcm::storage::StorageApi
		}
		pub fn message_queue(&self) -> message_queue::storage::StorageApi {
			message_queue::storage::StorageApi
		}
		pub fn oracle(&self) -> oracle::storage::StorageApi {
			oracle::storage::StorageApi
		}
		pub fn oracle_membership(&self) -> oracle_membership::storage::StorageApi {
			oracle_membership::storage::StorageApi
		}
		pub fn edge_connect(&self) -> edge_connect::storage::StorageApi {
			edge_connect::storage::StorageApi
		}
		pub fn task_management(&self) -> task_management::storage::StorageApi {
			task_management::storage::StorageApi
		}
		pub fn status_aggregator(&self) -> status_aggregator::storage::StorageApi {
			status_aggregator::storage::StorageApi
		}
		pub fn payment(&self) -> payment::storage::StorageApi {
			payment::storage::StorageApi
		}
		pub fn zk_verifier(&self) -> zk_verifier::storage::StorageApi {
			zk_verifier::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi {
			parachain_system::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn parachain_info(&self) -> parachain_info::calls::TransactionApi {
			parachain_info::calls::TransactionApi
		}
		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
		pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi {
			collator_selection::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi {
			xcmp_queue::calls::TransactionApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi {
			polkadot_xcm::calls::TransactionApi
		}
		pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi {
			cumulus_xcm::calls::TransactionApi
		}
		pub fn message_queue(&self) -> message_queue::calls::TransactionApi {
			message_queue::calls::TransactionApi
		}
		pub fn oracle(&self) -> oracle::calls::TransactionApi {
			oracle::calls::TransactionApi
		}
		pub fn oracle_membership(&self) -> oracle_membership::calls::TransactionApi {
			oracle_membership::calls::TransactionApi
		}
		pub fn edge_connect(&self) -> edge_connect::calls::TransactionApi {
			edge_connect::calls::TransactionApi
		}
		pub fn task_management(&self) -> task_management::calls::TransactionApi {
			task_management::calls::TransactionApi
		}
		pub fn payment(&self) -> payment::calls::TransactionApi {
			payment::calls::TransactionApi
		}
		pub fn zk_verifier(&self) -> zk_verifier::calls::TransactionApi {
			zk_verifier::calls::TransactionApi
		}
	}
	#[doc = r" check whether the metadata provided is aligned with this statically generated code."]
	pub fn is_codegen_valid_for(metadata: &::subxt::ext::subxt_core::Metadata) -> bool {
		let runtime_metadata_hash = metadata
			.hasher()
			.only_these_pallets(&PALLETS)
			.only_these_runtime_apis(&RUNTIME_APIS)
			.hash();
		runtime_metadata_hash
			== [
				212u8, 123u8, 250u8, 231u8, 87u8, 23u8, 14u8, 68u8, 52u8, 143u8, 146u8, 189u8, 24u8, 244u8,
				125u8, 143u8, 1u8, 24u8, 15u8, 251u8, 46u8, 44u8, 62u8, 118u8, 105u8, 8u8, 73u8, 78u8,
				43u8, 32u8, 184u8, 117u8,
			]
	}
	pub mod system {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the System pallet"]
		pub type Error = runtime_types::frame_system::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::frame_system::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "Can be executed by every `origin`."]
				pub struct Remark {
					pub remark: remark::Remark,
				}
				pub mod remark {
					use super::runtime_types;
					pub type Remark = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Remark {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub struct SetHeapPages {
					pub pages: set_heap_pages::Pages,
				}
				pub mod set_heap_pages {
					use super::runtime_types;
					pub type Pages = ::core::primitive::u64;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetHeapPages {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_heap_pages";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the new runtime code."]
				pub struct SetCode {
					pub code: set_code::Code,
				}
				pub mod set_code {
					use super::runtime_types;
					pub type Code = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCode {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
				#[doc = "version!"]
				pub struct SetCodeWithoutChecks {
					pub code: set_code_without_checks::Code,
				}
				pub mod set_code_without_checks {
					use super::runtime_types;
					pub type Code = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCodeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code_without_checks";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set some items of storage."]
				pub struct SetStorage {
					pub items: set_storage::Items,
				}
				pub mod set_storage {
					use super::runtime_types;
					pub type Items = ::subxt::ext::subxt_core::alloc::vec::Vec<(
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					)>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_storage";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Kill some items from storage."]
				pub struct KillStorage {
					pub keys: kill_storage::Keys,
				}
				pub mod kill_storage {
					use super::runtime_types;
					pub type Keys = ::subxt::ext::subxt_core::alloc::vec::Vec<
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_storage";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub struct KillPrefix {
					pub prefix: kill_prefix::Prefix,
					pub subkeys: kill_prefix::Subkeys,
				}
				pub mod kill_prefix {
					use super::runtime_types;
					pub type Prefix = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
					pub type Subkeys = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillPrefix {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_prefix";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Make some on-chain remark and emit event."]
				pub struct RemarkWithEvent {
					pub remark: remark_with_event::Remark,
				}
				pub mod remark_with_event {
					use super::runtime_types;
					pub type Remark = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemarkWithEvent {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark_with_event";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub struct AuthorizeUpgrade {
					pub code_hash: authorize_upgrade::CodeHash,
				}
				pub mod authorize_upgrade {
					use super::runtime_types;
					pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgrade {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "authorize_upgrade";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
				#[doc = "example that the spec name remains the same and that the version number increases. Not"]
				#[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub struct AuthorizeUpgradeWithoutChecks {
					pub code_hash: authorize_upgrade_without_checks::CodeHash,
				}
				pub mod authorize_upgrade_without_checks {
					use super::runtime_types;
					pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgradeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "authorize_upgrade_without_checks";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
				#[doc = ""]
				#[doc = "If the authorization required a version check, this call will ensure the spec name"]
				#[doc = "remains unchanged and that the spec version has increased."]
				#[doc = ""]
				#[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
				#[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
				#[doc = ""]
				#[doc = "All origins are allowed."]
				pub struct ApplyAuthorizedUpgrade {
					pub code: apply_authorized_upgrade::Code,
				}
				pub mod apply_authorized_upgrade {
					use super::runtime_types;
					pub type Code = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ApplyAuthorizedUpgrade {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "apply_authorized_upgrade";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "Can be executed by every `origin`."]
				pub fn remark(
					&self,
					remark: types::remark::Remark,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Remark> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"remark",
						types::Remark { remark },
						[
							43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8, 216u8, 98u8,
							100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8, 250u8, 147u8, 208u8, 2u8,
							40u8, 129u8, 209u8, 232u8, 207u8, 207u8, 13u8,
						],
					)
				}
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub fn set_heap_pages(
					&self,
					pages: types::set_heap_pages::Pages,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetHeapPages> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"set_heap_pages",
						types::SetHeapPages { pages },
						[
							188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8, 215u8, 242u8,
							195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8, 134u8, 121u8, 81u8, 209u8, 127u8,
							223u8, 98u8, 215u8, 150u8, 70u8, 57u8, 147u8,
						],
					)
				}
				#[doc = "Set the new runtime code."]
				pub fn set_code(
					&self,
					code: types::set_code::Code,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCode> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"set_code",
						types::SetCode { code },
						[
							233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8, 203u8, 136u8,
							160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8, 27u8, 147u8, 147u8, 236u8,
							65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
						],
					)
				}
				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
				#[doc = "version!"]
				pub fn set_code_without_checks(
					&self,
					code: types::set_code_without_checks::Code,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCodeWithoutChecks> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"set_code_without_checks",
						types::SetCodeWithoutChecks { code },
						[
							82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8, 157u8, 141u8,
							42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8, 147u8, 15u8, 178u8, 247u8, 229u8,
							213u8, 98u8, 207u8, 231u8, 119u8, 115u8,
						],
					)
				}
				#[doc = "Set some items of storage."]
				pub fn set_storage(
					&self,
					items: types::set_storage::Items,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetStorage> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"set_storage",
						types::SetStorage { items },
						[
							141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8, 163u8, 102u8,
							229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8, 150u8, 76u8, 78u8, 137u8,
							126u8, 88u8, 183u8, 88u8, 231u8, 146u8, 234u8, 43u8,
						],
					)
				}
				#[doc = "Kill some items from storage."]
				pub fn kill_storage(
					&self,
					keys: types::kill_storage::Keys,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillStorage> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"kill_storage",
						types::KillStorage { keys },
						[
							73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8, 234u8, 153u8,
							185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8, 156u8, 63u8, 47u8, 228u8, 249u8,
							216u8, 139u8, 143u8, 177u8, 41u8, 35u8,
						],
					)
				}
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub fn kill_prefix(
					&self,
					prefix: types::kill_prefix::Prefix,
					subkeys: types::kill_prefix::Subkeys,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillPrefix> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"kill_prefix",
						types::KillPrefix { prefix, subkeys },
						[
							184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8, 175u8, 242u8,
							167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8, 67u8, 236u8, 111u8, 110u8,
							234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8, 85u8,
						],
					)
				}
				#[doc = "Make some on-chain remark and emit event."]
				pub fn remark_with_event(
					&self,
					remark: types::remark_with_event::Remark,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemarkWithEvent> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"remark_with_event",
						types::RemarkWithEvent { remark },
						[
							120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8, 228u8, 233u8,
							130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8, 147u8, 170u8, 115u8, 91u8,
							149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
						],
					)
				}
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub fn authorize_upgrade(
					&self,
					code_hash: types::authorize_upgrade::CodeHash,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AuthorizeUpgrade> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"authorize_upgrade",
						types::AuthorizeUpgrade { code_hash },
						[
							4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8, 254u8, 170u8,
							214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8, 58u8, 145u8, 131u8, 109u8,
							63u8, 38u8, 165u8, 107u8, 215u8, 217u8, 172u8,
						],
					)
				}
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
				#[doc = "example that the spec name remains the same and that the version number increases. Not"]
				#[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub fn authorize_upgrade_without_checks(
					&self,
					code_hash: types::authorize_upgrade_without_checks::CodeHash,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
					types::AuthorizeUpgradeWithoutChecks,
				> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"authorize_upgrade_without_checks",
						types::AuthorizeUpgradeWithoutChecks { code_hash },
						[
							126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8, 136u8, 146u8,
							14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8, 197u8, 104u8, 184u8, 185u8,
							161u8, 99u8, 154u8, 80u8, 125u8, 181u8, 233u8,
						],
					)
				}
				#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
				#[doc = ""]
				#[doc = "If the authorization required a version check, this call will ensure the spec name"]
				#[doc = "remains unchanged and that the spec version has increased."]
				#[doc = ""]
				#[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
				#[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
				#[doc = ""]
				#[doc = "All origins are allowed."]
				pub fn apply_authorized_upgrade(
					&self,
					code: types::apply_authorized_upgrade::Code,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ApplyAuthorizedUpgrade>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"System",
						"apply_authorized_upgrade",
						types::ApplyAuthorizedUpgrade { code },
						[
							232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8, 222u8, 156u8, 245u8,
							102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8, 125u8, 32u8, 124u8, 101u8, 108u8, 93u8,
							211u8, 52u8, 0u8, 231u8, 33u8, 227u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the System pallet."]
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An extrinsic completed successfully."]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: extrinsic_success::DispatchInfo,
			}
			pub mod extrinsic_success {
				use super::runtime_types;
				pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An extrinsic failed."]
			pub struct ExtrinsicFailed {
				pub dispatch_error: extrinsic_failed::DispatchError,
				pub dispatch_info: extrinsic_failed::DispatchInfo,
			}
			pub mod extrinsic_failed {
				use super::runtime_types;
				pub type DispatchError = runtime_types::sp_runtime::DispatchError;
				pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "`:code` was updated."]
			pub struct CodeUpdated;
			impl ::subxt::ext::subxt_core::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A new account was created."]
			pub struct NewAccount {
				pub account: new_account::Account,
			}
			pub mod new_account {
				use super::runtime_types;
				pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An account was reaped."]
			pub struct KilledAccount {
				pub account: killed_account::Account,
			}
			pub mod killed_account {
				use super::runtime_types;
				pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "On on-chain remark happened."]
			pub struct Remarked {
				pub sender: remarked::Sender,
				pub hash: remarked::Hash,
			}
			pub mod remarked {
				use super::runtime_types;
				pub type Sender = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Hash = ::subxt::ext::subxt_core::utils::H256;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An upgrade was authorized."]
			pub struct UpgradeAuthorized {
				pub code_hash: upgrade_authorized::CodeHash,
				pub check_version: upgrade_authorized::CheckVersion,
			}
			pub mod upgrade_authorized {
				use super::runtime_types;
				pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
				pub type CheckVersion = ::core::primitive::bool;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for UpgradeAuthorized {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod account {
					use super::runtime_types;
					pub type Account = runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod extrinsic_count {
					use super::runtime_types;
					pub type ExtrinsicCount = ::core::primitive::u32;
				}
				pub mod inherents_applied {
					use super::runtime_types;
					pub type InherentsApplied = ::core::primitive::bool;
				}
				pub mod block_weight {
					use super::runtime_types;
					pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::sp_weights::weight_v2::Weight,
					>;
				}
				pub mod all_extrinsics_len {
					use super::runtime_types;
					pub type AllExtrinsicsLen = ::core::primitive::u32;
				}
				pub mod block_hash {
					use super::runtime_types;
					pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod extrinsic_data {
					use super::runtime_types;
					pub type ExtrinsicData = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod number {
					use super::runtime_types;
					pub type Number = ::core::primitive::u32;
				}
				pub mod parent_hash {
					use super::runtime_types;
					pub type ParentHash = ::subxt::ext::subxt_core::utils::H256;
				}
				pub mod digest {
					use super::runtime_types;
					pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
				}
				pub mod events {
					use super::runtime_types;
					pub type Events = ::subxt::ext::subxt_core::alloc::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::cyborg_runtime::RuntimeEvent,
							::subxt::ext::subxt_core::utils::H256,
						>,
					>;
				}
				pub mod event_count {
					use super::runtime_types;
					pub type EventCount = ::core::primitive::u32;
				}
				pub mod event_topics {
					use super::runtime_types;
					pub type EventTopics = ::subxt::ext::subxt_core::alloc::vec::Vec<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
				}
				pub mod last_runtime_upgrade {
					use super::runtime_types;
					pub type LastRuntimeUpgrade = runtime_types::frame_system::LastRuntimeUpgradeInfo;
				}
				pub mod upgraded_to_u32_ref_count {
					use super::runtime_types;
					pub type UpgradedToU32RefCount = ::core::primitive::bool;
				}
				pub mod upgraded_to_triple_ref_count {
					use super::runtime_types;
					pub type UpgradedToTripleRefCount = ::core::primitive::bool;
				}
				pub mod execution_phase {
					use super::runtime_types;
					pub type ExecutionPhase = runtime_types::frame_system::Phase;
				}
				pub mod authorized_upgrade {
					use super::runtime_types;
					pub type AuthorizedUpgrade = runtime_types::frame_system::CodeUpgradeAuthorization;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The full account information for a particular account ID."]
				pub fn account_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::account::Account,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"Account",
						(),
						[
							14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8, 175u8, 209u8,
							79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8, 124u8, 90u8, 158u8, 85u8, 45u8,
							37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
						],
					)
				}
				#[doc = " The full account information for a particular account ID."]
				pub fn account(
					&self,
					_0: impl ::core::borrow::Borrow<types::account::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::account::Param0>,
					types::account::Account,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"Account",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8, 175u8, 209u8,
							79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8, 124u8, 90u8, 158u8, 85u8, 45u8,
							37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
						],
					)
				}
				#[doc = " Total extrinsics count for the current block."]
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::extrinsic_count::ExtrinsicCount,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"ExtrinsicCount",
						(),
						[
							102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8, 153u8, 148u8,
							234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8, 120u8, 47u8, 137u8, 254u8,
							96u8, 100u8, 165u8, 182u8, 249u8, 230u8, 159u8, 79u8,
						],
					)
				}
				#[doc = " Whether all inherents have been applied."]
				pub fn inherents_applied(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::inherents_applied::InherentsApplied,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"InherentsApplied",
						(),
						[
							132u8, 249u8, 142u8, 252u8, 8u8, 103u8, 80u8, 120u8, 50u8, 6u8, 188u8, 223u8, 101u8,
							55u8, 165u8, 189u8, 172u8, 249u8, 165u8, 230u8, 183u8, 109u8, 34u8, 65u8, 185u8,
							150u8, 29u8, 8u8, 186u8, 129u8, 135u8, 239u8,
						],
					)
				}
				#[doc = " The current weight for the block."]
				pub fn block_weight(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::block_weight::BlockWeight,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"BlockWeight",
						(),
						[
							158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8, 62u8, 43u8,
							42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8, 229u8, 30u8, 216u8, 255u8,
							165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
						],
					)
				}
				#[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::all_extrinsics_len::AllExtrinsicsLen,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"AllExtrinsicsLen",
						(),
						[
							117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8, 243u8, 185u8,
							122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8, 101u8, 218u8, 203u8, 201u8,
							237u8, 254u8, 128u8, 183u8, 169u8, 221u8, 242u8, 65u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::block_hash::BlockHash,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"BlockHash",
						(),
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8, 103u8, 100u8,
							195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8, 164u8, 16u8, 20u8, 222u8, 28u8,
							214u8, 144u8, 142u8, 146u8, 69u8, 202u8, 118u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash(
					&self,
					_0: impl ::core::borrow::Borrow<types::block_hash::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::block_hash::Param0>,
					types::block_hash::BlockHash,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"BlockHash",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8, 103u8, 100u8,
							195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8, 164u8, 16u8, 20u8, 222u8, 28u8,
							214u8, 144u8, 142u8, 146u8, 69u8, 202u8, 118u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::extrinsic_data::ExtrinsicData,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"ExtrinsicData",
						(),
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8, 220u8, 106u8,
							245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8, 128u8, 61u8, 170u8, 137u8,
							231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data(
					&self,
					_0: impl ::core::borrow::Borrow<types::extrinsic_data::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::extrinsic_data::Param0,
					>,
					types::extrinsic_data::ExtrinsicData,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"ExtrinsicData",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8, 220u8, 106u8,
							245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8, 128u8, 61u8, 170u8, 137u8,
							231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub fn number(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::number::Number,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"Number",
						(),
						[
							30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8, 9u8, 8u8,
							8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8, 200u8, 78u8, 93u8, 115u8,
							28u8, 150u8, 113u8, 48u8, 53u8,
						],
					)
				}
				#[doc = " Hash of the previous block."]
				pub fn parent_hash(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::parent_hash::ParentHash,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"ParentHash",
						(),
						[
							26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8, 192u8, 62u8,
							93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8, 71u8, 82u8, 141u8, 229u8,
							32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
						],
					)
				}
				#[doc = " Digest of the current block, also part of the block header."]
				pub fn digest(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::digest::Digest,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"Digest",
						(),
						[
							61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8, 91u8, 51u8,
							140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8, 58u8, 92u8, 123u8, 141u8,
							14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
						],
					)
				}
				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
				#[doc = " It could otherwise inflate the PoV size of a block."]
				#[doc = ""]
				#[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
				#[doc = " just in case someone still reads them from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::events::Events,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"Events",
						(),
						[
							148u8, 195u8, 105u8, 29u8, 23u8, 37u8, 69u8, 172u8, 25u8, 239u8, 107u8, 136u8, 100u8,
							81u8, 43u8, 218u8, 24u8, 4u8, 70u8, 52u8, 8u8, 92u8, 234u8, 67u8, 98u8, 237u8, 217u8,
							107u8, 209u8, 192u8, 23u8, 217u8,
						],
					)
				}
				#[doc = " The number of events in the `Events<T>` list."]
				pub fn event_count(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::event_count::EventCount,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"EventCount",
						(),
						[
							175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8, 151u8, 205u8,
							189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8, 254u8, 131u8, 13u8, 143u8, 3u8,
							244u8, 245u8, 45u8, 2u8, 210u8, 79u8, 133u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::event_topics::EventTopics,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"EventTopics",
						(),
						[
							40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8, 133u8, 114u8,
							13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8, 120u8, 241u8, 48u8, 106u8,
							143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics(
					&self,
					_0: impl ::core::borrow::Borrow<types::event_topics::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::event_topics::Param0>,
					types::event_topics::EventTopics,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"EventTopics",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8, 133u8, 114u8,
							13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8, 120u8, 241u8, 48u8, 106u8,
							143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
						],
					)
				}
				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::last_runtime_upgrade::LastRuntimeUpgrade,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"LastRuntimeUpgrade",
						(),
						[
							137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8, 148u8, 68u8, 91u8,
							140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8, 194u8, 253u8, 30u8, 163u8, 39u8,
							54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"UpgradedToU32RefCount",
						(),
						[
							229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8, 130u8, 52u8,
							146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8, 107u8, 124u8, 31u8, 2u8, 22u8,
							86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"UpgradedToTripleRefCount",
						(),
						[
							97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8, 101u8, 24u8, 40u8,
							231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8, 167u8, 82u8, 254u8, 189u8, 3u8,
							101u8, 207u8, 206u8, 194u8, 155u8, 151u8,
						],
					)
				}
				#[doc = " The execution phase of the block."]
				pub fn execution_phase(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::execution_phase::ExecutionPhase,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"ExecutionPhase",
						(),
						[
							191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8, 0u8, 26u8,
							161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8, 35u8, 36u8, 253u8, 52u8,
							235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
						],
					)
				}
				#[doc = " `Some` if a code upgrade has been authorized."]
				pub fn authorized_upgrade(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::authorized_upgrade::AuthorizedUpgrade,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"System",
						"AuthorizedUpgrade",
						(),
						[
							165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8, 169u8, 55u8,
							178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8, 214u8, 213u8, 251u8, 123u8,
							5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
					runtime_types::frame_system::limits::BlockWeights,
				> {
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"System",
						"BlockWeights",
						[
							176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8, 190u8, 127u8,
							102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8, 163u8, 177u8, 104u8, 59u8,
							60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
						],
					)
				}
				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
					runtime_types::frame_system::limits::BlockLength,
				> {
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"System",
						"BlockLength",
						[
							23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8, 229u8, 185u8,
							133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8, 96u8, 166u8, 235u8, 162u8,
							160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
						],
					)
				}
				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"System",
						"BlockHashCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
					runtime_types::sp_weights::RuntimeDbWeight,
				> {
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"System",
						"DbWeight",
						[
							42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8, 200u8, 170u8,
							102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8, 183u8, 76u8, 74u8, 10u8, 70u8,
							243u8, 14u8, 218u8, 213u8, 126u8, 29u8, 177u8,
						],
					)
				}
				#[doc = " Get the chain's in-code version."]
				pub fn version(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
					runtime_types::sp_version::RuntimeVersion,
				> {
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"System",
						"Version",
						[
							219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8, 228u8, 83u8,
							111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8, 72u8, 205u8, 74u8, 242u8,
							233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8, 165u8,
						],
					)
				}
				#[doc = " The designated SS58 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u16>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"System",
						"SS58Prefix",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8, 41u8, 144u8, 11u8,
							236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8, 90u8, 208u8, 135u8, 15u8, 117u8,
							10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_system {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::cumulus_pallet_parachain_system::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::cumulus_pallet_parachain_system::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the current validation data."]
				#[doc = ""]
				#[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase if the call was not invoked."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`"]
				#[doc = ""]
				#[doc = "As a side effect, this function upgrades the current validation function"]
				#[doc = "if the appropriate time has come."]
				pub struct SetValidationData {
					pub data: set_validation_data::Data,
				}
				pub mod set_validation_data {
					use super::runtime_types;
					pub type Data =
						runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetValidationData {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "set_validation_data";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct SudoSendUpwardMessage {
					pub message: sudo_send_upward_message::Message,
				}
				pub mod sudo_send_upward_message {
					use super::runtime_types;
					pub type Message = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoSendUpwardMessage {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "sudo_send_upward_message";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "The `check_version` parameter sets a boolean flag for whether or not the runtime's spec"]
				#[doc = "version and name should be verified on upgrade. Since the authorization only has a hash,"]
				#[doc = "it cannot actually perform the verification."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub struct AuthorizeUpgrade {
					pub code_hash: authorize_upgrade::CodeHash,
					pub check_version: authorize_upgrade::CheckVersion,
				}
				pub mod authorize_upgrade {
					use super::runtime_types;
					pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
					pub type CheckVersion = ::core::primitive::bool;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgrade {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "authorize_upgrade";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
				#[doc = ""]
				#[doc = "If the authorization required a version check, this call will ensure the spec name"]
				#[doc = "remains unchanged and that the spec version has increased."]
				#[doc = ""]
				#[doc = "Note that this function will not apply the new `code`, but only attempt to schedule the"]
				#[doc = "upgrade with the Relay Chain."]
				#[doc = ""]
				#[doc = "All origins are allowed."]
				pub struct EnactAuthorizedUpgrade {
					pub code: enact_authorized_upgrade::Code,
				}
				pub mod enact_authorized_upgrade {
					use super::runtime_types;
					pub type Code = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for EnactAuthorizedUpgrade {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "enact_authorized_upgrade";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current validation data."]
				#[doc = ""]
				#[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase if the call was not invoked."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`"]
				#[doc = ""]
				#[doc = "As a side effect, this function upgrades the current validation function"]
				#[doc = "if the appropriate time has come."]
				pub fn set_validation_data(
					&self,
					data: types::set_validation_data::Data,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetValidationData> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"ParachainSystem",
						"set_validation_data",
						types::SetValidationData { data },
						[
							167u8, 126u8, 75u8, 137u8, 220u8, 60u8, 106u8, 214u8, 92u8, 170u8, 136u8, 176u8,
							98u8, 0u8, 234u8, 217u8, 146u8, 113u8, 149u8, 88u8, 114u8, 141u8, 228u8, 105u8,
							136u8, 71u8, 233u8, 18u8, 70u8, 36u8, 24u8, 249u8,
						],
					)
				}
				pub fn sudo_send_upward_message(
					&self,
					message: types::sudo_send_upward_message::Message,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoSendUpwardMessage>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"ParachainSystem",
						"sudo_send_upward_message",
						types::SudoSendUpwardMessage { message },
						[
							1u8, 231u8, 11u8, 78u8, 127u8, 117u8, 248u8, 67u8, 230u8, 199u8, 126u8, 47u8, 20u8,
							62u8, 252u8, 138u8, 199u8, 48u8, 41u8, 21u8, 28u8, 157u8, 218u8, 143u8, 4u8, 253u8,
							62u8, 192u8, 94u8, 252u8, 92u8, 180u8,
						],
					)
				}
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "The `check_version` parameter sets a boolean flag for whether or not the runtime's spec"]
				#[doc = "version and name should be verified on upgrade. Since the authorization only has a hash,"]
				#[doc = "it cannot actually perform the verification."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub fn authorize_upgrade(
					&self,
					code_hash: types::authorize_upgrade::CodeHash,
					check_version: types::authorize_upgrade::CheckVersion,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AuthorizeUpgrade> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"ParachainSystem",
						"authorize_upgrade",
						types::AuthorizeUpgrade {
							code_hash,
							check_version,
						},
						[
							213u8, 114u8, 107u8, 169u8, 223u8, 147u8, 205u8, 204u8, 3u8, 81u8, 228u8, 0u8, 82u8,
							57u8, 43u8, 95u8, 12u8, 59u8, 241u8, 176u8, 143u8, 131u8, 253u8, 166u8, 98u8, 187u8,
							94u8, 235u8, 177u8, 110u8, 162u8, 218u8,
						],
					)
				}
				#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
				#[doc = ""]
				#[doc = "If the authorization required a version check, this call will ensure the spec name"]
				#[doc = "remains unchanged and that the spec version has increased."]
				#[doc = ""]
				#[doc = "Note that this function will not apply the new `code`, but only attempt to schedule the"]
				#[doc = "upgrade with the Relay Chain."]
				#[doc = ""]
				#[doc = "All origins are allowed."]
				pub fn enact_authorized_upgrade(
					&self,
					code: types::enact_authorized_upgrade::Code,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::EnactAuthorizedUpgrade>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"ParachainSystem",
						"enact_authorized_upgrade",
						types::EnactAuthorizedUpgrade { code },
						[
							232u8, 135u8, 114u8, 87u8, 196u8, 146u8, 244u8, 19u8, 106u8, 73u8, 88u8, 193u8, 48u8,
							14u8, 72u8, 133u8, 247u8, 147u8, 50u8, 95u8, 252u8, 213u8, 192u8, 47u8, 244u8, 102u8,
							195u8, 120u8, 179u8, 87u8, 94u8, 8u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The validation function has been scheduled to apply."]
			pub struct ValidationFunctionStored;
			impl ::subxt::ext::subxt_core::events::StaticEvent for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The validation function was applied as of the contained relay chain block number."]
			pub struct ValidationFunctionApplied {
				pub relay_chain_block_num: validation_function_applied::RelayChainBlockNum,
			}
			pub mod validation_function_applied {
				use super::runtime_types;
				pub type RelayChainBlockNum = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The relay-chain aborted the upgrade process."]
			pub struct ValidationFunctionDiscarded;
			impl ::subxt::ext::subxt_core::events::StaticEvent for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some downward messages have been received and will be processed."]
			pub struct DownwardMessagesReceived {
				pub count: downward_messages_received::Count,
			}
			pub mod downward_messages_received {
				use super::runtime_types;
				pub type Count = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Downward messages were processed using the given weight."]
			pub struct DownwardMessagesProcessed {
				pub weight_used: downward_messages_processed::WeightUsed,
				pub dmq_head: downward_messages_processed::DmqHead,
			}
			pub mod downward_messages_processed {
				use super::runtime_types;
				pub type WeightUsed = runtime_types::sp_weights::weight_v2::Weight;
				pub type DmqHead = ::subxt::ext::subxt_core::utils::H256;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for DownwardMessagesProcessed {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesProcessed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An upward message was sent to the relay chain."]
			pub struct UpwardMessageSent {
				pub message_hash: upward_message_sent::MessageHash,
			}
			pub mod upward_message_sent {
				use super::runtime_types;
				pub type MessageHash = ::core::option::Option<[::core::primitive::u8; 32usize]>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for UpwardMessageSent {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpwardMessageSent";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod unincluded_segment {
					use super::runtime_types;
					pub type UnincludedSegment = ::subxt::ext::subxt_core::alloc::vec::Vec<
						runtime_types::cumulus_pallet_parachain_system::unincluded_segment::Ancestor<
							::subxt::ext::subxt_core::utils::H256,
						>,
					>;
				}
				pub mod aggregated_unincluded_segment {
					use super::runtime_types;
					pub type AggregatedUnincludedSegment =
						runtime_types::cumulus_pallet_parachain_system::unincluded_segment::SegmentTracker<
							::subxt::ext::subxt_core::utils::H256,
						>;
				}
				pub mod pending_validation_code {
					use super::runtime_types;
					pub type PendingValidationCode =
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				pub mod new_validation_code {
					use super::runtime_types;
					pub type NewValidationCode =
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				pub mod validation_data {
					use super::runtime_types;
					pub type ValidationData = runtime_types::polkadot_primitives::v7::PersistedValidationData<
						::subxt::ext::subxt_core::utils::H256,
						::core::primitive::u32,
					>;
				}
				pub mod did_set_validation_code {
					use super::runtime_types;
					pub type DidSetValidationCode = ::core::primitive::bool;
				}
				pub mod last_relay_chain_block_number {
					use super::runtime_types;
					pub type LastRelayChainBlockNumber = ::core::primitive::u32;
				}
				pub mod upgrade_restriction_signal {
					use super::runtime_types;
					pub type UpgradeRestrictionSignal =
						::core::option::Option<runtime_types::polkadot_primitives::v7::UpgradeRestriction>;
				}
				pub mod upgrade_go_ahead {
					use super::runtime_types;
					pub type UpgradeGoAhead =
						::core::option::Option<runtime_types::polkadot_primitives::v7::UpgradeGoAhead>;
				}
				pub mod relay_state_proof {
					use super::runtime_types;
					pub type RelayStateProof = runtime_types::sp_trie::storage_proof::StorageProof;
				}
				pub mod relevant_messaging_state {
					use super::runtime_types;
					pub type RelevantMessagingState = runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot ;
				}
				pub mod host_configuration {
					use super::runtime_types;
					pub type HostConfiguration =
						runtime_types::polkadot_primitives::v7::AbridgedHostConfiguration;
				}
				pub mod last_dmq_mqc_head {
					use super::runtime_types;
					pub type LastDmqMqcHead =
						runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain;
				}
				pub mod last_hrmp_mqc_heads {
					use super::runtime_types;
					pub type LastHrmpMqcHeads = ::subxt::ext::subxt_core::utils::KeyedVec<
						runtime_types::polkadot_parachain_primitives::primitives::Id,
						runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					>;
				}
				pub mod processed_downward_messages {
					use super::runtime_types;
					pub type ProcessedDownwardMessages = ::core::primitive::u32;
				}
				pub mod hrmp_watermark {
					use super::runtime_types;
					pub type HrmpWatermark = ::core::primitive::u32;
				}
				pub mod hrmp_outbound_messages {
					use super::runtime_types;
					pub type HrmpOutboundMessages = ::subxt::ext::subxt_core::alloc::vec::Vec<
						runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
							runtime_types::polkadot_parachain_primitives::primitives::Id,
						>,
					>;
				}
				pub mod upward_messages {
					use super::runtime_types;
					pub type UpwardMessages = ::subxt::ext::subxt_core::alloc::vec::Vec<
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					>;
				}
				pub mod pending_upward_messages {
					use super::runtime_types;
					pub type PendingUpwardMessages = ::subxt::ext::subxt_core::alloc::vec::Vec<
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					>;
				}
				pub mod upward_delivery_fee_factor {
					use super::runtime_types;
					pub type UpwardDeliveryFeeFactor = runtime_types::sp_arithmetic::fixed_point::FixedU128;
				}
				pub mod announced_hrmp_messages_per_candidate {
					use super::runtime_types;
					pub type AnnouncedHrmpMessagesPerCandidate = ::core::primitive::u32;
				}
				pub mod reserved_xcmp_weight_override {
					use super::runtime_types;
					pub type ReservedXcmpWeightOverride = runtime_types::sp_weights::weight_v2::Weight;
				}
				pub mod reserved_dmp_weight_override {
					use super::runtime_types;
					pub type ReservedDmpWeightOverride = runtime_types::sp_weights::weight_v2::Weight;
				}
				pub mod custom_validation_head_data {
					use super::runtime_types;
					pub type CustomValidationHeadData =
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Latest included block descendants the runtime accepted. In other words, these are"]
				#[doc = " ancestors of the currently executing block which have not been included in the observed"]
				#[doc = " relay-chain state."]
				#[doc = ""]
				#[doc = " The segment length is limited by the capacity returned from the [`ConsensusHook`] configured"]
				#[doc = " in the pallet."]
				pub fn unincluded_segment(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::unincluded_segment::UnincludedSegment,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"UnincludedSegment",
						(),
						[
							73u8, 83u8, 226u8, 16u8, 203u8, 233u8, 221u8, 109u8, 23u8, 114u8, 56u8, 154u8, 100u8,
							116u8, 253u8, 10u8, 164u8, 22u8, 110u8, 73u8, 245u8, 226u8, 54u8, 146u8, 67u8, 109u8,
							149u8, 142u8, 154u8, 218u8, 55u8, 178u8,
						],
					)
				}
				#[doc = " Storage field that keeps track of bandwidth used by the unincluded segment along with the"]
				#[doc = " latest HRMP watermark. Used for limiting the acceptance of new blocks with"]
				#[doc = " respect to relay chain constraints."]
				pub fn aggregated_unincluded_segment(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::aggregated_unincluded_segment::AggregatedUnincludedSegment,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"AggregatedUnincludedSegment",
						(),
						[
							165u8, 51u8, 182u8, 156u8, 65u8, 114u8, 167u8, 133u8, 245u8, 52u8, 32u8, 119u8,
							159u8, 65u8, 201u8, 108u8, 99u8, 43u8, 84u8, 63u8, 95u8, 182u8, 134u8, 163u8, 51u8,
							202u8, 243u8, 82u8, 225u8, 192u8, 186u8, 2u8,
						],
					)
				}
				#[doc = " In case of a scheduled upgrade, this storage field contains the validation code to be"]
				#[doc = " applied."]
				#[doc = ""]
				#[doc = " As soon as the relay chain gives us the go-ahead signal, we will overwrite the"]
				#[doc = " [`:code`][sp_core::storage::well_known_keys::CODE] which will result the next block process"]
				#[doc = " with the new validation code. This concludes the upgrade process."]
				pub fn pending_validation_code(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::pending_validation_code::PendingValidationCode,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"PendingValidationCode",
						(),
						[
							78u8, 159u8, 219u8, 211u8, 177u8, 80u8, 102u8, 93u8, 83u8, 146u8, 90u8, 233u8, 232u8,
							11u8, 104u8, 172u8, 93u8, 68u8, 44u8, 228u8, 99u8, 197u8, 254u8, 28u8, 181u8, 215u8,
							247u8, 238u8, 49u8, 49u8, 195u8, 249u8,
						],
					)
				}
				#[doc = " Validation code that is set by the parachain and is to be communicated to collator and"]
				#[doc = " consequently the relay-chain."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block if no other pallet already set"]
				#[doc = " the value."]
				pub fn new_validation_code(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::new_validation_code::NewValidationCode,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"NewValidationCode",
						(),
						[
							185u8, 123u8, 152u8, 122u8, 230u8, 136u8, 79u8, 73u8, 206u8, 19u8, 59u8, 57u8, 75u8,
							250u8, 83u8, 185u8, 29u8, 76u8, 89u8, 137u8, 77u8, 163u8, 25u8, 125u8, 182u8, 67u8,
							2u8, 180u8, 48u8, 237u8, 49u8, 171u8,
						],
					)
				}
				#[doc = " The [`PersistedValidationData`] set for this block."]
				#[doc = " This value is expected to be set only once per block and it's never stored"]
				#[doc = " in the trie."]
				pub fn validation_data(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::validation_data::ValidationData,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"ValidationData",
						(),
						[
							193u8, 240u8, 25u8, 56u8, 103u8, 173u8, 56u8, 56u8, 229u8, 243u8, 91u8, 25u8, 249u8,
							95u8, 122u8, 93u8, 37u8, 181u8, 54u8, 244u8, 217u8, 200u8, 62u8, 136u8, 80u8, 148u8,
							16u8, 177u8, 124u8, 211u8, 95u8, 24u8,
						],
					)
				}
				#[doc = " Were the validation data set to notify the relay chain?"]
				pub fn did_set_validation_code(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::did_set_validation_code::DidSetValidationCode,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"DidSetValidationCode",
						(),
						[
							233u8, 228u8, 48u8, 111u8, 200u8, 35u8, 30u8, 139u8, 251u8, 77u8, 196u8, 252u8, 35u8,
							222u8, 129u8, 235u8, 7u8, 19u8, 156u8, 82u8, 126u8, 173u8, 29u8, 62u8, 20u8, 67u8,
							166u8, 116u8, 108u8, 182u8, 57u8, 246u8,
						],
					)
				}
				#[doc = " The relay chain block number associated with the last parachain block."]
				#[doc = ""]
				#[doc = " This is updated in `on_finalize`."]
				pub fn last_relay_chain_block_number(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::last_relay_chain_block_number::LastRelayChainBlockNumber,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"LastRelayChainBlockNumber",
						(),
						[
							17u8, 65u8, 131u8, 169u8, 195u8, 243u8, 195u8, 93u8, 220u8, 174u8, 75u8, 216u8,
							214u8, 227u8, 96u8, 40u8, 8u8, 153u8, 116u8, 160u8, 79u8, 255u8, 35u8, 232u8, 242u8,
							42u8, 100u8, 150u8, 208u8, 210u8, 142u8, 186u8,
						],
					)
				}
				#[doc = " An option which indicates if the relay-chain restricts signalling a validation code upgrade."]
				#[doc = " In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced"]
				#[doc = " candidate will be invalid."]
				#[doc = ""]
				#[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
				#[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
				#[doc = " set after the inherent."]
				pub fn upgrade_restriction_signal(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::upgrade_restriction_signal::UpgradeRestrictionSignal,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"UpgradeRestrictionSignal",
						(),
						[
							235u8, 240u8, 37u8, 44u8, 181u8, 52u8, 7u8, 216u8, 20u8, 139u8, 69u8, 124u8, 21u8,
							173u8, 237u8, 64u8, 105u8, 88u8, 49u8, 69u8, 123u8, 55u8, 181u8, 167u8, 112u8, 183u8,
							190u8, 231u8, 231u8, 127u8, 77u8, 148u8,
						],
					)
				}
				#[doc = " Optional upgrade go-ahead signal from the relay-chain."]
				#[doc = ""]
				#[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
				#[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
				#[doc = " set after the inherent."]
				pub fn upgrade_go_ahead(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::upgrade_go_ahead::UpgradeGoAhead,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"UpgradeGoAhead",
						(),
						[
							149u8, 144u8, 186u8, 88u8, 180u8, 34u8, 82u8, 226u8, 100u8, 148u8, 246u8, 55u8,
							233u8, 97u8, 43u8, 0u8, 48u8, 31u8, 69u8, 154u8, 29u8, 147u8, 241u8, 91u8, 81u8,
							126u8, 206u8, 117u8, 14u8, 149u8, 87u8, 88u8,
						],
					)
				}
				#[doc = " The state proof for the last relay parent block."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn relay_state_proof(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::relay_state_proof::RelayStateProof,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"RelayStateProof",
						(),
						[
							46u8, 115u8, 163u8, 190u8, 246u8, 47u8, 200u8, 159u8, 206u8, 204u8, 94u8, 250u8,
							127u8, 112u8, 109u8, 111u8, 210u8, 195u8, 244u8, 41u8, 36u8, 187u8, 71u8, 150u8,
							149u8, 253u8, 143u8, 33u8, 83u8, 189u8, 182u8, 238u8,
						],
					)
				}
				#[doc = " The snapshot of some state related to messaging relevant to the current parachain as per"]
				#[doc = " the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn relevant_messaging_state(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::relevant_messaging_state::RelevantMessagingState,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"RelevantMessagingState",
						(),
						[
							117u8, 166u8, 186u8, 126u8, 21u8, 174u8, 86u8, 253u8, 163u8, 90u8, 54u8, 226u8,
							186u8, 253u8, 126u8, 168u8, 145u8, 45u8, 155u8, 32u8, 97u8, 110u8, 208u8, 125u8,
							47u8, 113u8, 165u8, 199u8, 210u8, 118u8, 217u8, 73u8,
						],
					)
				}
				#[doc = " The parachain host configuration that was obtained from the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn host_configuration(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::host_configuration::HostConfiguration,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"HostConfiguration",
						(),
						[
							252u8, 23u8, 111u8, 189u8, 120u8, 204u8, 129u8, 223u8, 248u8, 179u8, 239u8, 173u8,
							133u8, 61u8, 140u8, 2u8, 75u8, 32u8, 204u8, 178u8, 69u8, 21u8, 44u8, 227u8, 178u8,
							179u8, 33u8, 26u8, 131u8, 156u8, 78u8, 85u8,
						],
					)
				}
				#[doc = " The last downward message queue chain head we have observed."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub fn last_dmq_mqc_head(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::last_dmq_mqc_head::LastDmqMqcHead,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"LastDmqMqcHead",
						(),
						[
							1u8, 70u8, 140u8, 40u8, 51u8, 127u8, 75u8, 80u8, 5u8, 49u8, 196u8, 31u8, 30u8, 61u8,
							54u8, 252u8, 0u8, 0u8, 100u8, 115u8, 177u8, 250u8, 138u8, 48u8, 107u8, 41u8, 93u8,
							87u8, 195u8, 107u8, 206u8, 227u8,
						],
					)
				}
				#[doc = " The message queue chain heads we have observed per each channel incoming channel."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub fn last_hrmp_mqc_heads(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::last_hrmp_mqc_heads::LastHrmpMqcHeads,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"LastHrmpMqcHeads",
						(),
						[
							131u8, 170u8, 142u8, 30u8, 101u8, 113u8, 131u8, 81u8, 38u8, 168u8, 98u8, 3u8, 9u8,
							109u8, 96u8, 179u8, 115u8, 177u8, 128u8, 11u8, 238u8, 54u8, 81u8, 60u8, 97u8, 112u8,
							224u8, 175u8, 86u8, 133u8, 182u8, 76u8,
						],
					)
				}
				#[doc = " Number of downward messages processed in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn processed_downward_messages(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::processed_downward_messages::ProcessedDownwardMessages,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"ProcessedDownwardMessages",
						(),
						[
							151u8, 234u8, 196u8, 87u8, 130u8, 79u8, 4u8, 102u8, 47u8, 10u8, 33u8, 132u8, 149u8,
							118u8, 61u8, 141u8, 5u8, 1u8, 30u8, 120u8, 220u8, 156u8, 16u8, 11u8, 14u8, 52u8,
							126u8, 151u8, 244u8, 149u8, 197u8, 51u8,
						],
					)
				}
				#[doc = " HRMP watermark that was set in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn hrmp_watermark(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::hrmp_watermark::HrmpWatermark,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"HrmpWatermark",
						(),
						[
							77u8, 62u8, 59u8, 220u8, 7u8, 125u8, 98u8, 249u8, 108u8, 212u8, 223u8, 99u8, 152u8,
							13u8, 29u8, 80u8, 166u8, 65u8, 232u8, 113u8, 145u8, 128u8, 123u8, 35u8, 238u8, 31u8,
							113u8, 156u8, 220u8, 104u8, 217u8, 165u8,
						],
					)
				}
				#[doc = " HRMP messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn hrmp_outbound_messages(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::hrmp_outbound_messages::HrmpOutboundMessages,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"HrmpOutboundMessages",
						(),
						[
							42u8, 9u8, 96u8, 217u8, 25u8, 101u8, 129u8, 147u8, 150u8, 20u8, 164u8, 186u8, 217u8,
							178u8, 15u8, 201u8, 233u8, 104u8, 92u8, 120u8, 29u8, 245u8, 196u8, 13u8, 141u8,
							210u8, 102u8, 62u8, 216u8, 80u8, 246u8, 145u8,
						],
					)
				}
				#[doc = " Upward messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn upward_messages(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::upward_messages::UpwardMessages,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"UpwardMessages",
						(),
						[
							179u8, 127u8, 8u8, 94u8, 194u8, 246u8, 53u8, 79u8, 80u8, 22u8, 18u8, 75u8, 116u8,
							163u8, 90u8, 161u8, 30u8, 140u8, 57u8, 126u8, 60u8, 91u8, 23u8, 30u8, 120u8, 245u8,
							125u8, 96u8, 152u8, 25u8, 248u8, 85u8,
						],
					)
				}
				#[doc = " Upward messages that are still pending and not yet send to the relay chain."]
				pub fn pending_upward_messages(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::pending_upward_messages::PendingUpwardMessages,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"PendingUpwardMessages",
						(),
						[
							239u8, 45u8, 18u8, 173u8, 148u8, 150u8, 55u8, 176u8, 173u8, 156u8, 246u8, 226u8,
							198u8, 214u8, 104u8, 187u8, 186u8, 13u8, 83u8, 194u8, 153u8, 29u8, 228u8, 109u8,
							26u8, 18u8, 212u8, 151u8, 246u8, 24u8, 133u8, 216u8,
						],
					)
				}
				#[doc = " The factor to multiply the base delivery fee by for UMP."]
				pub fn upward_delivery_fee_factor(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::upward_delivery_fee_factor::UpwardDeliveryFeeFactor,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"UpwardDeliveryFeeFactor",
						(),
						[
							40u8, 217u8, 164u8, 111u8, 151u8, 132u8, 69u8, 226u8, 163u8, 175u8, 43u8, 239u8,
							179u8, 217u8, 136u8, 161u8, 13u8, 251u8, 163u8, 102u8, 24u8, 27u8, 168u8, 89u8,
							221u8, 83u8, 93u8, 64u8, 96u8, 117u8, 146u8, 71u8,
						],
					)
				}
				#[doc = " The number of HRMP messages we observed in `on_initialize` and thus used that number for"]
				#[doc = " announcing the weight of `on_initialize` and `on_finalize`."]
				pub fn announced_hrmp_messages_per_candidate(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::announced_hrmp_messages_per_candidate::AnnouncedHrmpMessagesPerCandidate,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"AnnouncedHrmpMessagesPerCandidate",
						(),
						[
							93u8, 11u8, 229u8, 172u8, 73u8, 87u8, 13u8, 149u8, 15u8, 94u8, 163u8, 107u8, 156u8,
							22u8, 131u8, 177u8, 96u8, 247u8, 213u8, 224u8, 41u8, 126u8, 157u8, 33u8, 154u8,
							194u8, 95u8, 234u8, 65u8, 19u8, 58u8, 161u8,
						],
					)
				}
				#[doc = " The weight we reserve at the beginning of the block for processing XCMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub fn reserved_xcmp_weight_override(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::reserved_xcmp_weight_override::ReservedXcmpWeightOverride,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"ReservedXcmpWeightOverride",
						(),
						[
							176u8, 93u8, 203u8, 74u8, 18u8, 170u8, 246u8, 203u8, 109u8, 89u8, 86u8, 77u8, 96u8,
							66u8, 189u8, 79u8, 184u8, 253u8, 11u8, 230u8, 87u8, 120u8, 1u8, 254u8, 215u8, 41u8,
							210u8, 86u8, 239u8, 206u8, 60u8, 2u8,
						],
					)
				}
				#[doc = " The weight we reserve at the beginning of the block for processing DMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub fn reserved_dmp_weight_override(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::reserved_dmp_weight_override::ReservedDmpWeightOverride,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"ReservedDmpWeightOverride",
						(),
						[
							205u8, 124u8, 9u8, 156u8, 255u8, 207u8, 208u8, 23u8, 179u8, 132u8, 254u8, 157u8,
							237u8, 240u8, 167u8, 203u8, 253u8, 111u8, 136u8, 32u8, 100u8, 152u8, 16u8, 19u8,
							175u8, 14u8, 108u8, 61u8, 59u8, 231u8, 70u8, 112u8,
						],
					)
				}
				#[doc = " A custom head data that should be returned as result of `validate_block`."]
				#[doc = ""]
				#[doc = " See `Pallet::set_custom_validation_head_data` for more information."]
				pub fn custom_validation_head_data(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::custom_validation_head_data::CustomValidationHeadData,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainSystem",
						"CustomValidationHeadData",
						(),
						[
							52u8, 186u8, 187u8, 57u8, 245u8, 171u8, 202u8, 23u8, 92u8, 80u8, 118u8, 66u8, 251u8,
							156u8, 175u8, 254u8, 141u8, 185u8, 115u8, 209u8, 170u8, 165u8, 1u8, 242u8, 120u8,
							234u8, 162u8, 24u8, 135u8, 105u8, 8u8, 177u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Returns the parachain ID we are running with."]
				pub fn self_para_id(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
					runtime_types::polkadot_parachain_primitives::primitives::Id,
				> {
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"ParachainSystem",
						"SelfParaId",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8, 114u8, 121u8,
							147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8, 200u8, 189u8, 156u8, 140u8,
							36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
			}
		}
	}
	pub mod timestamp {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_timestamp::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "[`Config::MinimumPeriod`]."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _None_."]
				#[doc = ""]
				#[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
				#[doc = "that changing the complexity of this call could result exhausting the resources in a"]
				#[doc = "block to execute any other calls."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				pub struct Set {
					#[codec(compact)]
					pub now: set::Now,
				}
				pub mod set {
					use super::runtime_types;
					pub type Now = ::core::primitive::u64;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Set {
					const PALLET: &'static str = "Timestamp";
					const CALL: &'static str = "set";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "[`Config::MinimumPeriod`]."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _None_."]
				#[doc = ""]
				#[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
				#[doc = "that changing the complexity of this call could result exhausting the resources in a"]
				#[doc = "block to execute any other calls."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				pub fn set(
					&self,
					now: types::set::Now,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Set> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Timestamp",
						"set",
						types::Set { now },
						[
							37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8, 199u8, 213u8,
							54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8, 200u8, 4u8, 231u8, 195u8, 173u8,
							6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod now {
					use super::runtime_types;
					pub type Now = ::core::primitive::u64;
				}
				pub mod did_update {
					use super::runtime_types;
					pub type DidUpdate = ::core::primitive::bool;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current time for the current block."]
				pub fn now(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::now::Now,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Timestamp",
						"Now",
						(),
						[
							44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8, 92u8, 61u8,
							39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8, 141u8, 26u8, 12u8, 115u8,
							51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
						],
					)
				}
				#[doc = " Whether the timestamp has been updated in this block."]
				#[doc = ""]
				#[doc = " This value is updated to `true` upon successful submission of a timestamp by a node."]
				#[doc = " It is then checked at the end of each block execution in the `on_finalize` hook."]
				pub fn did_update(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::did_update::DidUpdate,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Timestamp",
						"DidUpdate",
						(),
						[
							229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8, 205u8, 160u8,
							164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8, 248u8, 204u8, 107u8, 46u8, 20u8,
							200u8, 238u8, 167u8, 71u8, 41u8, 214u8, 140u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum period between blocks."]
				#[doc = ""]
				#[doc = " Be aware that this is different to the *expected* period that the block production"]
				#[doc = " apparatus provides. Your chosen consensus system will generally work with this to"]
				#[doc = " determine a sensible block time. For example, in the Aura pallet it will be double this"]
				#[doc = " period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Timestamp",
						"MinimumPeriod",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8, 226u8,
							157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8, 53u8, 85u8, 81u8,
							96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_info {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::staging_parachain_info::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod parachain_id {
					use super::runtime_types;
					pub type ParachainId = runtime_types::polkadot_parachain_primitives::primitives::Id;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn parachain_id(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::parachain_id::ParachainId,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ParachainInfo",
						"ParachainId",
						(),
						[
							160u8, 130u8, 74u8, 181u8, 231u8, 180u8, 246u8, 152u8, 204u8, 44u8, 245u8, 91u8,
							113u8, 246u8, 218u8, 50u8, 254u8, 248u8, 35u8, 219u8, 83u8, 144u8, 228u8, 245u8,
							122u8, 53u8, 194u8, 172u8, 222u8, 118u8, 202u8, 91u8,
						],
					)
				}
			}
		}
	}
	pub mod balances {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_balances::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_balances::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				pub struct TransferAllowDeath {
					pub dest: transfer_allow_death::Dest,
					#[codec(compact)]
					pub value: transfer_allow_death::Value,
				}
				pub mod transfer_allow_death {
					use super::runtime_types;
					pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAllowDeath {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_allow_death";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
				#[doc = "may be specified."]
				pub struct ForceTransfer {
					pub source: force_transfer::Source,
					pub dest: force_transfer::Dest,
					#[codec(compact)]
					pub value: force_transfer::Value,
				}
				pub mod force_transfer {
					use super::runtime_types;
					pub type Source = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
				#[doc = "kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
				#[doc = ""]
				#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
				pub struct TransferKeepAlive {
					pub dest: transfer_keep_alive::Dest,
					#[codec(compact)]
					pub value: transfer_keep_alive::Value,
				}
				pub mod transfer_keep_alive {
					use super::runtime_types;
					pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_keep_alive";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true)."]
				pub struct TransferAll {
					pub dest: transfer_all::Dest,
					pub keep_alive: transfer_all::KeepAlive,
				}
				pub mod transfer_all {
					use super::runtime_types;
					pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type KeepAlive = ::core::primitive::bool;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_all";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub struct ForceUnreserve {
					pub who: force_unreserve::Who,
					pub amount: force_unreserve::Amount,
				}
				pub mod force_unreserve {
					use super::runtime_types;
					pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type Amount = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceUnreserve {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_unreserve";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Upgrade a specified account."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `who`: The account to be upgraded."]
				#[doc = ""]
				#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
				#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
				#[doc = "possibility of churn)."]
				pub struct UpgradeAccounts {
					pub who: upgrade_accounts::Who,
				}
				pub mod upgrade_accounts {
					use super::runtime_types;
					pub type Who =
						::subxt::ext::subxt_core::alloc::vec::Vec<::subxt::ext::subxt_core::utils::AccountId32>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpgradeAccounts {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "upgrade_accounts";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the regular balance of a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub struct ForceSetBalance {
					pub who: force_set_balance::Who,
					#[codec(compact)]
					pub new_free: force_set_balance::NewFree,
				}
				pub mod force_set_balance {
					use super::runtime_types;
					pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type NewFree = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceSetBalance {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_set_balance";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Adjust the total issuance in a saturating way."]
				#[doc = ""]
				#[doc = "Can only be called by root and always needs a positive `delta`."]
				#[doc = ""]
				#[doc = "# Example"]
				pub struct ForceAdjustTotalIssuance {
					pub direction: force_adjust_total_issuance::Direction,
					#[codec(compact)]
					pub delta: force_adjust_total_issuance::Delta,
				}
				pub mod force_adjust_total_issuance {
					use super::runtime_types;
					pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
					pub type Delta = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_adjust_total_issuance";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Burn the specified liquid free balance from the origin account."]
				#[doc = ""]
				#[doc = "If the origin's account ends up below the existential deposit as a result"]
				#[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
				#[doc = ""]
				#[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
				#[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
				pub struct Burn {
					#[codec(compact)]
					pub value: burn::Value,
					pub keep_alive: burn::KeepAlive,
				}
				pub mod burn {
					use super::runtime_types;
					pub type Value = ::core::primitive::u128;
					pub type KeepAlive = ::core::primitive::bool;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Burn {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "burn";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				pub fn transfer_allow_death(
					&self,
					dest: types::transfer_allow_death::Dest,
					value: types::transfer_allow_death::Value,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAllowDeath> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"transfer_allow_death",
						types::TransferAllowDeath { dest, value },
						[
							51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8, 140u8, 27u8,
							205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8, 219u8, 210u8, 147u8, 13u8,
							39u8, 51u8, 21u8, 237u8, 179u8, 132u8, 130u8,
						],
					)
				}
				#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
				#[doc = "may be specified."]
				pub fn force_transfer(
					&self,
					source: types::force_transfer::Source,
					dest: types::force_transfer::Dest,
					value: types::force_transfer::Value,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceTransfer> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"force_transfer",
						types::ForceTransfer {
							source,
							dest,
							value,
						},
						[
							154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8, 153u8, 249u8,
							102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8, 180u8, 90u8, 105u8, 81u8, 217u8,
							60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
						],
					)
				}
				#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
				#[doc = "kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
				#[doc = ""]
				#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
				pub fn transfer_keep_alive(
					&self,
					dest: types::transfer_keep_alive::Dest,
					value: types::transfer_keep_alive::Value,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferKeepAlive> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"transfer_keep_alive",
						types::TransferKeepAlive { dest, value },
						[
							245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8, 55u8, 247u8,
							83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8, 208u8, 207u8, 233u8, 89u8,
							70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
						],
					)
				}
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true)."]
				pub fn transfer_all(
					&self,
					dest: types::transfer_all::Dest,
					keep_alive: types::transfer_all::KeepAlive,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAll> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"transfer_all",
						types::TransferAll { dest, keep_alive },
						[
							105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8, 112u8, 188u8,
							81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8, 9u8, 34u8, 15u8, 67u8,
							34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
						],
					)
				}
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub fn force_unreserve(
					&self,
					who: types::force_unreserve::Who,
					amount: types::force_unreserve::Amount,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceUnreserve> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"force_unreserve",
						types::ForceUnreserve { who, amount },
						[
							142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8, 140u8, 120u8,
							153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8, 199u8, 198u8, 168u8, 208u8,
							159u8, 39u8, 134u8, 92u8, 103u8, 84u8, 171u8,
						],
					)
				}
				#[doc = "Upgrade a specified account."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `who`: The account to be upgraded."]
				#[doc = ""]
				#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
				#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
				#[doc = "possibility of churn)."]
				pub fn upgrade_accounts(
					&self,
					who: types::upgrade_accounts::Who,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpgradeAccounts> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"upgrade_accounts",
						types::UpgradeAccounts { who },
						[
							66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8, 233u8, 255u8,
							124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8, 214u8, 166u8, 217u8, 116u8,
							178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
						],
					)
				}
				#[doc = "Set the regular balance of a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn force_set_balance(
					&self,
					who: types::force_set_balance::Who,
					new_free: types::force_set_balance::NewFree,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceSetBalance> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"force_set_balance",
						types::ForceSetBalance { who, new_free },
						[
							114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8, 39u8, 151u8,
							196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8, 116u8, 93u8, 169u8, 30u8,
							199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
						],
					)
				}
				#[doc = "Adjust the total issuance in a saturating way."]
				#[doc = ""]
				#[doc = "Can only be called by root and always needs a positive `delta`."]
				#[doc = ""]
				#[doc = "# Example"]
				pub fn force_adjust_total_issuance(
					&self,
					direction: types::force_adjust_total_issuance::Direction,
					delta: types::force_adjust_total_issuance::Delta,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceAdjustTotalIssuance>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"force_adjust_total_issuance",
						types::ForceAdjustTotalIssuance { direction, delta },
						[
							208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8, 190u8, 63u8,
							236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8, 58u8, 176u8, 21u8, 78u8, 42u8,
							106u8, 46u8, 248u8, 251u8, 190u8, 150u8, 202u8,
						],
					)
				}
				#[doc = "Burn the specified liquid free balance from the origin account."]
				#[doc = ""]
				#[doc = "If the origin's account ends up below the existential deposit as a result"]
				#[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
				#[doc = ""]
				#[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
				#[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
				pub fn burn(
					&self,
					value: types::burn::Value,
					keep_alive: types::burn::KeepAlive,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Burn> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Balances",
						"burn",
						types::Burn { value, keep_alive },
						[
							176u8, 64u8, 7u8, 109u8, 16u8, 44u8, 145u8, 125u8, 147u8, 152u8, 130u8, 114u8, 221u8,
							201u8, 150u8, 162u8, 118u8, 71u8, 52u8, 92u8, 240u8, 116u8, 203u8, 98u8, 5u8, 22u8,
							43u8, 102u8, 94u8, 208u8, 101u8, 57u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: endowed::Account,
				pub free_balance: endowed::FreeBalance,
			}
			pub mod endowed {
				use super::runtime_types;
				pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type FreeBalance = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: dust_lost::Account,
				pub amount: dust_lost::Amount,
			}
			pub mod dust_lost {
				use super::runtime_types;
				pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: transfer::From,
				pub to: transfer::To,
				pub amount: transfer::Amount,
			}
			pub mod transfer {
				use super::runtime_types;
				pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: balance_set::Who,
				pub free: balance_set::Free,
			}
			pub mod balance_set {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Free = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: reserved::Who,
				pub amount: reserved::Amount,
			}
			pub mod reserved {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: unreserved::Who,
				pub amount: unreserved::Amount,
			}
			pub mod unreserved {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: reserve_repatriated::From,
				pub to: reserve_repatriated::To,
				pub amount: reserve_repatriated::Amount,
				pub destination_status: reserve_repatriated::DestinationStatus,
			}
			pub mod reserve_repatriated {
				use super::runtime_types;
				pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
				pub type DestinationStatus =
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: deposit::Who,
				pub amount: deposit::Amount,
			}
			pub mod deposit {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: withdraw::Who,
				pub amount: withdraw::Amount,
			}
			pub mod withdraw {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: slashed::Who,
				pub amount: slashed::Amount,
			}
			pub mod slashed {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some amount was minted into an account."]
			pub struct Minted {
				pub who: minted::Who,
				pub amount: minted::Amount,
			}
			pub mod minted {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Minted {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Minted";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some amount was burned from an account."]
			pub struct Burned {
				pub who: burned::Who,
				pub amount: burned::Amount,
			}
			pub mod burned {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Burned {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Burned";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some amount was suspended from an account (it can be restored later)."]
			pub struct Suspended {
				pub who: suspended::Who,
				pub amount: suspended::Amount,
			}
			pub mod suspended {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Suspended {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Suspended";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some amount was restored into an account."]
			pub struct Restored {
				pub who: restored::Who,
				pub amount: restored::Amount,
			}
			pub mod restored {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Restored {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Restored";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An account was upgraded."]
			pub struct Upgraded {
				pub who: upgraded::Who,
			}
			pub mod upgraded {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Upgraded {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Upgraded";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
			pub struct Issued {
				pub amount: issued::Amount,
			}
			pub mod issued {
				use super::runtime_types;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Issued {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Issued";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
			pub struct Rescinded {
				pub amount: rescinded::Amount,
			}
			pub mod rescinded {
				use super::runtime_types;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Rescinded {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Rescinded";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some balance was locked."]
			pub struct Locked {
				pub who: locked::Who,
				pub amount: locked::Amount,
			}
			pub mod locked {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Locked {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Locked";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some balance was unlocked."]
			pub struct Unlocked {
				pub who: unlocked::Who,
				pub amount: unlocked::Amount,
			}
			pub mod unlocked {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Unlocked {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unlocked";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some balance was frozen."]
			pub struct Frozen {
				pub who: frozen::Who,
				pub amount: frozen::Amount,
			}
			pub mod frozen {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Frozen {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Frozen";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some balance was thawed."]
			pub struct Thawed {
				pub who: thawed::Who,
				pub amount: thawed::Amount,
			}
			pub mod thawed {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Thawed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Thawed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The `TotalIssuance` was forcefully changed."]
			pub struct TotalIssuanceForced {
				pub old: total_issuance_forced::Old,
				pub new: total_issuance_forced::New,
			}
			pub mod total_issuance_forced {
				use super::runtime_types;
				pub type Old = ::core::primitive::u128;
				pub type New = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for TotalIssuanceForced {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "TotalIssuanceForced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod total_issuance {
					use super::runtime_types;
					pub type TotalIssuance = ::core::primitive::u128;
				}
				pub mod inactive_issuance {
					use super::runtime_types;
					pub type InactiveIssuance = ::core::primitive::u128;
				}
				pub mod account {
					use super::runtime_types;
					pub type Account =
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod locks {
					use super::runtime_types;
					pub type Locks = runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
					>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod reserves {
					use super::runtime_types;
					pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod holds {
					use super::runtime_types;
					pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							runtime_types::cyborg_runtime::RuntimeHoldReason,
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod freezes {
					use super::runtime_types;
					pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							runtime_types::cyborg_runtime::RuntimeFreezeReason,
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::total_issuance::TotalIssuance,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"TotalIssuance",
						(),
						[
							116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8, 171u8, 210u8,
							226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8, 255u8, 19u8, 194u8, 11u8, 27u8,
							194u8, 81u8, 204u8, 59u8, 224u8, 202u8, 185u8,
						],
					)
				}
				#[doc = " The total units of outstanding deactivated balance in the system."]
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::inactive_issuance::InactiveIssuance,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"InactiveIssuance",
						(),
						[
							212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8, 249u8, 77u8,
							247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8, 30u8, 216u8, 16u8, 37u8,
							87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::account::Account,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Account",
						(),
						[
							213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8, 90u8, 3u8,
							88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8, 18u8, 17u8, 234u8, 143u8,
							189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::core::borrow::Borrow<types::account::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::account::Param0>,
					types::account::Account,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Account",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8, 90u8, 3u8,
							88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8, 18u8, 17u8, 234u8, 143u8,
							189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn locks_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::locks::Locks,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Locks",
						(),
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8, 167u8, 18u8,
							132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8, 13u8, 220u8, 163u8, 122u8,
							26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn locks(
					&self,
					_0: impl ::core::borrow::Borrow<types::locks::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::locks::Param0>,
					types::locks::Locks,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Locks",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8, 167u8, 18u8,
							132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8, 13u8, 220u8, 163u8, 122u8,
							26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn reserves_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::reserves::Reserves,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Reserves",
						(),
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8, 140u8, 178u8,
							182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8, 106u8, 193u8, 88u8, 255u8,
							244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn reserves(
					&self,
					_0: impl ::core::borrow::Borrow<types::reserves::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::reserves::Param0>,
					types::reserves::Reserves,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Reserves",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8, 140u8, 178u8,
							182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8, 106u8, 193u8, 88u8, 255u8,
							244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::holds::Holds,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Holds",
						(),
						[
							37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8, 117u8, 119u8,
							114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8, 37u8, 134u8, 22u8, 106u8, 221u8,
							215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds(
					&self,
					_0: impl ::core::borrow::Borrow<types::holds::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::holds::Param0>,
					types::holds::Holds,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Holds",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8, 117u8, 119u8,
							114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8, 37u8, 134u8, 22u8, 106u8, 221u8,
							215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::freezes::Freezes,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Freezes",
						(),
						[
							170u8, 69u8, 116u8, 92u8, 165u8, 14u8, 129u8, 179u8, 165u8, 6u8, 123u8, 156u8, 4u8,
							30u8, 25u8, 181u8, 191u8, 29u8, 3u8, 92u8, 96u8, 167u8, 102u8, 38u8, 128u8, 140u8,
							85u8, 248u8, 114u8, 127u8, 128u8, 40u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes(
					&self,
					_0: impl ::core::borrow::Borrow<types::freezes::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::freezes::Param0>,
					types::freezes::Freezes,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Balances",
						"Freezes",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							170u8, 69u8, 116u8, 92u8, 165u8, 14u8, 129u8, 179u8, 165u8, 6u8, 123u8, 156u8, 4u8,
							30u8, 25u8, 181u8, 191u8, 29u8, 3u8, 92u8, 96u8, 167u8, 102u8, 38u8, 128u8, 140u8,
							85u8, 248u8, 114u8, 127u8, 128u8, 40u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
				#[doc = ""]
				#[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
				#[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
				#[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
				#[doc = " behaviour if you set this to zero."]
				#[doc = ""]
				#[doc = " Bottom line: Do yourself a favour and make it at least one!"]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8, 27u8, 144u8,
							208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8, 71u8, 63u8, 49u8,
							237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn max_locks(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Balances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn max_reserves(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Balances",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
				pub fn max_freezes(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Balances",
						"MaxFreezes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
			#[doc = "has been paid by `who`."]
			pub struct TransactionFeePaid {
				pub who: transaction_fee_paid::Who,
				pub actual_fee: transaction_fee_paid::ActualFee,
				pub tip: transaction_fee_paid::Tip,
			}
			pub mod transaction_fee_paid {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type ActualFee = ::core::primitive::u128;
				pub type Tip = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for TransactionFeePaid {
				const PALLET: &'static str = "TransactionPayment";
				const EVENT: &'static str = "TransactionFeePaid";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod next_fee_multiplier {
					use super::runtime_types;
					pub type NextFeeMultiplier = runtime_types::sp_arithmetic::fixed_point::FixedU128;
				}
				pub mod storage_version {
					use super::runtime_types;
					pub type StorageVersion = runtime_types::pallet_transaction_payment::Releases;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::next_fee_multiplier::NextFeeMultiplier,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TransactionPayment",
						"NextFeeMultiplier",
						(),
						[
							247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8, 147u8, 213u8, 59u8,
							80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8, 159u8, 176u8, 79u8, 249u8, 201u8,
							170u8, 1u8, 129u8, 79u8, 146u8, 197u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::storage_version::StorageVersion,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TransactionPayment",
						"StorageVersion",
						(),
						[
							105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8, 178u8, 126u8,
							31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8, 251u8, 174u8, 15u8, 74u8,
							134u8, 216u8, 244u8, 168u8, 175u8, 158u8, 144u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A fee multiplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
				#[doc = " `priority`"]
				#[doc = ""]
				#[doc = " This value is multiplied by the `final_fee` to obtain a \"virtual tip\" that is later"]
				#[doc = " added to a tip component in regular `priority` calculations."]
				#[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
				#[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
				#[doc = ""]
				#[doc = " ```rust,ignore"]
				#[doc = " // For `Normal`"]
				#[doc = " let priority = priority_calc(tip);"]
				#[doc = ""]
				#[doc = " // For `Operational`"]
				#[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
				#[doc = " let priority = priority_calc(tip + virtual_tip);"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
				#[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
				#[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
				#[doc = " transactions."]
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u8>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8, 28u8, 91u8, 221u8,
							64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8, 114u8, 97u8, 79u8, 62u8, 212u8,
							202u8, 114u8, 237u8, 228u8, 183u8, 165u8,
						],
					)
				}
			}
		}
	}
	pub mod sudo {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the Sudo pallet."]
		pub type Error = runtime_types::pallet_sudo::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_sudo::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				pub struct Sudo {
					pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo::Call>,
				}
				pub mod sudo {
					use super::runtime_types;
					pub type Call = runtime_types::cyborg_runtime::RuntimeCall;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Sudo {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub struct SudoUncheckedWeight {
					pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_unchecked_weight::Call>,
					pub weight: sudo_unchecked_weight::Weight,
				}
				pub mod sudo_unchecked_weight {
					use super::runtime_types;
					pub type Call = runtime_types::cyborg_runtime::RuntimeCall;
					pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoUncheckedWeight {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_unchecked_weight";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				pub struct SetKey {
					pub new: set_key::New,
				}
				pub mod set_key {
					use super::runtime_types;
					pub type New = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "set_key";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub struct SudoAs {
					pub who: sudo_as::Who,
					pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_as::Call>,
				}
				pub mod sudo_as {
					use super::runtime_types;
					pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type Call = runtime_types::cyborg_runtime::RuntimeCall;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoAs {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_as";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Permanently removes the sudo key."]
				#[doc = ""]
				#[doc = "**This cannot be un-done.**"]
				pub struct RemoveKey;
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "remove_key";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				pub fn sudo(
					&self,
					call: types::sudo::Call,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Sudo> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Sudo",
						"sudo",
						types::Sudo {
							call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
						},
						[
							147u8, 41u8, 152u8, 42u8, 14u8, 129u8, 134u8, 2u8, 230u8, 112u8, 62u8, 88u8, 203u8,
							207u8, 122u8, 43u8, 31u8, 141u8, 255u8, 68u8, 222u8, 57u8, 175u8, 169u8, 5u8, 149u8,
							211u8, 236u8, 113u8, 176u8, 72u8, 20u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn sudo_unchecked_weight(
					&self,
					call: types::sudo_unchecked_weight::Call,
					weight: types::sudo_unchecked_weight::Weight,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoUncheckedWeight> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Sudo",
						"sudo_unchecked_weight",
						types::SudoUncheckedWeight {
							call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
							weight,
						},
						[
							177u8, 188u8, 154u8, 237u8, 63u8, 229u8, 231u8, 119u8, 224u8, 218u8, 34u8, 228u8,
							156u8, 242u8, 241u8, 122u8, 237u8, 240u8, 245u8, 33u8, 198u8, 194u8, 25u8, 22u8,
							207u8, 228u8, 252u8, 224u8, 2u8, 239u8, 118u8, 59u8,
						],
					)
				}
				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				pub fn set_key(
					&self,
					new: types::set_key::New,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetKey> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Sudo",
						"set_key",
						types::SetKey { new },
						[
							9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8, 227u8, 197u8,
							44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8, 158u8, 101u8, 192u8, 22u8, 193u8,
							102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn sudo_as(
					&self,
					who: types::sudo_as::Who,
					call: types::sudo_as::Call,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoAs> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Sudo",
						"sudo_as",
						types::SudoAs {
							who,
							call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
						},
						[
							139u8, 174u8, 31u8, 229u8, 106u8, 240u8, 7u8, 50u8, 124u8, 114u8, 57u8, 144u8, 166u8,
							172u8, 18u8, 137u8, 69u8, 9u8, 166u8, 15u8, 137u8, 143u8, 249u8, 18u8, 251u8, 154u8,
							71u8, 80u8, 201u8, 232u8, 74u8, 196u8,
						],
					)
				}
				#[doc = "Permanently removes the sudo key."]
				#[doc = ""]
				#[doc = "**This cannot be un-done.**"]
				pub fn remove_key(
					&self,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveKey> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Sudo",
						"remove_key",
						types::RemoveKey {},
						[
							133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8, 25u8, 28u8, 109u8,
							40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8, 250u8, 191u8, 61u8, 195u8, 172u8, 142u8,
							184u8, 239u8, 247u8, 10u8, 211u8, 79u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A sudo call just took place."]
			pub struct Sudid {
				pub sudo_result: sudid::SudoResult,
			}
			pub mod sudid {
				use super::runtime_types;
				pub type SudoResult = ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The sudo key has been updated."]
			pub struct KeyChanged {
				pub old: key_changed::Old,
				pub new: key_changed::New,
			}
			pub mod key_changed {
				use super::runtime_types;
				pub type Old = ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>;
				pub type New = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The key was permanently removed."]
			pub struct KeyRemoved;
			impl ::subxt::ext::subxt_core::events::StaticEvent for KeyRemoved {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyRemoved";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
			pub struct SudoAsDone {
				pub sudo_result: sudo_as_done::SudoResult,
			}
			pub mod sudo_as_done {
				use super::runtime_types;
				pub type SudoResult = ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for SudoAsDone {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "SudoAsDone";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod key {
					use super::runtime_types;
					pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The `AccountId` of the sudo key."]
				pub fn key(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::key::Key,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Sudo",
						"Key",
						(),
						[
							72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8, 31u8, 84u8,
							137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8, 36u8, 252u8, 151u8, 107u8,
							15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod author {
					use super::runtime_types;
					pub type Author = ::subxt::ext::subxt_core::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Author of current block."]
				pub fn author(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::author::Author,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Authorship",
						"Author",
						(),
						[
							247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8, 220u8, 50u8,
							166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8, 61u8, 192u8, 237u8, 53u8, 22u8,
							148u8, 164u8, 245u8, 99u8, 24u8, 146u8, 18u8,
						],
					)
				}
			}
		}
	}
	pub mod collator_selection {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_collator_selection::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_collator_selection::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the list of invulnerable (fixed) collators. These collators must do some"]
				#[doc = "preparation, namely to have registered session keys."]
				#[doc = ""]
				#[doc = "The call will remove any accounts that have not registered keys from the set. That is,"]
				#[doc = "it is non-atomic; the caller accepts all `AccountId`s passed in `new` _individually_ as"]
				#[doc = "acceptable Invulnerables, and is not proposing a _set_ of new Invulnerables."]
				#[doc = ""]
				#[doc = "This call does not maintain mutual exclusivity of `Invulnerables` and `Candidates`. It"]
				#[doc = "is recommended to use a batch of `add_invulnerable` and `remove_invulnerable` instead. A"]
				#[doc = "`batch_all` can also be used to enforce atomicity. If any candidates are included in"]
				#[doc = "`new`, they should be removed with `remove_invulnerable_candidate` after execution."]
				#[doc = ""]
				#[doc = "Must be called by the `UpdateOrigin`."]
				pub struct SetInvulnerables {
					pub new: set_invulnerables::New,
				}
				pub mod set_invulnerables {
					use super::runtime_types;
					pub type New =
						::subxt::ext::subxt_core::alloc::vec::Vec<::subxt::ext::subxt_core::utils::AccountId32>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetInvulnerables {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_invulnerables";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the ideal number of non-invulnerable collators. If lowering this number, then the"]
				#[doc = "number of running collators could be higher than this figure. Aside from that edge case,"]
				#[doc = "there should be no other way to have more candidates than the desired number."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub struct SetDesiredCandidates {
					pub max: set_desired_candidates::Max,
				}
				pub mod set_desired_candidates {
					use super::runtime_types;
					pub type Max = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetDesiredCandidates {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_desired_candidates";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the candidacy bond amount."]
				#[doc = ""]
				#[doc = "If the candidacy bond is increased by this call, all current candidates which have a"]
				#[doc = "deposit lower than the new bond will be kicked from the list and get their deposits"]
				#[doc = "back."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub struct SetCandidacyBond {
					pub bond: set_candidacy_bond::Bond,
				}
				pub mod set_candidacy_bond {
					use super::runtime_types;
					pub type Bond = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCandidacyBond {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_candidacy_bond";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Register this account as a collator candidate. The account must (a) already have"]
				#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
				#[doc = ""]
				#[doc = "This call is not available to `Invulnerable` collators."]
				pub struct RegisterAsCandidate;
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RegisterAsCandidate {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "register_as_candidate";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
				#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
				#[doc = ""]
				#[doc = "This call will fail if the total number of candidates would drop below"]
				#[doc = "`MinEligibleCollators`."]
				pub struct LeaveIntent;
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for LeaveIntent {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "leave_intent";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Add a new account `who` to the list of `Invulnerables` collators. `who` must have"]
				#[doc = "registered session keys. If `who` is a candidate, they will be removed."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub struct AddInvulnerable {
					pub who: add_invulnerable::Who,
				}
				pub mod add_invulnerable {
					use super::runtime_types;
					pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddInvulnerable {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "add_invulnerable";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Remove an account `who` from the list of `Invulnerables` collators. `Invulnerables` must"]
				#[doc = "be sorted."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub struct RemoveInvulnerable {
					pub who: remove_invulnerable::Who,
				}
				pub mod remove_invulnerable {
					use super::runtime_types;
					pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveInvulnerable {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "remove_invulnerable";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Update the candidacy bond of collator candidate `origin` to a new amount `new_deposit`."]
				#[doc = ""]
				#[doc = "Setting a `new_deposit` that is lower than the current deposit while `origin` is"]
				#[doc = "occupying a top-`DesiredCandidates` slot is not allowed."]
				#[doc = ""]
				#[doc = "This call will fail if `origin` is not a collator candidate, the updated bond is lower"]
				#[doc = "than the minimum candidacy bond, and/or the amount cannot be reserved."]
				pub struct UpdateBond {
					pub new_deposit: update_bond::NewDeposit,
				}
				pub mod update_bond {
					use super::runtime_types;
					pub type NewDeposit = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpdateBond {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "update_bond";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The caller `origin` replaces a candidate `target` in the collator candidate list by"]
				#[doc = "reserving `deposit`. The amount `deposit` reserved by the caller must be greater than"]
				#[doc = "the existing bond of the target it is trying to replace."]
				#[doc = ""]
				#[doc = "This call will fail if the caller is already a collator candidate or invulnerable, the"]
				#[doc = "caller does not have registered session keys, the target is not a collator candidate,"]
				#[doc = "and/or the `deposit` amount cannot be reserved."]
				pub struct TakeCandidateSlot {
					pub deposit: take_candidate_slot::Deposit,
					pub target: take_candidate_slot::Target,
				}
				pub mod take_candidate_slot {
					use super::runtime_types;
					pub type Deposit = ::core::primitive::u128;
					pub type Target = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TakeCandidateSlot {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "take_candidate_slot";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the list of invulnerable (fixed) collators. These collators must do some"]
				#[doc = "preparation, namely to have registered session keys."]
				#[doc = ""]
				#[doc = "The call will remove any accounts that have not registered keys from the set. That is,"]
				#[doc = "it is non-atomic; the caller accepts all `AccountId`s passed in `new` _individually_ as"]
				#[doc = "acceptable Invulnerables, and is not proposing a _set_ of new Invulnerables."]
				#[doc = ""]
				#[doc = "This call does not maintain mutual exclusivity of `Invulnerables` and `Candidates`. It"]
				#[doc = "is recommended to use a batch of `add_invulnerable` and `remove_invulnerable` instead. A"]
				#[doc = "`batch_all` can also be used to enforce atomicity. If any candidates are included in"]
				#[doc = "`new`, they should be removed with `remove_invulnerable_candidate` after execution."]
				#[doc = ""]
				#[doc = "Must be called by the `UpdateOrigin`."]
				pub fn set_invulnerables(
					&self,
					new: types::set_invulnerables::New,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetInvulnerables> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"set_invulnerables",
						types::SetInvulnerables { new },
						[
							113u8, 217u8, 14u8, 48u8, 6u8, 198u8, 8u8, 170u8, 8u8, 237u8, 230u8, 184u8, 17u8,
							181u8, 15u8, 126u8, 117u8, 3u8, 208u8, 215u8, 40u8, 16u8, 150u8, 162u8, 37u8, 196u8,
							235u8, 36u8, 247u8, 24u8, 187u8, 17u8,
						],
					)
				}
				#[doc = "Set the ideal number of non-invulnerable collators. If lowering this number, then the"]
				#[doc = "number of running collators could be higher than this figure. Aside from that edge case,"]
				#[doc = "there should be no other way to have more candidates than the desired number."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub fn set_desired_candidates(
					&self,
					max: types::set_desired_candidates::Max,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetDesiredCandidates> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"set_desired_candidates",
						types::SetDesiredCandidates { max },
						[
							174u8, 44u8, 232u8, 155u8, 228u8, 219u8, 239u8, 75u8, 86u8, 150u8, 135u8, 214u8,
							58u8, 9u8, 25u8, 133u8, 245u8, 101u8, 85u8, 246u8, 15u8, 248u8, 165u8, 87u8, 88u8,
							28u8, 10u8, 196u8, 86u8, 89u8, 28u8, 165u8,
						],
					)
				}
				#[doc = "Set the candidacy bond amount."]
				#[doc = ""]
				#[doc = "If the candidacy bond is increased by this call, all current candidates which have a"]
				#[doc = "deposit lower than the new bond will be kicked from the list and get their deposits"]
				#[doc = "back."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub fn set_candidacy_bond(
					&self,
					bond: types::set_candidacy_bond::Bond,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCandidacyBond> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"set_candidacy_bond",
						types::SetCandidacyBond { bond },
						[
							250u8, 4u8, 185u8, 228u8, 101u8, 223u8, 49u8, 44u8, 172u8, 148u8, 216u8, 242u8,
							192u8, 88u8, 228u8, 59u8, 225u8, 222u8, 171u8, 40u8, 23u8, 1u8, 46u8, 183u8, 189u8,
							191u8, 156u8, 12u8, 218u8, 116u8, 76u8, 59u8,
						],
					)
				}
				#[doc = "Register this account as a collator candidate. The account must (a) already have"]
				#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
				#[doc = ""]
				#[doc = "This call is not available to `Invulnerable` collators."]
				pub fn register_as_candidate(
					&self,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RegisterAsCandidate> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"register_as_candidate",
						types::RegisterAsCandidate {},
						[
							69u8, 222u8, 214u8, 106u8, 105u8, 168u8, 82u8, 239u8, 158u8, 117u8, 224u8, 89u8,
							228u8, 51u8, 221u8, 244u8, 88u8, 63u8, 72u8, 119u8, 224u8, 111u8, 93u8, 39u8, 18u8,
							66u8, 72u8, 105u8, 70u8, 66u8, 178u8, 173u8,
						],
					)
				}
				#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
				#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
				#[doc = ""]
				#[doc = "This call will fail if the total number of candidates would drop below"]
				#[doc = "`MinEligibleCollators`."]
				pub fn leave_intent(
					&self,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::LeaveIntent> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"leave_intent",
						types::LeaveIntent {},
						[
							126u8, 57u8, 10u8, 67u8, 120u8, 229u8, 70u8, 23u8, 154u8, 215u8, 226u8, 178u8, 203u8,
							152u8, 195u8, 177u8, 157u8, 158u8, 40u8, 17u8, 93u8, 225u8, 253u8, 217u8, 48u8,
							165u8, 55u8, 79u8, 43u8, 123u8, 193u8, 147u8,
						],
					)
				}
				#[doc = "Add a new account `who` to the list of `Invulnerables` collators. `who` must have"]
				#[doc = "registered session keys. If `who` is a candidate, they will be removed."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub fn add_invulnerable(
					&self,
					who: types::add_invulnerable::Who,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AddInvulnerable> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"add_invulnerable",
						types::AddInvulnerable { who },
						[
							115u8, 109u8, 38u8, 19u8, 81u8, 194u8, 124u8, 140u8, 239u8, 23u8, 85u8, 62u8, 241u8,
							83u8, 11u8, 241u8, 14u8, 34u8, 206u8, 63u8, 104u8, 78u8, 96u8, 182u8, 173u8, 198u8,
							230u8, 107u8, 102u8, 6u8, 164u8, 75u8,
						],
					)
				}
				#[doc = "Remove an account `who` from the list of `Invulnerables` collators. `Invulnerables` must"]
				#[doc = "be sorted."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub fn remove_invulnerable(
					&self,
					who: types::remove_invulnerable::Who,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveInvulnerable> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"remove_invulnerable",
						types::RemoveInvulnerable { who },
						[
							103u8, 146u8, 23u8, 136u8, 61u8, 65u8, 172u8, 157u8, 216u8, 200u8, 119u8, 28u8,
							189u8, 215u8, 13u8, 100u8, 102u8, 13u8, 94u8, 12u8, 78u8, 156u8, 149u8, 74u8, 126u8,
							118u8, 127u8, 49u8, 129u8, 2u8, 12u8, 118u8,
						],
					)
				}
				#[doc = "Update the candidacy bond of collator candidate `origin` to a new amount `new_deposit`."]
				#[doc = ""]
				#[doc = "Setting a `new_deposit` that is lower than the current deposit while `origin` is"]
				#[doc = "occupying a top-`DesiredCandidates` slot is not allowed."]
				#[doc = ""]
				#[doc = "This call will fail if `origin` is not a collator candidate, the updated bond is lower"]
				#[doc = "than the minimum candidacy bond, and/or the amount cannot be reserved."]
				pub fn update_bond(
					&self,
					new_deposit: types::update_bond::NewDeposit,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpdateBond> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"update_bond",
						types::UpdateBond { new_deposit },
						[
							47u8, 184u8, 193u8, 220u8, 160u8, 1u8, 253u8, 203u8, 8u8, 142u8, 43u8, 151u8, 190u8,
							138u8, 201u8, 174u8, 233u8, 112u8, 200u8, 247u8, 251u8, 94u8, 23u8, 224u8, 150u8,
							179u8, 190u8, 140u8, 199u8, 50u8, 2u8, 249u8,
						],
					)
				}
				#[doc = "The caller `origin` replaces a candidate `target` in the collator candidate list by"]
				#[doc = "reserving `deposit`. The amount `deposit` reserved by the caller must be greater than"]
				#[doc = "the existing bond of the target it is trying to replace."]
				#[doc = ""]
				#[doc = "This call will fail if the caller is already a collator candidate or invulnerable, the"]
				#[doc = "caller does not have registered session keys, the target is not a collator candidate,"]
				#[doc = "and/or the `deposit` amount cannot be reserved."]
				pub fn take_candidate_slot(
					&self,
					deposit: types::take_candidate_slot::Deposit,
					target: types::take_candidate_slot::Target,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TakeCandidateSlot> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"CollatorSelection",
						"take_candidate_slot",
						types::TakeCandidateSlot { deposit, target },
						[
							48u8, 150u8, 189u8, 206u8, 199u8, 196u8, 173u8, 3u8, 206u8, 10u8, 50u8, 160u8, 15u8,
							53u8, 189u8, 126u8, 154u8, 36u8, 90u8, 66u8, 235u8, 12u8, 107u8, 44u8, 117u8, 33u8,
							207u8, 194u8, 251u8, 194u8, 224u8, 80u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "New Invulnerables were set."]
			pub struct NewInvulnerables {
				pub invulnerables: new_invulnerables::Invulnerables,
			}
			pub mod new_invulnerables {
				use super::runtime_types;
				pub type Invulnerables =
					::subxt::ext::subxt_core::alloc::vec::Vec<::subxt::ext::subxt_core::utils::AccountId32>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NewInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewInvulnerables";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A new Invulnerable was added."]
			pub struct InvulnerableAdded {
				pub account_id: invulnerable_added::AccountId,
			}
			pub mod invulnerable_added {
				use super::runtime_types;
				pub type AccountId = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for InvulnerableAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "InvulnerableAdded";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An Invulnerable was removed."]
			pub struct InvulnerableRemoved {
				pub account_id: invulnerable_removed::AccountId,
			}
			pub mod invulnerable_removed {
				use super::runtime_types;
				pub type AccountId = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for InvulnerableRemoved {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "InvulnerableRemoved";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The number of desired candidates was set."]
			pub struct NewDesiredCandidates {
				pub desired_candidates: new_desired_candidates::DesiredCandidates,
			}
			pub mod new_desired_candidates {
				use super::runtime_types;
				pub type DesiredCandidates = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NewDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewDesiredCandidates";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The candidacy bond was set."]
			pub struct NewCandidacyBond {
				pub bond_amount: new_candidacy_bond::BondAmount,
			}
			pub mod new_candidacy_bond {
				use super::runtime_types;
				pub type BondAmount = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A new candidate joined."]
			pub struct CandidateAdded {
				pub account_id: candidate_added::AccountId,
				pub deposit: candidate_added::Deposit,
			}
			pub mod candidate_added {
				use super::runtime_types;
				pub type AccountId = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Deposit = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Bond of a candidate updated."]
			pub struct CandidateBondUpdated {
				pub account_id: candidate_bond_updated::AccountId,
				pub deposit: candidate_bond_updated::Deposit,
			}
			pub mod candidate_bond_updated {
				use super::runtime_types;
				pub type AccountId = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Deposit = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for CandidateBondUpdated {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateBondUpdated";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A candidate was removed."]
			pub struct CandidateRemoved {
				pub account_id: candidate_removed::AccountId,
			}
			pub mod candidate_removed {
				use super::runtime_types;
				pub type AccountId = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for CandidateRemoved {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateRemoved";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An account was replaced in the candidate list by another one."]
			pub struct CandidateReplaced {
				pub old: candidate_replaced::Old,
				pub new: candidate_replaced::New,
				pub deposit: candidate_replaced::Deposit,
			}
			pub mod candidate_replaced {
				use super::runtime_types;
				pub type Old = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type New = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Deposit = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for CandidateReplaced {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateReplaced";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An account was unable to be added to the Invulnerables because they did not have keys"]
			#[doc = "registered. Other Invulnerables may have been set."]
			pub struct InvalidInvulnerableSkipped {
				pub account_id: invalid_invulnerable_skipped::AccountId,
			}
			pub mod invalid_invulnerable_skipped {
				use super::runtime_types;
				pub type AccountId = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for InvalidInvulnerableSkipped {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "InvalidInvulnerableSkipped";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod invulnerables {
					use super::runtime_types;
					pub type Invulnerables = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::subxt::ext::subxt_core::utils::AccountId32,
					>;
				}
				pub mod candidate_list {
					use super::runtime_types;
					pub type CandidateList = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_collator_selection::pallet::CandidateInfo<
							::subxt::ext::subxt_core::utils::AccountId32,
							::core::primitive::u128,
						>,
					>;
				}
				pub mod last_authored_block {
					use super::runtime_types;
					pub type LastAuthoredBlock = ::core::primitive::u32;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod desired_candidates {
					use super::runtime_types;
					pub type DesiredCandidates = ::core::primitive::u32;
				}
				pub mod candidacy_bond {
					use super::runtime_types;
					pub type CandidacyBond = ::core::primitive::u128;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The invulnerable, permissioned collators. This list must be sorted."]
				pub fn invulnerables(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::invulnerables::Invulnerables,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"CollatorSelection",
						"Invulnerables",
						(),
						[
							109u8, 180u8, 25u8, 41u8, 152u8, 158u8, 186u8, 214u8, 89u8, 222u8, 103u8, 14u8, 91u8,
							3u8, 65u8, 6u8, 255u8, 62u8, 47u8, 255u8, 132u8, 164u8, 217u8, 200u8, 130u8, 29u8,
							168u8, 23u8, 81u8, 217u8, 35u8, 123u8,
						],
					)
				}
				#[doc = " The (community, limited) collation candidates. `Candidates` and `Invulnerables` should be"]
				#[doc = " mutually exclusive."]
				#[doc = ""]
				#[doc = " This list is sorted in ascending order by deposit and when the deposits are equal, the least"]
				#[doc = " recently updated is considered greater."]
				pub fn candidate_list(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::candidate_list::CandidateList,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"CollatorSelection",
						"CandidateList",
						(),
						[
							77u8, 195u8, 89u8, 139u8, 79u8, 111u8, 151u8, 215u8, 19u8, 152u8, 67u8, 49u8, 74u8,
							76u8, 3u8, 60u8, 51u8, 140u8, 6u8, 134u8, 159u8, 55u8, 196u8, 57u8, 189u8, 31u8,
							219u8, 218u8, 164u8, 189u8, 196u8, 60u8,
						],
					)
				}
				#[doc = " Last block authored by collator."]
				pub fn last_authored_block_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::last_authored_block::LastAuthoredBlock,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"CollatorSelection",
						"LastAuthoredBlock",
						(),
						[
							176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8, 72u8, 241u8,
							144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8, 126u8, 32u8, 214u8, 26u8,
							171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
						],
					)
				}
				#[doc = " Last block authored by collator."]
				pub fn last_authored_block(
					&self,
					_0: impl ::core::borrow::Borrow<types::last_authored_block::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::last_authored_block::Param0,
					>,
					types::last_authored_block::LastAuthoredBlock,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"CollatorSelection",
						"LastAuthoredBlock",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8, 72u8, 241u8,
							144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8, 126u8, 32u8, 214u8, 26u8,
							171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
						],
					)
				}
				#[doc = " Desired number of candidates."]
				#[doc = ""]
				#[doc = " This should ideally always be less than [`Config::MaxCandidates`] for weights to be correct."]
				pub fn desired_candidates(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::desired_candidates::DesiredCandidates,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"CollatorSelection",
						"DesiredCandidates",
						(),
						[
							69u8, 199u8, 130u8, 132u8, 10u8, 127u8, 204u8, 220u8, 59u8, 107u8, 96u8, 180u8, 42u8,
							235u8, 14u8, 126u8, 231u8, 242u8, 162u8, 126u8, 63u8, 223u8, 15u8, 250u8, 22u8,
							210u8, 54u8, 34u8, 235u8, 191u8, 250u8, 21u8,
						],
					)
				}
				#[doc = " Fixed amount to deposit to become a collator."]
				#[doc = ""]
				#[doc = " When a collator calls `leave_intent` they immediately receive the deposit back."]
				pub fn candidacy_bond(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::candidacy_bond::CandidacyBond,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"CollatorSelection",
						"CandidacyBond",
						(),
						[
							71u8, 134u8, 156u8, 102u8, 201u8, 83u8, 240u8, 251u8, 189u8, 213u8, 211u8, 182u8,
							126u8, 122u8, 41u8, 174u8, 105u8, 29u8, 216u8, 23u8, 255u8, 55u8, 245u8, 187u8,
							234u8, 234u8, 178u8, 155u8, 145u8, 49u8, 196u8, 214u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the session pallet."]
		pub type Error = runtime_types::pallet_session::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_session::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be signed."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
				#[doc = "  fixed."]
				pub struct SetKeys {
					pub keys: set_keys::Keys,
					pub proof: set_keys::Proof,
				}
				pub mod set_keys {
					use super::runtime_types;
					pub type Keys = runtime_types::cyborg_runtime::SessionKeys;
					pub type Proof = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "set_keys";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
				#[doc = "  `T::Keys::key_ids()` which is fixed."]
				pub struct PurgeKeys;
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for PurgeKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "purge_keys";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be signed."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
				#[doc = "  fixed."]
				pub fn set_keys(
					&self,
					keys: types::set_keys::Keys,
					proof: types::set_keys::Proof,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetKeys> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Session",
						"set_keys",
						types::SetKeys { keys, proof },
						[
							219u8, 63u8, 235u8, 242u8, 176u8, 248u8, 204u8, 20u8, 121u8, 176u8, 105u8, 242u8,
							190u8, 124u8, 153u8, 219u8, 12u8, 224u8, 196u8, 18u8, 183u8, 159u8, 33u8, 97u8, 44u8,
							64u8, 0u8, 10u8, 52u8, 181u8, 70u8, 206u8,
						],
					)
				}
				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
				#[doc = "  `T::Keys::key_ids()` which is fixed."]
				pub fn purge_keys(
					&self,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::PurgeKeys> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Session",
						"purge_keys",
						types::PurgeKeys {},
						[
							215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8, 151u8, 158u8,
							31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8, 67u8, 217u8, 39u8, 241u8,
							245u8, 203u8, 240u8, 203u8, 172u8, 16u8, 209u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
			pub struct NewSession {
				pub session_index: new_session::SessionIndex,
			}
			pub mod new_session {
				use super::runtime_types;
				pub type SessionIndex = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod validators {
					use super::runtime_types;
					pub type Validators =
						::subxt::ext::subxt_core::alloc::vec::Vec<::subxt::ext::subxt_core::utils::AccountId32>;
				}
				pub mod current_index {
					use super::runtime_types;
					pub type CurrentIndex = ::core::primitive::u32;
				}
				pub mod queued_changed {
					use super::runtime_types;
					pub type QueuedChanged = ::core::primitive::bool;
				}
				pub mod queued_keys {
					use super::runtime_types;
					pub type QueuedKeys = ::subxt::ext::subxt_core::alloc::vec::Vec<(
						::subxt::ext::subxt_core::utils::AccountId32,
						runtime_types::cyborg_runtime::SessionKeys,
					)>;
				}
				pub mod disabled_validators {
					use super::runtime_types;
					pub type DisabledValidators =
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u32>;
				}
				pub mod next_keys {
					use super::runtime_types;
					pub type NextKeys = runtime_types::cyborg_runtime::SessionKeys;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod key_owner {
					use super::runtime_types;
					pub type KeyOwner = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param0 = runtime_types::sp_core::crypto::KeyTypeId;
					pub type Param1 = [::core::primitive::u8];
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current set of validators."]
				pub fn validators(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::validators::Validators,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"Validators",
						(),
						[
							50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8, 133u8, 194u8,
							210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8, 115u8, 12u8, 12u8, 141u8, 4u8,
							178u8, 201u8, 241u8, 223u8, 234u8, 6u8, 86u8,
						],
					)
				}
				#[doc = " Current index of the session."]
				pub fn current_index(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::current_index::CurrentIndex,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"CurrentIndex",
						(),
						[
							167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8, 135u8, 65u8,
							187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8, 134u8, 159u8, 232u8, 224u8,
							243u8, 27u8, 31u8, 166u8, 145u8, 44u8, 221u8, 230u8,
						],
					)
				}
				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub fn queued_changed(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::queued_changed::QueuedChanged,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"QueuedChanged",
						(),
						[
							184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8, 198u8, 227u8,
							140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8, 36u8, 226u8, 64u8, 113u8,
							141u8, 199u8, 111u8, 99u8, 144u8, 198u8, 153u8,
						],
					)
				}
				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub fn queued_keys(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::queued_keys::QueuedKeys,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"QueuedKeys",
						(),
						[
							205u8, 110u8, 116u8, 201u8, 29u8, 220u8, 3u8, 147u8, 3u8, 236u8, 73u8, 108u8, 108u8,
							173u8, 76u8, 44u8, 102u8, 69u8, 47u8, 90u8, 185u8, 162u8, 57u8, 23u8, 210u8, 45u8,
							18u8, 242u8, 10u8, 95u8, 67u8, 109u8,
						],
					)
				}
				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub fn disabled_validators(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::disabled_validators::DisabledValidators,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"DisabledValidators",
						(),
						[
							213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8, 36u8, 233u8,
							158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8, 183u8, 46u8, 68u8, 154u8,
							240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::next_keys::NextKeys,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"NextKeys",
						(),
						[
							45u8, 92u8, 45u8, 21u8, 150u8, 181u8, 197u8, 56u8, 229u8, 146u8, 183u8, 210u8, 56u8,
							197u8, 9u8, 202u8, 226u8, 183u8, 110u8, 173u8, 100u8, 75u8, 248u8, 207u8, 215u8,
							163u8, 13u8, 113u8, 222u8, 128u8, 18u8, 192u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys(
					&self,
					_0: impl ::core::borrow::Borrow<types::next_keys::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::next_keys::Param0>,
					types::next_keys::NextKeys,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"NextKeys",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							45u8, 92u8, 45u8, 21u8, 150u8, 181u8, 197u8, 56u8, 229u8, 146u8, 183u8, 210u8, 56u8,
							197u8, 9u8, 202u8, 226u8, 183u8, 110u8, 173u8, 100u8, 75u8, 248u8, 207u8, 215u8,
							163u8, 13u8, 113u8, 222u8, 128u8, 18u8, 192u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::key_owner::KeyOwner,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"KeyOwner",
						(),
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8, 253u8, 253u8,
							248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8, 253u8, 109u8, 88u8, 77u8, 217u8,
							140u8, 51u8, 40u8, 118u8, 35u8, 107u8, 206u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::key_owner::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::key_owner::Param0>,
					types::key_owner::KeyOwner,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"KeyOwner",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8, 253u8, 253u8,
							248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8, 253u8, 109u8, 88u8, 77u8, 217u8,
							140u8, 51u8, 40u8, 118u8, 35u8, 107u8, 206u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner(
					&self,
					_0: impl ::core::borrow::Borrow<types::key_owner::Param0>,
					_1: impl ::core::borrow::Borrow<types::key_owner::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::key_owner::Param0>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::key_owner::Param1>,
					),
					types::key_owner::KeyOwner,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Session",
						"KeyOwner",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8, 253u8, 253u8,
							248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8, 253u8, 109u8, 88u8, 77u8, 217u8,
							140u8, 51u8, 40u8, 118u8, 35u8, 107u8, 206u8,
						],
					)
				}
			}
		}
	}
	pub mod aura {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod authorities {
					use super::runtime_types;
					pub type Authorities = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>;
				}
				pub mod current_slot {
					use super::runtime_types;
					pub type CurrentSlot = runtime_types::sp_consensus_slots::Slot;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current authority set."]
				pub fn authorities(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::authorities::Authorities,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Aura",
						"Authorities",
						(),
						[
							95u8, 52u8, 203u8, 53u8, 254u8, 107u8, 134u8, 122u8, 95u8, 253u8, 51u8, 137u8, 142u8,
							106u8, 237u8, 248u8, 159u8, 80u8, 41u8, 233u8, 137u8, 133u8, 13u8, 217u8, 176u8,
							88u8, 132u8, 199u8, 241u8, 47u8, 125u8, 27u8,
						],
					)
				}
				#[doc = " The current slot of this block."]
				#[doc = ""]
				#[doc = " This will be set in `on_initialize`."]
				pub fn current_slot(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::current_slot::CurrentSlot,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Aura",
						"CurrentSlot",
						(),
						[
							112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8, 236u8, 167u8,
							219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8, 201u8, 169u8, 211u8, 133u8,
							135u8, 213u8, 150u8, 105u8, 60u8, 252u8, 43u8, 57u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The slot duration Aura should run with, expressed in milliseconds."]
				#[doc = " The effective value of this type should not change while the chain is running."]
				#[doc = ""]
				#[doc = " For backwards compatibility either use [`MinimumPeriodTimesTwo`] or a const."]
				pub fn slot_duration(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Aura",
						"SlotDuration",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8, 226u8,
							157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8, 53u8, 85u8, 81u8,
							96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8,
						],
					)
				}
			}
		}
	}
	pub mod aura_ext {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod authorities {
					use super::runtime_types;
					pub type Authorities = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>;
				}
				pub mod slot_info {
					use super::runtime_types;
					pub type SlotInfo = (
						runtime_types::sp_consensus_slots::Slot,
						::core::primitive::u32,
					);
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Serves as cache for the authorities."]
				#[doc = ""]
				#[doc = " The authorities in AuRa are overwritten in `on_initialize` when we switch to a new session,"]
				#[doc = " but we require the old authorities to verify the seal when validating a PoV. This will"]
				#[doc = " always be updated to the latest AuRa authorities in `on_finalize`."]
				pub fn authorities(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::authorities::Authorities,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"AuraExt",
						"Authorities",
						(),
						[
							95u8, 52u8, 203u8, 53u8, 254u8, 107u8, 134u8, 122u8, 95u8, 253u8, 51u8, 137u8, 142u8,
							106u8, 237u8, 248u8, 159u8, 80u8, 41u8, 233u8, 137u8, 133u8, 13u8, 217u8, 176u8,
							88u8, 132u8, 199u8, 241u8, 47u8, 125u8, 27u8,
						],
					)
				}
				#[doc = " Current slot paired with a number of authored blocks."]
				#[doc = ""]
				#[doc = " Updated on each block initialization."]
				pub fn slot_info(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::slot_info::SlotInfo,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"AuraExt",
						"SlotInfo",
						(),
						[
							135u8, 135u8, 71u8, 123u8, 102u8, 223u8, 215u8, 76u8, 183u8, 169u8, 108u8, 60u8,
							122u8, 5u8, 24u8, 201u8, 96u8, 59u8, 132u8, 95u8, 253u8, 100u8, 148u8, 184u8, 133u8,
							146u8, 101u8, 201u8, 91u8, 30u8, 76u8, 169u8,
						],
					)
				}
			}
		}
	}
	pub mod xcmp_queue {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::cumulus_pallet_xcmp_queue::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::cumulus_pallet_xcmp_queue::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub struct SuspendXcmExecution;
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SuspendXcmExecution {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "suspend_xcm_execution";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Resumes all XCM executions for the XCMP queue."]
				#[doc = ""]
				#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub struct ResumeXcmExecution;
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ResumeXcmExecution {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "resume_xcm_execution";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Overwrites the number of pages which must be in the queue for the other side to be"]
				#[doc = "told to suspend their sending."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
				pub struct UpdateSuspendThreshold {
					pub new: update_suspend_threshold::New,
				}
				pub mod update_suspend_threshold {
					use super::runtime_types;
					pub type New = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpdateSuspendThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_suspend_threshold";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Overwrites the number of pages which must be in the queue after which we drop any"]
				#[doc = "further messages from the channel."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
				pub struct UpdateDropThreshold {
					pub new: update_drop_threshold::New,
				}
				pub mod update_drop_threshold {
					use super::runtime_types;
					pub type New = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpdateDropThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_drop_threshold";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Overwrites the number of pages which the queue must be reduced to before it signals"]
				#[doc = "that message sending may recommence after it has been suspended."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
				pub struct UpdateResumeThreshold {
					pub new: update_resume_threshold::New,
				}
				pub mod update_resume_threshold {
					use super::runtime_types;
					pub type New = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpdateResumeThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_resume_threshold";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub fn suspend_xcm_execution(
					&self,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SuspendXcmExecution> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"XcmpQueue",
						"suspend_xcm_execution",
						types::SuspendXcmExecution {},
						[
							54u8, 120u8, 33u8, 251u8, 74u8, 56u8, 29u8, 76u8, 104u8, 218u8, 115u8, 198u8, 148u8,
							237u8, 9u8, 191u8, 241u8, 48u8, 33u8, 24u8, 60u8, 144u8, 22u8, 78u8, 58u8, 50u8,
							26u8, 188u8, 231u8, 42u8, 201u8, 76u8,
						],
					)
				}
				#[doc = "Resumes all XCM executions for the XCMP queue."]
				#[doc = ""]
				#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub fn resume_xcm_execution(
					&self,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ResumeXcmExecution> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"XcmpQueue",
						"resume_xcm_execution",
						types::ResumeXcmExecution {},
						[
							173u8, 231u8, 78u8, 253u8, 108u8, 234u8, 199u8, 124u8, 184u8, 154u8, 95u8, 194u8,
							13u8, 77u8, 175u8, 7u8, 7u8, 112u8, 161u8, 72u8, 133u8, 71u8, 63u8, 218u8, 97u8,
							226u8, 133u8, 6u8, 93u8, 177u8, 247u8, 109u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages which must be in the queue for the other side to be"]
				#[doc = "told to suspend their sending."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
				pub fn update_suspend_threshold(
					&self,
					new: types::update_suspend_threshold::New,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpdateSuspendThreshold>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"XcmpQueue",
						"update_suspend_threshold",
						types::UpdateSuspendThreshold { new },
						[
							64u8, 91u8, 172u8, 51u8, 220u8, 174u8, 54u8, 47u8, 57u8, 89u8, 75u8, 39u8, 126u8,
							198u8, 143u8, 35u8, 70u8, 125u8, 167u8, 14u8, 17u8, 18u8, 146u8, 222u8, 100u8, 92u8,
							81u8, 239u8, 173u8, 43u8, 42u8, 174u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages which must be in the queue after which we drop any"]
				#[doc = "further messages from the channel."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
				pub fn update_drop_threshold(
					&self,
					new: types::update_drop_threshold::New,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpdateDropThreshold> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"XcmpQueue",
						"update_drop_threshold",
						types::UpdateDropThreshold { new },
						[
							123u8, 54u8, 12u8, 180u8, 165u8, 198u8, 141u8, 200u8, 149u8, 168u8, 186u8, 237u8,
							162u8, 91u8, 89u8, 242u8, 229u8, 16u8, 32u8, 254u8, 59u8, 168u8, 31u8, 134u8, 217u8,
							251u8, 0u8, 102u8, 113u8, 194u8, 175u8, 9u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages which the queue must be reduced to before it signals"]
				#[doc = "that message sending may recommence after it has been suspended."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
				pub fn update_resume_threshold(
					&self,
					new: types::update_resume_threshold::New,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpdateResumeThreshold>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"XcmpQueue",
						"update_resume_threshold",
						types::UpdateResumeThreshold { new },
						[
							172u8, 136u8, 11u8, 106u8, 42u8, 157u8, 167u8, 183u8, 87u8, 62u8, 182u8, 17u8, 184u8,
							59u8, 215u8, 230u8, 18u8, 243u8, 212u8, 34u8, 54u8, 188u8, 95u8, 119u8, 173u8, 20u8,
							91u8, 206u8, 212u8, 57u8, 136u8, 77u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An HRMP message was sent to a sibling parachain."]
			pub struct XcmpMessageSent {
				pub message_hash: xcmp_message_sent::MessageHash,
			}
			pub mod xcmp_message_sent {
				use super::runtime_types;
				pub type MessageHash = [::core::primitive::u8; 32usize];
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for XcmpMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "XcmpMessageSent";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod inbound_xcmp_suspended {
					use super::runtime_types;
					pub type InboundXcmpSuspended =
						runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
							runtime_types::polkadot_parachain_primitives::primitives::Id,
						>;
				}
				pub mod outbound_xcmp_status {
					use super::runtime_types;
					pub type OutboundXcmpStatus = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
					>;
				}
				pub mod outbound_xcmp_messages {
					use super::runtime_types;
					pub type OutboundXcmpMessages =
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>;
					pub type Param0 = runtime_types::polkadot_parachain_primitives::primitives::Id;
					pub type Param1 = ::core::primitive::u16;
				}
				pub mod signal_messages {
					use super::runtime_types;
					pub type SignalMessages =
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>;
					pub type Param0 = runtime_types::polkadot_parachain_primitives::primitives::Id;
				}
				pub mod queue_config {
					use super::runtime_types;
					pub type QueueConfig = runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData;
				}
				pub mod queue_suspended {
					use super::runtime_types;
					pub type QueueSuspended = ::core::primitive::bool;
				}
				pub mod delivery_fee_factor {
					use super::runtime_types;
					pub type DeliveryFeeFactor = runtime_types::sp_arithmetic::fixed_point::FixedU128;
					pub type Param0 = runtime_types::polkadot_parachain_primitives::primitives::Id;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The suspended inbound XCMP channels. All others are not suspended."]
				#[doc = ""]
				#[doc = " This is a `StorageValue` instead of a `StorageMap` since we expect multiple reads per block"]
				#[doc = " to different keys with a one byte payload. The access to `BoundedBTreeSet` will be cached"]
				#[doc = " within the block and therefore only included once in the proof size."]
				#[doc = ""]
				#[doc = " NOTE: The PoV benchmarking cannot know this and will over-estimate, but the actual proof"]
				#[doc = " will be smaller."]
				pub fn inbound_xcmp_suspended(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::inbound_xcmp_suspended::InboundXcmpSuspended,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"InboundXcmpSuspended",
						(),
						[
							110u8, 23u8, 239u8, 104u8, 136u8, 224u8, 179u8, 180u8, 40u8, 159u8, 54u8, 15u8, 55u8,
							111u8, 75u8, 147u8, 131u8, 127u8, 9u8, 57u8, 133u8, 70u8, 175u8, 181u8, 232u8, 49u8,
							13u8, 19u8, 59u8, 151u8, 179u8, 215u8,
						],
					)
				}
				#[doc = " The non-empty XCMP channels in order of becoming non-empty, and the index of the first"]
				#[doc = " and last outbound message. If the two indices are equal, then it indicates an empty"]
				#[doc = " queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater"]
				#[doc = " than 65535 items. Queue indices for normal messages begin at one; zero is reserved in"]
				#[doc = " case of the need to send a high-priority signal message this block."]
				#[doc = " The bool is true if there is a signal message waiting to be sent."]
				pub fn outbound_xcmp_status(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::outbound_xcmp_status::OutboundXcmpStatus,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"OutboundXcmpStatus",
						(),
						[
							236u8, 81u8, 241u8, 94u8, 247u8, 213u8, 123u8, 240u8, 144u8, 27u8, 39u8, 73u8, 147u8,
							85u8, 18u8, 2u8, 249u8, 25u8, 132u8, 158u8, 118u8, 84u8, 2u8, 226u8, 174u8, 94u8,
							25u8, 117u8, 86u8, 121u8, 214u8, 32u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::outbound_xcmp_messages::OutboundXcmpMessages,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						(),
						[
							163u8, 69u8, 82u8, 238u8, 52u8, 57u8, 181u8, 23u8, 138u8, 75u8, 43u8, 208u8, 209u8,
							195u8, 180u8, 199u8, 174u8, 101u8, 28u8, 248u8, 76u8, 190u8, 140u8, 116u8, 251u8,
							123u8, 160u8, 119u8, 204u8, 91u8, 59u8, 234u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::outbound_xcmp_messages::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::outbound_xcmp_messages::Param0,
					>,
					types::outbound_xcmp_messages::OutboundXcmpMessages,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							163u8, 69u8, 82u8, 238u8, 52u8, 57u8, 181u8, 23u8, 138u8, 75u8, 43u8, 208u8, 209u8,
							195u8, 180u8, 199u8, 174u8, 101u8, 28u8, 248u8, 76u8, 190u8, 140u8, 116u8, 251u8,
							123u8, 160u8, 119u8, 204u8, 91u8, 59u8, 234u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages(
					&self,
					_0: impl ::core::borrow::Borrow<types::outbound_xcmp_messages::Param0>,
					_1: impl ::core::borrow::Borrow<types::outbound_xcmp_messages::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::outbound_xcmp_messages::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::outbound_xcmp_messages::Param1,
						>,
					),
					types::outbound_xcmp_messages::OutboundXcmpMessages,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							163u8, 69u8, 82u8, 238u8, 52u8, 57u8, 181u8, 23u8, 138u8, 75u8, 43u8, 208u8, 209u8,
							195u8, 180u8, 199u8, 174u8, 101u8, 28u8, 248u8, 76u8, 190u8, 140u8, 116u8, 251u8,
							123u8, 160u8, 119u8, 204u8, 91u8, 59u8, 234u8,
						],
					)
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub fn signal_messages_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::signal_messages::SignalMessages,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"SignalMessages",
						(),
						[
							35u8, 133u8, 54u8, 149u8, 97u8, 64u8, 30u8, 174u8, 154u8, 60u8, 119u8, 92u8, 207u8,
							67u8, 151u8, 242u8, 6u8, 128u8, 60u8, 204u8, 15u8, 135u8, 36u8, 234u8, 29u8, 122u8,
							220u8, 28u8, 243u8, 152u8, 217u8, 61u8,
						],
					)
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub fn signal_messages(
					&self,
					_0: impl ::core::borrow::Borrow<types::signal_messages::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::signal_messages::Param0,
					>,
					types::signal_messages::SignalMessages,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"SignalMessages",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							35u8, 133u8, 54u8, 149u8, 97u8, 64u8, 30u8, 174u8, 154u8, 60u8, 119u8, 92u8, 207u8,
							67u8, 151u8, 242u8, 6u8, 128u8, 60u8, 204u8, 15u8, 135u8, 36u8, 234u8, 29u8, 122u8,
							220u8, 28u8, 243u8, 152u8, 217u8, 61u8,
						],
					)
				}
				#[doc = " The configuration which controls the dynamics of the outbound queue."]
				pub fn queue_config(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::queue_config::QueueConfig,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"QueueConfig",
						(),
						[
							185u8, 67u8, 247u8, 243u8, 211u8, 232u8, 57u8, 240u8, 237u8, 181u8, 23u8, 114u8,
							215u8, 128u8, 193u8, 1u8, 176u8, 53u8, 110u8, 195u8, 148u8, 80u8, 187u8, 143u8, 62u8,
							30u8, 143u8, 34u8, 248u8, 109u8, 3u8, 141u8,
						],
					)
				}
				#[doc = " Whether or not the XCMP queue is suspended from executing incoming XCMs or not."]
				pub fn queue_suspended(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::queue_suspended::QueueSuspended,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"QueueSuspended",
						(),
						[
							165u8, 66u8, 105u8, 244u8, 113u8, 43u8, 177u8, 252u8, 212u8, 243u8, 143u8, 184u8,
							87u8, 51u8, 163u8, 104u8, 29u8, 84u8, 119u8, 74u8, 233u8, 129u8, 203u8, 105u8, 2u8,
							101u8, 19u8, 170u8, 69u8, 253u8, 80u8, 132u8,
						],
					)
				}
				#[doc = " The factor to multiply the base delivery fee by."]
				pub fn delivery_fee_factor_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::delivery_fee_factor::DeliveryFeeFactor,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"DeliveryFeeFactor",
						(),
						[
							43u8, 5u8, 63u8, 235u8, 115u8, 155u8, 130u8, 27u8, 75u8, 216u8, 177u8, 135u8, 203u8,
							147u8, 167u8, 95u8, 208u8, 188u8, 25u8, 14u8, 84u8, 63u8, 116u8, 41u8, 148u8, 110u8,
							115u8, 215u8, 196u8, 36u8, 75u8, 102u8,
						],
					)
				}
				#[doc = " The factor to multiply the base delivery fee by."]
				pub fn delivery_fee_factor(
					&self,
					_0: impl ::core::borrow::Borrow<types::delivery_fee_factor::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::delivery_fee_factor::Param0,
					>,
					types::delivery_fee_factor::DeliveryFeeFactor,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"XcmpQueue",
						"DeliveryFeeFactor",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							43u8, 5u8, 63u8, 235u8, 115u8, 155u8, 130u8, 27u8, 75u8, 216u8, 177u8, 135u8, 203u8,
							147u8, 167u8, 95u8, 208u8, 188u8, 25u8, 14u8, 84u8, 63u8, 116u8, 41u8, 148u8, 110u8,
							115u8, 215u8, 196u8, 36u8, 75u8, 102u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum number of inbound XCMP channels that can be suspended simultaneously."]
				#[doc = ""]
				#[doc = " Any further channel suspensions will fail and messages may get dropped without further"]
				#[doc = " notice. Choosing a high value (1000) is okay; the trade-off that is described in"]
				#[doc = " [`InboundXcmpSuspended`] still applies at that scale."]
				pub fn max_inbound_suspended(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"XcmpQueue",
						"MaxInboundSuspended",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " Maximal number of outbound XCMP channels that can have messages queued at the same time."]
				#[doc = ""]
				#[doc = " If this is reached, then no further messages can be sent to channels that do not yet"]
				#[doc = " have a message queued. This should be set to the expected maximum of outbound channels"]
				#[doc = " which is determined by [`Self::ChannelInfo`]. It is important to set this large enough,"]
				#[doc = " since otherwise the congestion control protocol will not work as intended and messages"]
				#[doc = " may be dropped. This value increases the PoV and should therefore not be picked too"]
				#[doc = " high. Governance needs to pay attention to not open more channels than this value."]
				pub fn max_active_outbound_channels(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"XcmpQueue",
						"MaxActiveOutboundChannels",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The maximal page size for HRMP message pages."]
				#[doc = ""]
				#[doc = " A lower limit can be set dynamically, but this is the hard-limit for the PoV worst case"]
				#[doc = " benchmarking. The limit for the size of a message is slightly below this, since some"]
				#[doc = " overhead is incurred for encoding the format."]
				pub fn max_page_size(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"XcmpQueue",
						"MaxPageSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
			}
		}
	}
	pub mod polkadot_xcm {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_xcm::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_xcm::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Send {
					pub dest: ::subxt::ext::subxt_core::alloc::boxed::Box<send::Dest>,
					pub message: ::subxt::ext::subxt_core::alloc::boxed::Box<send::Message>,
				}
				pub mod send {
					use super::runtime_types;
					pub type Dest = runtime_types::xcm::VersionedLocation;
					pub type Message = runtime_types::xcm::VersionedXcm;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Send {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "send";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "**This function is deprecated: Use `limited_teleport_assets` instead.**"]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` chain."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub struct TeleportAssets {
					pub dest: ::subxt::ext::subxt_core::alloc::boxed::Box<teleport_assets::Dest>,
					pub beneficiary:
						::subxt::ext::subxt_core::alloc::boxed::Box<teleport_assets::Beneficiary>,
					pub assets: ::subxt::ext::subxt_core::alloc::boxed::Box<teleport_assets::Assets>,
					pub fee_asset_item: teleport_assets::FeeAssetItem,
				}
				pub mod teleport_assets {
					use super::runtime_types;
					pub type Dest = runtime_types::xcm::VersionedLocation;
					pub type Beneficiary = runtime_types::xcm::VersionedLocation;
					pub type Assets = runtime_types::xcm::VersionedAssets;
					pub type FeeAssetItem = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TeleportAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "teleport_assets";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location and may not be teleportable to `dest`."]
				#[doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `assets` have destination reserve: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"]
				#[doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"]
				#[doc = "   to mint and deposit reserve-based assets to `beneficiary`."]
				#[doc = ""]
				#[doc = "**This function is deprecated: Use `limited_reserve_transfer_assets` instead.**"]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub struct ReserveTransferAssets {
					pub dest: ::subxt::ext::subxt_core::alloc::boxed::Box<reserve_transfer_assets::Dest>,
					pub beneficiary:
						::subxt::ext::subxt_core::alloc::boxed::Box<reserve_transfer_assets::Beneficiary>,
					pub assets: ::subxt::ext::subxt_core::alloc::boxed::Box<reserve_transfer_assets::Assets>,
					pub fee_asset_item: reserve_transfer_assets::FeeAssetItem,
				}
				pub mod reserve_transfer_assets {
					use super::runtime_types;
					pub type Dest = runtime_types::xcm::VersionedLocation;
					pub type Beneficiary = runtime_types::xcm::VersionedLocation;
					pub type Assets = runtime_types::xcm::VersionedAssets;
					pub type FeeAssetItem = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ReserveTransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "reserve_transfer_assets";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Execute an XCM message from a local, signed, origin."]
				#[doc = ""]
				#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
				#[doc = "partially."]
				#[doc = ""]
				#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than"]
				#[doc = "the maximum amount of weight that the message could take to be executed, then no"]
				#[doc = "execution attempt will be made."]
				pub struct Execute {
					pub message: ::subxt::ext::subxt_core::alloc::boxed::Box<execute::Message>,
					pub max_weight: execute::MaxWeight,
				}
				pub mod execute {
					use super::runtime_types;
					pub type Message = runtime_types::xcm::VersionedXcm;
					pub type MaxWeight = runtime_types::sp_weights::weight_v2::Weight;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "execute";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Extoll that a particular destination can be communicated with through a particular"]
				#[doc = "version of XCM."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The destination that is being described."]
				#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
				pub struct ForceXcmVersion {
					pub location: ::subxt::ext::subxt_core::alloc::boxed::Box<force_xcm_version::Location>,
					pub version: force_xcm_version::Version,
				}
				pub mod force_xcm_version {
					use super::runtime_types;
					pub type Location = runtime_types::staging_xcm::v4::location::Location;
					pub type Version = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceXcmVersion {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_xcm_version";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
				#[doc = "version a destination can accept is unknown)."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
				pub struct ForceDefaultXcmVersion {
					pub maybe_xcm_version: force_default_xcm_version::MaybeXcmVersion,
				}
				pub mod force_default_xcm_version {
					use super::runtime_types;
					pub type MaybeXcmVersion = ::core::option::Option<::core::primitive::u32>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceDefaultXcmVersion {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_default_xcm_version";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
				pub struct ForceSubscribeVersionNotify {
					pub location:
						::subxt::ext::subxt_core::alloc::boxed::Box<force_subscribe_version_notify::Location>,
				}
				pub mod force_subscribe_version_notify {
					use super::runtime_types;
					pub type Location = runtime_types::xcm::VersionedLocation;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceSubscribeVersionNotify {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_subscribe_version_notify";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
				#[doc = "version changes."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
				#[doc = "  notifications which we no longer desire."]
				pub struct ForceUnsubscribeVersionNotify {
					pub location:
						::subxt::ext::subxt_core::alloc::boxed::Box<force_unsubscribe_version_notify::Location>,
				}
				pub mod force_unsubscribe_version_notify {
					use super::runtime_types;
					pub type Location = runtime_types::xcm::VersionedLocation;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceUnsubscribeVersionNotify {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_unsubscribe_version_notify";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location and may not be teleportable to `dest`."]
				#[doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `assets` have destination reserve: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"]
				#[doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"]
				#[doc = "   to mint and deposit reserve-based assets to `beneficiary`."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub struct LimitedReserveTransferAssets {
					pub dest:
						::subxt::ext::subxt_core::alloc::boxed::Box<limited_reserve_transfer_assets::Dest>,
					pub beneficiary: ::subxt::ext::subxt_core::alloc::boxed::Box<
						limited_reserve_transfer_assets::Beneficiary,
					>,
					pub assets:
						::subxt::ext::subxt_core::alloc::boxed::Box<limited_reserve_transfer_assets::Assets>,
					pub fee_asset_item: limited_reserve_transfer_assets::FeeAssetItem,
					pub weight_limit: limited_reserve_transfer_assets::WeightLimit,
				}
				pub mod limited_reserve_transfer_assets {
					use super::runtime_types;
					pub type Dest = runtime_types::xcm::VersionedLocation;
					pub type Beneficiary = runtime_types::xcm::VersionedLocation;
					pub type Assets = runtime_types::xcm::VersionedAssets;
					pub type FeeAssetItem = ::core::primitive::u32;
					pub type WeightLimit = runtime_types::xcm::v3::WeightLimit;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for LimitedReserveTransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "limited_reserve_transfer_assets";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` chain."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub struct LimitedTeleportAssets {
					pub dest: ::subxt::ext::subxt_core::alloc::boxed::Box<limited_teleport_assets::Dest>,
					pub beneficiary:
						::subxt::ext::subxt_core::alloc::boxed::Box<limited_teleport_assets::Beneficiary>,
					pub assets: ::subxt::ext::subxt_core::alloc::boxed::Box<limited_teleport_assets::Assets>,
					pub fee_asset_item: limited_teleport_assets::FeeAssetItem,
					pub weight_limit: limited_teleport_assets::WeightLimit,
				}
				pub mod limited_teleport_assets {
					use super::runtime_types;
					pub type Dest = runtime_types::xcm::VersionedLocation;
					pub type Beneficiary = runtime_types::xcm::VersionedLocation;
					pub type Assets = runtime_types::xcm::VersionedAssets;
					pub type FeeAssetItem = ::core::primitive::u32;
					pub type WeightLimit = runtime_types::xcm::v3::WeightLimit;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for LimitedTeleportAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "limited_teleport_assets";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set or unset the global suspension state of the XCM executor."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `suspended`: `true` to suspend, `false` to resume."]
				pub struct ForceSuspension {
					pub suspended: force_suspension::Suspended,
				}
				pub mod force_suspension {
					use super::runtime_types;
					pub type Suspended = ::core::primitive::bool;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceSuspension {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_suspension";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve, or through teleports."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item` (hence referred to as `fees`), up to enough to pay for"]
				#[doc = "`weight_limit` of weight. If more weight is needed than `weight_limit`, then the"]
				#[doc = "operation will fail and the sent assets may be at risk."]
				#[doc = ""]
				#[doc = "`assets` (excluding `fees`) must have same reserve location or otherwise be teleportable"]
				#[doc = "to `dest`, no limitations imposed on `fees`."]
				#[doc = " - for local reserve: transfer assets to sovereign account of destination chain and"]
				#[doc = "   forward a notification XCM to `dest` to mint and deposit reserve-based assets to"]
				#[doc = "   `beneficiary`."]
				#[doc = " - for destination reserve: burn local assets and forward a notification to `dest` chain"]
				#[doc = "   to withdraw the reserve assets from this chain's sovereign account and deposit them"]
				#[doc = "   to `beneficiary`."]
				#[doc = " - for remote reserve: burn local assets, forward XCM to reserve chain to move reserves"]
				#[doc = "   from this chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint"]
				#[doc = "   and deposit reserve-based assets to `beneficiary`."]
				#[doc = " - for teleports: burn local assets and forward XCM to `dest` chain to mint/teleport"]
				#[doc = "   assets and deposit them to `beneficiary`."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent,"]
				#[doc = "  Parachain(..))` to send from parachain to parachain, or `X1(Parachain(..))` to send"]
				#[doc = "  from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub struct TransferAssets {
					pub dest: ::subxt::ext::subxt_core::alloc::boxed::Box<transfer_assets::Dest>,
					pub beneficiary:
						::subxt::ext::subxt_core::alloc::boxed::Box<transfer_assets::Beneficiary>,
					pub assets: ::subxt::ext::subxt_core::alloc::boxed::Box<transfer_assets::Assets>,
					pub fee_asset_item: transfer_assets::FeeAssetItem,
					pub weight_limit: transfer_assets::WeightLimit,
				}
				pub mod transfer_assets {
					use super::runtime_types;
					pub type Dest = runtime_types::xcm::VersionedLocation;
					pub type Beneficiary = runtime_types::xcm::VersionedLocation;
					pub type Assets = runtime_types::xcm::VersionedAssets;
					pub type FeeAssetItem = ::core::primitive::u32;
					pub type WeightLimit = runtime_types::xcm::v3::WeightLimit;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "transfer_assets";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Claims assets trapped on this pallet because of leftover assets during XCM execution."]
				#[doc = ""]
				#[doc = "- `origin`: Anyone can call this extrinsic."]
				#[doc = "- `assets`: The exact assets that were trapped. Use the version to specify what version"]
				#[doc = "was the latest when they were trapped."]
				#[doc = "- `beneficiary`: The location/account where the claimed assets will be deposited."]
				pub struct ClaimAssets {
					pub assets: ::subxt::ext::subxt_core::alloc::boxed::Box<claim_assets::Assets>,
					pub beneficiary: ::subxt::ext::subxt_core::alloc::boxed::Box<claim_assets::Beneficiary>,
				}
				pub mod claim_assets {
					use super::runtime_types;
					pub type Assets = runtime_types::xcm::VersionedAssets;
					pub type Beneficiary = runtime_types::xcm::VersionedLocation;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ClaimAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "claim_assets";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Transfer assets from the local chain to the destination chain using explicit transfer"]
				#[doc = "types for assets and fees."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location or may be teleportable to `dest`. Caller must"]
				#[doc = "provide the `assets_transfer_type` to be used for `assets`:"]
				#[doc = " - `TransferType::LocalReserve`: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `TransferType::DestinationReserve`: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `TransferType::RemoteReserve(reserve)`: burn local assets, forward XCM to `reserve`"]
				#[doc = "   chain to move reserves from this chain's SA to `dest` chain's SA, and forward another"]
				#[doc = "   XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`. Typically"]
				#[doc = "   the remote `reserve` is Asset Hub."]
				#[doc = " - `TransferType::Teleport`: burn local assets and forward XCM to `dest` chain to"]
				#[doc = "   mint/teleport assets and deposit them to `beneficiary`."]
				#[doc = ""]
				#[doc = "On the destination chain, as well as any intermediary hops, `BuyExecution` is used to"]
				#[doc = "buy execution using transferred `assets` identified by `remote_fees_id`."]
				#[doc = "Make sure enough of the specified `remote_fees_id` asset is included in the given list"]
				#[doc = "of `assets`. `remote_fees_id` should be enough to pay for `weight_limit`. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "`remote_fees_id` may use different transfer type than rest of `assets` and can be"]
				#[doc = "specified through `fees_transfer_type`."]
				#[doc = ""]
				#[doc = "The caller needs to specify what should happen to the transferred assets once they reach"]
				#[doc = "the `dest` chain. This is done through the `custom_xcm_on_dest` parameter, which"]
				#[doc = "contains the instructions to execute on `dest` as a final step."]
				#[doc = "  This is usually as simple as:"]
				#[doc = "  `Xcm(vec![DepositAsset { assets: Wild(AllCounted(assets.len())), beneficiary }])`,"]
				#[doc = "  but could be something more exotic like sending the `assets` even further."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain, or `(parents: 2, (GlobalConsensus(..), ..))` to send from"]
				#[doc = "  parachain across a bridge to another ecosystem destination."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `assets_transfer_type`: The XCM `TransferType` used to transfer the `assets`."]
				#[doc = "- `remote_fees_id`: One of the included `assets` to be be used to pay fees."]
				#[doc = "- `fees_transfer_type`: The XCM `TransferType` used to transfer the `fees` assets."]
				#[doc = "- `custom_xcm_on_dest`: The XCM to be executed on `dest` chain as the last step of the"]
				#[doc = "  transfer, which also determines what happens to the assets on the destination chain."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub struct TransferAssetsUsingTypeAndThen {
					pub dest:
						::subxt::ext::subxt_core::alloc::boxed::Box<transfer_assets_using_type_and_then::Dest>,
					pub assets: ::subxt::ext::subxt_core::alloc::boxed::Box<
						transfer_assets_using_type_and_then::Assets,
					>,
					pub assets_transfer_type: ::subxt::ext::subxt_core::alloc::boxed::Box<
						transfer_assets_using_type_and_then::AssetsTransferType,
					>,
					pub remote_fees_id: ::subxt::ext::subxt_core::alloc::boxed::Box<
						transfer_assets_using_type_and_then::RemoteFeesId,
					>,
					pub fees_transfer_type: ::subxt::ext::subxt_core::alloc::boxed::Box<
						transfer_assets_using_type_and_then::FeesTransferType,
					>,
					pub custom_xcm_on_dest: ::subxt::ext::subxt_core::alloc::boxed::Box<
						transfer_assets_using_type_and_then::CustomXcmOnDest,
					>,
					pub weight_limit: transfer_assets_using_type_and_then::WeightLimit,
				}
				pub mod transfer_assets_using_type_and_then {
					use super::runtime_types;
					pub type Dest = runtime_types::xcm::VersionedLocation;
					pub type Assets = runtime_types::xcm::VersionedAssets;
					pub type AssetsTransferType =
						runtime_types::staging_xcm_executor::traits::asset_transfer::TransferType;
					pub type RemoteFeesId = runtime_types::xcm::VersionedAssetId;
					pub type FeesTransferType =
						runtime_types::staging_xcm_executor::traits::asset_transfer::TransferType;
					pub type CustomXcmOnDest = runtime_types::xcm::VersionedXcm;
					pub type WeightLimit = runtime_types::xcm::v3::WeightLimit;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAssetsUsingTypeAndThen {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "transfer_assets_using_type_and_then";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send(
					&self,
					dest: types::send::Dest,
					message: types::send::Message,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Send> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"send",
						types::Send {
							dest: ::subxt::ext::subxt_core::alloc::boxed::Box::new(dest),
							message: ::subxt::ext::subxt_core::alloc::boxed::Box::new(message),
						},
						[
							47u8, 63u8, 128u8, 176u8, 10u8, 137u8, 124u8, 238u8, 155u8, 37u8, 193u8, 160u8, 83u8,
							240u8, 21u8, 179u8, 169u8, 131u8, 27u8, 104u8, 195u8, 208u8, 123u8, 14u8, 221u8,
							12u8, 45u8, 81u8, 148u8, 76u8, 17u8, 100u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "**This function is deprecated: Use `limited_teleport_assets` instead.**"]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` chain."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn teleport_assets(
					&self,
					dest: types::teleport_assets::Dest,
					beneficiary: types::teleport_assets::Beneficiary,
					assets: types::teleport_assets::Assets,
					fee_asset_item: types::teleport_assets::FeeAssetItem,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TeleportAssets> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"teleport_assets",
						types::TeleportAssets {
							dest: ::subxt::ext::subxt_core::alloc::boxed::Box::new(dest),
							beneficiary: ::subxt::ext::subxt_core::alloc::boxed::Box::new(beneficiary),
							assets: ::subxt::ext::subxt_core::alloc::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							124u8, 191u8, 118u8, 61u8, 45u8, 225u8, 97u8, 83u8, 198u8, 20u8, 139u8, 117u8, 241u8,
							1u8, 19u8, 54u8, 79u8, 181u8, 131u8, 112u8, 11u8, 118u8, 147u8, 12u8, 89u8, 156u8,
							123u8, 123u8, 195u8, 45u8, 50u8, 107u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location and may not be teleportable to `dest`."]
				#[doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `assets` have destination reserve: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"]
				#[doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"]
				#[doc = "   to mint and deposit reserve-based assets to `beneficiary`."]
				#[doc = ""]
				#[doc = "**This function is deprecated: Use `limited_reserve_transfer_assets` instead.**"]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn reserve_transfer_assets(
					&self,
					dest: types::reserve_transfer_assets::Dest,
					beneficiary: types::reserve_transfer_assets::Beneficiary,
					assets: types::reserve_transfer_assets::Assets,
					fee_asset_item: types::reserve_transfer_assets::FeeAssetItem,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ReserveTransferAssets>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"reserve_transfer_assets",
						types::ReserveTransferAssets {
							dest: ::subxt::ext::subxt_core::alloc::boxed::Box::new(dest),
							beneficiary: ::subxt::ext::subxt_core::alloc::boxed::Box::new(beneficiary),
							assets: ::subxt::ext::subxt_core::alloc::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							97u8, 102u8, 230u8, 44u8, 135u8, 197u8, 43u8, 53u8, 182u8, 125u8, 140u8, 141u8,
							229u8, 73u8, 29u8, 55u8, 159u8, 104u8, 197u8, 20u8, 124u8, 234u8, 250u8, 94u8, 133u8,
							253u8, 189u8, 6u8, 216u8, 162u8, 218u8, 89u8,
						],
					)
				}
				#[doc = "Execute an XCM message from a local, signed, origin."]
				#[doc = ""]
				#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
				#[doc = "partially."]
				#[doc = ""]
				#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than"]
				#[doc = "the maximum amount of weight that the message could take to be executed, then no"]
				#[doc = "execution attempt will be made."]
				pub fn execute(
					&self,
					message: types::execute::Message,
					max_weight: types::execute::MaxWeight,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Execute> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"execute",
						types::Execute {
							message: ::subxt::ext::subxt_core::alloc::boxed::Box::new(message),
							max_weight,
						},
						[
							71u8, 109u8, 92u8, 110u8, 198u8, 150u8, 140u8, 125u8, 248u8, 236u8, 177u8, 156u8,
							198u8, 223u8, 51u8, 15u8, 52u8, 240u8, 20u8, 200u8, 68u8, 145u8, 36u8, 156u8, 159u8,
							153u8, 125u8, 48u8, 181u8, 61u8, 53u8, 208u8,
						],
					)
				}
				#[doc = "Extoll that a particular destination can be communicated with through a particular"]
				#[doc = "version of XCM."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The destination that is being described."]
				#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
				pub fn force_xcm_version(
					&self,
					location: types::force_xcm_version::Location,
					version: types::force_xcm_version::Version,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceXcmVersion> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"force_xcm_version",
						types::ForceXcmVersion {
							location: ::subxt::ext::subxt_core::alloc::boxed::Box::new(location),
							version,
						},
						[
							69u8, 151u8, 198u8, 154u8, 69u8, 181u8, 41u8, 111u8, 145u8, 230u8, 103u8, 42u8,
							237u8, 91u8, 235u8, 6u8, 156u8, 65u8, 187u8, 48u8, 171u8, 200u8, 49u8, 4u8, 9u8,
							210u8, 229u8, 152u8, 187u8, 88u8, 80u8, 246u8,
						],
					)
				}
				#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
				#[doc = "version a destination can accept is unknown)."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: types::force_default_xcm_version::MaybeXcmVersion,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceDefaultXcmVersion>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"force_default_xcm_version",
						types::ForceDefaultXcmVersion { maybe_xcm_version },
						[
							43u8, 114u8, 102u8, 104u8, 209u8, 234u8, 108u8, 173u8, 109u8, 188u8, 94u8, 214u8,
							136u8, 43u8, 153u8, 75u8, 161u8, 192u8, 76u8, 12u8, 221u8, 237u8, 158u8, 247u8, 41u8,
							193u8, 35u8, 174u8, 183u8, 207u8, 79u8, 213u8,
						],
					)
				}
				#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
				pub fn force_subscribe_version_notify(
					&self,
					location: types::force_subscribe_version_notify::Location,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceSubscribeVersionNotify>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"force_subscribe_version_notify",
						types::ForceSubscribeVersionNotify {
							location: ::subxt::ext::subxt_core::alloc::boxed::Box::new(location),
						},
						[
							203u8, 171u8, 70u8, 130u8, 46u8, 63u8, 76u8, 50u8, 105u8, 23u8, 249u8, 190u8, 115u8,
							74u8, 70u8, 125u8, 132u8, 112u8, 138u8, 60u8, 33u8, 35u8, 45u8, 29u8, 95u8, 103u8,
							187u8, 182u8, 188u8, 196u8, 248u8, 152u8,
						],
					)
				}
				#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
				#[doc = "version changes."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
				#[doc = "  notifications which we no longer desire."]
				pub fn force_unsubscribe_version_notify(
					&self,
					location: types::force_unsubscribe_version_notify::Location,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
					types::ForceUnsubscribeVersionNotify,
				> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"force_unsubscribe_version_notify",
						types::ForceUnsubscribeVersionNotify {
							location: ::subxt::ext::subxt_core::alloc::boxed::Box::new(location),
						},
						[
							6u8, 113u8, 168u8, 215u8, 233u8, 202u8, 249u8, 134u8, 131u8, 8u8, 142u8, 203u8,
							142u8, 95u8, 216u8, 70u8, 38u8, 99u8, 166u8, 97u8, 218u8, 132u8, 247u8, 14u8, 42u8,
							99u8, 4u8, 115u8, 200u8, 180u8, 213u8, 50u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location and may not be teleportable to `dest`."]
				#[doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `assets` have destination reserve: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"]
				#[doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"]
				#[doc = "   to mint and deposit reserve-based assets to `beneficiary`."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_reserve_transfer_assets(
					&self,
					dest: types::limited_reserve_transfer_assets::Dest,
					beneficiary: types::limited_reserve_transfer_assets::Beneficiary,
					assets: types::limited_reserve_transfer_assets::Assets,
					fee_asset_item: types::limited_reserve_transfer_assets::FeeAssetItem,
					weight_limit: types::limited_reserve_transfer_assets::WeightLimit,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::LimitedReserveTransferAssets>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"limited_reserve_transfer_assets",
						types::LimitedReserveTransferAssets {
							dest: ::subxt::ext::subxt_core::alloc::boxed::Box::new(dest),
							beneficiary: ::subxt::ext::subxt_core::alloc::boxed::Box::new(beneficiary),
							assets: ::subxt::ext::subxt_core::alloc::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							198u8, 66u8, 204u8, 162u8, 222u8, 246u8, 141u8, 165u8, 241u8, 62u8, 43u8, 236u8,
							56u8, 200u8, 54u8, 47u8, 174u8, 83u8, 167u8, 220u8, 174u8, 111u8, 123u8, 202u8,
							248u8, 232u8, 166u8, 80u8, 152u8, 223u8, 86u8, 141u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` chain."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_teleport_assets(
					&self,
					dest: types::limited_teleport_assets::Dest,
					beneficiary: types::limited_teleport_assets::Beneficiary,
					assets: types::limited_teleport_assets::Assets,
					fee_asset_item: types::limited_teleport_assets::FeeAssetItem,
					weight_limit: types::limited_teleport_assets::WeightLimit,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::LimitedTeleportAssets>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"limited_teleport_assets",
						types::LimitedTeleportAssets {
							dest: ::subxt::ext::subxt_core::alloc::boxed::Box::new(dest),
							beneficiary: ::subxt::ext::subxt_core::alloc::boxed::Box::new(beneficiary),
							assets: ::subxt::ext::subxt_core::alloc::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							70u8, 61u8, 32u8, 43u8, 101u8, 104u8, 251u8, 60u8, 212u8, 124u8, 113u8, 243u8, 241u8,
							183u8, 5u8, 231u8, 209u8, 231u8, 136u8, 3u8, 145u8, 242u8, 179u8, 171u8, 185u8,
							185u8, 7u8, 34u8, 5u8, 203u8, 21u8, 210u8,
						],
					)
				}
				#[doc = "Set or unset the global suspension state of the XCM executor."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `suspended`: `true` to suspend, `false` to resume."]
				pub fn force_suspension(
					&self,
					suspended: types::force_suspension::Suspended,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceSuspension> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"force_suspension",
						types::ForceSuspension { suspended },
						[
							78u8, 125u8, 93u8, 55u8, 129u8, 44u8, 36u8, 227u8, 75u8, 46u8, 68u8, 202u8, 81u8,
							127u8, 111u8, 92u8, 149u8, 38u8, 225u8, 185u8, 183u8, 154u8, 89u8, 159u8, 79u8, 10u8,
							229u8, 1u8, 226u8, 243u8, 65u8, 238u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve, or through teleports."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item` (hence referred to as `fees`), up to enough to pay for"]
				#[doc = "`weight_limit` of weight. If more weight is needed than `weight_limit`, then the"]
				#[doc = "operation will fail and the sent assets may be at risk."]
				#[doc = ""]
				#[doc = "`assets` (excluding `fees`) must have same reserve location or otherwise be teleportable"]
				#[doc = "to `dest`, no limitations imposed on `fees`."]
				#[doc = " - for local reserve: transfer assets to sovereign account of destination chain and"]
				#[doc = "   forward a notification XCM to `dest` to mint and deposit reserve-based assets to"]
				#[doc = "   `beneficiary`."]
				#[doc = " - for destination reserve: burn local assets and forward a notification to `dest` chain"]
				#[doc = "   to withdraw the reserve assets from this chain's sovereign account and deposit them"]
				#[doc = "   to `beneficiary`."]
				#[doc = " - for remote reserve: burn local assets, forward XCM to reserve chain to move reserves"]
				#[doc = "   from this chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint"]
				#[doc = "   and deposit reserve-based assets to `beneficiary`."]
				#[doc = " - for teleports: burn local assets and forward XCM to `dest` chain to mint/teleport"]
				#[doc = "   assets and deposit them to `beneficiary`."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent,"]
				#[doc = "  Parachain(..))` to send from parachain to parachain, or `X1(Parachain(..))` to send"]
				#[doc = "  from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn transfer_assets(
					&self,
					dest: types::transfer_assets::Dest,
					beneficiary: types::transfer_assets::Beneficiary,
					assets: types::transfer_assets::Assets,
					fee_asset_item: types::transfer_assets::FeeAssetItem,
					weight_limit: types::transfer_assets::WeightLimit,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAssets> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"transfer_assets",
						types::TransferAssets {
							dest: ::subxt::ext::subxt_core::alloc::boxed::Box::new(dest),
							beneficiary: ::subxt::ext::subxt_core::alloc::boxed::Box::new(beneficiary),
							assets: ::subxt::ext::subxt_core::alloc::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							44u8, 155u8, 182u8, 37u8, 123u8, 148u8, 150u8, 191u8, 117u8, 32u8, 16u8, 238u8,
							121u8, 188u8, 217u8, 110u8, 10u8, 236u8, 174u8, 91u8, 100u8, 201u8, 109u8, 109u8,
							60u8, 177u8, 233u8, 66u8, 181u8, 191u8, 105u8, 37u8,
						],
					)
				}
				#[doc = "Claims assets trapped on this pallet because of leftover assets during XCM execution."]
				#[doc = ""]
				#[doc = "- `origin`: Anyone can call this extrinsic."]
				#[doc = "- `assets`: The exact assets that were trapped. Use the version to specify what version"]
				#[doc = "was the latest when they were trapped."]
				#[doc = "- `beneficiary`: The location/account where the claimed assets will be deposited."]
				pub fn claim_assets(
					&self,
					assets: types::claim_assets::Assets,
					beneficiary: types::claim_assets::Beneficiary,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ClaimAssets> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"claim_assets",
						types::ClaimAssets {
							assets: ::subxt::ext::subxt_core::alloc::boxed::Box::new(assets),
							beneficiary: ::subxt::ext::subxt_core::alloc::boxed::Box::new(beneficiary),
						},
						[
							155u8, 23u8, 166u8, 172u8, 251u8, 171u8, 136u8, 240u8, 253u8, 51u8, 164u8, 43u8,
							141u8, 23u8, 189u8, 177u8, 33u8, 32u8, 212u8, 56u8, 174u8, 165u8, 129u8, 7u8, 49u8,
							217u8, 213u8, 214u8, 250u8, 91u8, 200u8, 195u8,
						],
					)
				}
				#[doc = "Transfer assets from the local chain to the destination chain using explicit transfer"]
				#[doc = "types for assets and fees."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location or may be teleportable to `dest`. Caller must"]
				#[doc = "provide the `assets_transfer_type` to be used for `assets`:"]
				#[doc = " - `TransferType::LocalReserve`: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `TransferType::DestinationReserve`: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `TransferType::RemoteReserve(reserve)`: burn local assets, forward XCM to `reserve`"]
				#[doc = "   chain to move reserves from this chain's SA to `dest` chain's SA, and forward another"]
				#[doc = "   XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`. Typically"]
				#[doc = "   the remote `reserve` is Asset Hub."]
				#[doc = " - `TransferType::Teleport`: burn local assets and forward XCM to `dest` chain to"]
				#[doc = "   mint/teleport assets and deposit them to `beneficiary`."]
				#[doc = ""]
				#[doc = "On the destination chain, as well as any intermediary hops, `BuyExecution` is used to"]
				#[doc = "buy execution using transferred `assets` identified by `remote_fees_id`."]
				#[doc = "Make sure enough of the specified `remote_fees_id` asset is included in the given list"]
				#[doc = "of `assets`. `remote_fees_id` should be enough to pay for `weight_limit`. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "`remote_fees_id` may use different transfer type than rest of `assets` and can be"]
				#[doc = "specified through `fees_transfer_type`."]
				#[doc = ""]
				#[doc = "The caller needs to specify what should happen to the transferred assets once they reach"]
				#[doc = "the `dest` chain. This is done through the `custom_xcm_on_dest` parameter, which"]
				#[doc = "contains the instructions to execute on `dest` as a final step."]
				#[doc = "  This is usually as simple as:"]
				#[doc = "  `Xcm(vec![DepositAsset { assets: Wild(AllCounted(assets.len())), beneficiary }])`,"]
				#[doc = "  but could be something more exotic like sending the `assets` even further."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain, or `(parents: 2, (GlobalConsensus(..), ..))` to send from"]
				#[doc = "  parachain across a bridge to another ecosystem destination."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `assets_transfer_type`: The XCM `TransferType` used to transfer the `assets`."]
				#[doc = "- `remote_fees_id`: One of the included `assets` to be be used to pay fees."]
				#[doc = "- `fees_transfer_type`: The XCM `TransferType` used to transfer the `fees` assets."]
				#[doc = "- `custom_xcm_on_dest`: The XCM to be executed on `dest` chain as the last step of the"]
				#[doc = "  transfer, which also determines what happens to the assets on the destination chain."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn transfer_assets_using_type_and_then(
					&self,
					dest: types::transfer_assets_using_type_and_then::Dest,
					assets: types::transfer_assets_using_type_and_then::Assets,
					assets_transfer_type: types::transfer_assets_using_type_and_then::AssetsTransferType,
					remote_fees_id: types::transfer_assets_using_type_and_then::RemoteFeesId,
					fees_transfer_type: types::transfer_assets_using_type_and_then::FeesTransferType,
					custom_xcm_on_dest: types::transfer_assets_using_type_and_then::CustomXcmOnDest,
					weight_limit: types::transfer_assets_using_type_and_then::WeightLimit,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
					types::TransferAssetsUsingTypeAndThen,
				> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"PolkadotXcm",
						"transfer_assets_using_type_and_then",
						types::TransferAssetsUsingTypeAndThen {
							dest: ::subxt::ext::subxt_core::alloc::boxed::Box::new(dest),
							assets: ::subxt::ext::subxt_core::alloc::boxed::Box::new(assets),
							assets_transfer_type: ::subxt::ext::subxt_core::alloc::boxed::Box::new(
								assets_transfer_type,
							),
							remote_fees_id: ::subxt::ext::subxt_core::alloc::boxed::Box::new(remote_fees_id),
							fees_transfer_type: ::subxt::ext::subxt_core::alloc::boxed::Box::new(
								fees_transfer_type,
							),
							custom_xcm_on_dest: ::subxt::ext::subxt_core::alloc::boxed::Box::new(
								custom_xcm_on_dest,
							),
							weight_limit,
						},
						[
							128u8, 51u8, 64u8, 139u8, 106u8, 225u8, 14u8, 247u8, 44u8, 109u8, 11u8, 15u8, 7u8,
							235u8, 7u8, 195u8, 177u8, 94u8, 9u8, 107u8, 110u8, 174u8, 154u8, 157u8, 20u8, 232u8,
							38u8, 207u8, 228u8, 151u8, 10u8, 226u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Execution of an XCM message was attempted."]
			pub struct Attempted {
				pub outcome: attempted::Outcome,
			}
			pub mod attempted {
				use super::runtime_types;
				pub type Outcome = runtime_types::staging_xcm::v4::traits::Outcome;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Attempted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A XCM message was sent."]
			pub struct Sent {
				pub origin: sent::Origin,
				pub destination: sent::Destination,
				pub message: sent::Message,
				pub message_id: sent::MessageId,
			}
			pub mod sent {
				use super::runtime_types;
				pub type Origin = runtime_types::staging_xcm::v4::location::Location;
				pub type Destination = runtime_types::staging_xcm::v4::location::Location;
				pub type Message = runtime_types::staging_xcm::v4::Xcm;
				pub type MessageId = [::core::primitive::u8; 32usize];
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Sent {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Sent";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Query response received which does not match a registered query. This may be because a"]
			#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
			#[doc = "because the query timed out."]
			pub struct UnexpectedResponse {
				pub origin: unexpected_response::Origin,
				pub query_id: unexpected_response::QueryId,
			}
			pub mod unexpected_response {
				use super::runtime_types;
				pub type Origin = runtime_types::staging_xcm::v4::location::Location;
				pub type QueryId = ::core::primitive::u64;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for UnexpectedResponse {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
			#[doc = "no registered notification call."]
			pub struct ResponseReady {
				pub query_id: response_ready::QueryId,
				pub response: response_ready::Response,
			}
			pub mod response_ready {
				use super::runtime_types;
				pub type QueryId = ::core::primitive::u64;
				pub type Response = runtime_types::staging_xcm::v4::Response;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ResponseReady {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Query response has been received and query is removed. The registered notification has"]
			#[doc = "been dispatched and executed successfully."]
			pub struct Notified {
				pub query_id: notified::QueryId,
				pub pallet_index: notified::PalletIndex,
				pub call_index: notified::CallIndex,
			}
			pub mod notified {
				use super::runtime_types;
				pub type QueryId = ::core::primitive::u64;
				pub type PalletIndex = ::core::primitive::u8;
				pub type CallIndex = ::core::primitive::u8;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Notified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Notified";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Query response has been received and query is removed. The registered notification"]
			#[doc = "could not be dispatched because the dispatch weight is greater than the maximum weight"]
			#[doc = "originally budgeted by this runtime for the query result."]
			pub struct NotifyOverweight {
				pub query_id: notify_overweight::QueryId,
				pub pallet_index: notify_overweight::PalletIndex,
				pub call_index: notify_overweight::CallIndex,
				pub actual_weight: notify_overweight::ActualWeight,
				pub max_budgeted_weight: notify_overweight::MaxBudgetedWeight,
			}
			pub mod notify_overweight {
				use super::runtime_types;
				pub type QueryId = ::core::primitive::u64;
				pub type PalletIndex = ::core::primitive::u8;
				pub type CallIndex = ::core::primitive::u8;
				pub type ActualWeight = runtime_types::sp_weights::weight_v2::Weight;
				pub type MaxBudgetedWeight = runtime_types::sp_weights::weight_v2::Weight;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NotifyOverweight {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyOverweight";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Query response has been received and query is removed. There was a general error with"]
			#[doc = "dispatching the notification call."]
			pub struct NotifyDispatchError {
				pub query_id: notify_dispatch_error::QueryId,
				pub pallet_index: notify_dispatch_error::PalletIndex,
				pub call_index: notify_dispatch_error::CallIndex,
			}
			pub mod notify_dispatch_error {
				use super::runtime_types;
				pub type QueryId = ::core::primitive::u64;
				pub type PalletIndex = ::core::primitive::u8;
				pub type CallIndex = ::core::primitive::u8;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NotifyDispatchError {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
			#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
			#[doc = "is not `(origin, QueryId, Response)`."]
			pub struct NotifyDecodeFailed {
				pub query_id: notify_decode_failed::QueryId,
				pub pallet_index: notify_decode_failed::PalletIndex,
				pub call_index: notify_decode_failed::CallIndex,
			}
			pub mod notify_decode_failed {
				use super::runtime_types;
				pub type QueryId = ::core::primitive::u64;
				pub type PalletIndex = ::core::primitive::u8;
				pub type CallIndex = ::core::primitive::u8;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NotifyDecodeFailed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Expected query response has been received but the origin location of the response does"]
			#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
			#[doc = "be received and acted upon."]
			pub struct InvalidResponder {
				pub origin: invalid_responder::Origin,
				pub query_id: invalid_responder::QueryId,
				pub expected_location: invalid_responder::ExpectedLocation,
			}
			pub mod invalid_responder {
				use super::runtime_types;
				pub type Origin = runtime_types::staging_xcm::v4::location::Location;
				pub type QueryId = ::core::primitive::u64;
				pub type ExpectedLocation =
					::core::option::Option<runtime_types::staging_xcm::v4::location::Location>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for InvalidResponder {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Expected query response has been received but the expected origin location placed in"]
			#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
			#[doc = ""]
			#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
			#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
			#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
			#[doc = "needed."]
			pub struct InvalidResponderVersion {
				pub origin: invalid_responder_version::Origin,
				pub query_id: invalid_responder_version::QueryId,
			}
			pub mod invalid_responder_version {
				use super::runtime_types;
				pub type Origin = runtime_types::staging_xcm::v4::location::Location;
				pub type QueryId = ::core::primitive::u64;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for InvalidResponderVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Received query response has been read and removed."]
			pub struct ResponseTaken {
				pub query_id: response_taken::QueryId,
			}
			pub mod response_taken {
				use super::runtime_types;
				pub type QueryId = ::core::primitive::u64;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ResponseTaken {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some assets have been placed in an asset trap."]
			pub struct AssetsTrapped {
				pub hash: assets_trapped::Hash,
				pub origin: assets_trapped::Origin,
				pub assets: assets_trapped::Assets,
			}
			pub mod assets_trapped {
				use super::runtime_types;
				pub type Hash = ::subxt::ext::subxt_core::utils::H256;
				pub type Origin = runtime_types::staging_xcm::v4::location::Location;
				pub type Assets = runtime_types::xcm::VersionedAssets;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for AssetsTrapped {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "An XCM version change notification message has been attempted to be sent."]
			#[doc = ""]
			#[doc = "The cost of sending it (borne by the chain) is included."]
			pub struct VersionChangeNotified {
				pub destination: version_change_notified::Destination,
				pub result: version_change_notified::Result,
				pub cost: version_change_notified::Cost,
				pub message_id: version_change_notified::MessageId,
			}
			pub mod version_change_notified {
				use super::runtime_types;
				pub type Destination = runtime_types::staging_xcm::v4::location::Location;
				pub type Result = ::core::primitive::u32;
				pub type Cost = runtime_types::staging_xcm::v4::asset::Assets;
				pub type MessageId = [::core::primitive::u8; 32usize];
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for VersionChangeNotified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The supported version of a location has been changed. This might be through an"]
			#[doc = "automatic notification or a manual intervention."]
			pub struct SupportedVersionChanged {
				pub location: supported_version_changed::Location,
				pub version: supported_version_changed::Version,
			}
			pub mod supported_version_changed {
				use super::runtime_types;
				pub type Location = runtime_types::staging_xcm::v4::location::Location;
				pub type Version = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for SupportedVersionChanged {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "sending the notification to it."]
			pub struct NotifyTargetSendFail {
				pub location: notify_target_send_fail::Location,
				pub query_id: notify_target_send_fail::QueryId,
				pub error: notify_target_send_fail::Error,
			}
			pub mod notify_target_send_fail {
				use super::runtime_types;
				pub type Location = runtime_types::staging_xcm::v4::location::Location;
				pub type QueryId = ::core::primitive::u64;
				pub type Error = runtime_types::xcm::v3::traits::Error;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NotifyTargetSendFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "migrating the location to our new XCM format."]
			pub struct NotifyTargetMigrationFail {
				pub location: notify_target_migration_fail::Location,
				pub query_id: notify_target_migration_fail::QueryId,
			}
			pub mod notify_target_migration_fail {
				use super::runtime_types;
				pub type Location = runtime_types::xcm::VersionedLocation;
				pub type QueryId = ::core::primitive::u64;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NotifyTargetMigrationFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Expected query response has been received but the expected querier location placed in"]
			#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
			#[doc = ""]
			#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
			#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
			#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
			#[doc = "needed."]
			pub struct InvalidQuerierVersion {
				pub origin: invalid_querier_version::Origin,
				pub query_id: invalid_querier_version::QueryId,
			}
			pub mod invalid_querier_version {
				use super::runtime_types;
				pub type Origin = runtime_types::staging_xcm::v4::location::Location;
				pub type QueryId = ::core::primitive::u64;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for InvalidQuerierVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerierVersion";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Expected query response has been received but the querier location of the response does"]
			#[doc = "not match the expected. The query remains registered for a later, valid, response to"]
			#[doc = "be received and acted upon."]
			pub struct InvalidQuerier {
				pub origin: invalid_querier::Origin,
				pub query_id: invalid_querier::QueryId,
				pub expected_querier: invalid_querier::ExpectedQuerier,
				pub maybe_actual_querier: invalid_querier::MaybeActualQuerier,
			}
			pub mod invalid_querier {
				use super::runtime_types;
				pub type Origin = runtime_types::staging_xcm::v4::location::Location;
				pub type QueryId = ::core::primitive::u64;
				pub type ExpectedQuerier = runtime_types::staging_xcm::v4::location::Location;
				pub type MaybeActualQuerier =
					::core::option::Option<runtime_types::staging_xcm::v4::location::Location>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for InvalidQuerier {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerier";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A remote has requested XCM version change notification from us and we have honored it."]
			#[doc = "A version information message is sent to them and its cost is included."]
			pub struct VersionNotifyStarted {
				pub destination: version_notify_started::Destination,
				pub cost: version_notify_started::Cost,
				pub message_id: version_notify_started::MessageId,
			}
			pub mod version_notify_started {
				use super::runtime_types;
				pub type Destination = runtime_types::staging_xcm::v4::location::Location;
				pub type Cost = runtime_types::staging_xcm::v4::asset::Assets;
				pub type MessageId = [::core::primitive::u8; 32usize];
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for VersionNotifyStarted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyStarted";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "We have requested that a remote chain send us XCM version change notifications."]
			pub struct VersionNotifyRequested {
				pub destination: version_notify_requested::Destination,
				pub cost: version_notify_requested::Cost,
				pub message_id: version_notify_requested::MessageId,
			}
			pub mod version_notify_requested {
				use super::runtime_types;
				pub type Destination = runtime_types::staging_xcm::v4::location::Location;
				pub type Cost = runtime_types::staging_xcm::v4::asset::Assets;
				pub type MessageId = [::core::primitive::u8; 32usize];
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for VersionNotifyRequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyRequested";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "We have requested that a remote chain stops sending us XCM version change"]
			#[doc = "notifications."]
			pub struct VersionNotifyUnrequested {
				pub destination: version_notify_unrequested::Destination,
				pub cost: version_notify_unrequested::Cost,
				pub message_id: version_notify_unrequested::MessageId,
			}
			pub mod version_notify_unrequested {
				use super::runtime_types;
				pub type Destination = runtime_types::staging_xcm::v4::location::Location;
				pub type Cost = runtime_types::staging_xcm::v4::asset::Assets;
				pub type MessageId = [::core::primitive::u8; 32usize];
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for VersionNotifyUnrequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyUnrequested";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Fees were paid from a location for an operation (often for using `SendXcm`)."]
			pub struct FeesPaid {
				pub paying: fees_paid::Paying,
				pub fees: fees_paid::Fees,
			}
			pub mod fees_paid {
				use super::runtime_types;
				pub type Paying = runtime_types::staging_xcm::v4::location::Location;
				pub type Fees = runtime_types::staging_xcm::v4::asset::Assets;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for FeesPaid {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "FeesPaid";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Some assets have been claimed from an asset trap"]
			pub struct AssetsClaimed {
				pub hash: assets_claimed::Hash,
				pub origin: assets_claimed::Origin,
				pub assets: assets_claimed::Assets,
			}
			pub mod assets_claimed {
				use super::runtime_types;
				pub type Hash = ::subxt::ext::subxt_core::utils::H256;
				pub type Origin = runtime_types::staging_xcm::v4::location::Location;
				pub type Assets = runtime_types::xcm::VersionedAssets;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for AssetsClaimed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsClaimed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A XCM version migration finished."]
			pub struct VersionMigrationFinished {
				pub version: version_migration_finished::Version,
			}
			pub mod version_migration_finished {
				use super::runtime_types;
				pub type Version = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for VersionMigrationFinished {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionMigrationFinished";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod query_counter {
					use super::runtime_types;
					pub type QueryCounter = ::core::primitive::u64;
				}
				pub mod queries {
					use super::runtime_types;
					pub type Queries = runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>;
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod asset_traps {
					use super::runtime_types;
					pub type AssetTraps = ::core::primitive::u32;
					pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
				}
				pub mod safe_xcm_version {
					use super::runtime_types;
					pub type SafeXcmVersion = ::core::primitive::u32;
				}
				pub mod supported_version {
					use super::runtime_types;
					pub type SupportedVersion = ::core::primitive::u32;
					pub type Param0 = ::core::primitive::u32;
					pub type Param1 = runtime_types::xcm::VersionedLocation;
				}
				pub mod version_notifiers {
					use super::runtime_types;
					pub type VersionNotifiers = ::core::primitive::u64;
					pub type Param0 = ::core::primitive::u32;
					pub type Param1 = runtime_types::xcm::VersionedLocation;
				}
				pub mod version_notify_targets {
					use super::runtime_types;
					pub type VersionNotifyTargets = (
						::core::primitive::u64,
						runtime_types::sp_weights::weight_v2::Weight,
						::core::primitive::u32,
					);
					pub type Param0 = ::core::primitive::u32;
					pub type Param1 = runtime_types::xcm::VersionedLocation;
				}
				pub mod version_discovery_queue {
					use super::runtime_types;
					pub type VersionDiscoveryQueue =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							runtime_types::xcm::VersionedLocation,
							::core::primitive::u32,
						)>;
				}
				pub mod current_migration {
					use super::runtime_types;
					pub type CurrentMigration = runtime_types::pallet_xcm::pallet::VersionMigrationStage;
				}
				pub mod remote_locked_fungibles {
					use super::runtime_types;
					pub type RemoteLockedFungibles =
						runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord<()>;
					pub type Param0 = ::core::primitive::u32;
					pub type Param1 = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param2 = runtime_types::xcm::VersionedAssetId;
				}
				pub mod locked_fungibles {
					use super::runtime_types;
					pub type LockedFungibles = runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u128,
						runtime_types::xcm::VersionedLocation,
					)>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod xcm_execution_suspended {
					use super::runtime_types;
					pub type XcmExecutionSuspended = ::core::primitive::bool;
				}
				pub mod should_record_xcm {
					use super::runtime_types;
					pub type ShouldRecordXcm = ::core::primitive::bool;
				}
				pub mod recorded_xcm {
					use super::runtime_types;
					pub type RecordedXcm = runtime_types::staging_xcm::v4::Xcm;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The latest available query index."]
				pub fn query_counter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::query_counter::QueryCounter,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"QueryCounter",
						(),
						[
							216u8, 73u8, 160u8, 232u8, 60u8, 245u8, 218u8, 219u8, 152u8, 68u8, 146u8, 219u8,
							255u8, 7u8, 86u8, 112u8, 83u8, 49u8, 94u8, 173u8, 64u8, 203u8, 147u8, 226u8, 236u8,
							39u8, 129u8, 106u8, 209u8, 113u8, 150u8, 50u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::queries::Queries,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"Queries",
						(),
						[
							246u8, 75u8, 240u8, 129u8, 106u8, 114u8, 99u8, 154u8, 176u8, 188u8, 146u8, 125u8,
							244u8, 103u8, 187u8, 171u8, 60u8, 119u8, 4u8, 90u8, 58u8, 180u8, 48u8, 165u8, 145u8,
							125u8, 227u8, 233u8, 11u8, 142u8, 122u8, 3u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries(
					&self,
					_0: impl ::core::borrow::Borrow<types::queries::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::queries::Param0>,
					types::queries::Queries,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"Queries",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							246u8, 75u8, 240u8, 129u8, 106u8, 114u8, 99u8, 154u8, 176u8, 188u8, 146u8, 125u8,
							244u8, 103u8, 187u8, 171u8, 60u8, 119u8, 4u8, 90u8, 58u8, 180u8, 48u8, 165u8, 145u8,
							125u8, 227u8, 233u8, 11u8, 142u8, 122u8, 3u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `Assets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::asset_traps::AssetTraps,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"AssetTraps",
						(),
						[
							148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8, 50u8, 77u8, 226u8,
							8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8, 126u8, 28u8, 28u8, 202u8, 62u8,
							87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `Assets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps(
					&self,
					_0: impl ::core::borrow::Borrow<types::asset_traps::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::asset_traps::Param0>,
					types::asset_traps::AssetTraps,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"AssetTraps",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8, 50u8, 77u8, 226u8,
							8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8, 126u8, 28u8, 28u8, 202u8, 62u8,
							87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
						],
					)
				}
				#[doc = " Default version to encode XCM when latest version of destination is unknown. If `None`,"]
				#[doc = " then the destinations whose XCM version is unknown are considered unreachable."]
				pub fn safe_xcm_version(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::safe_xcm_version::SafeXcmVersion,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"SafeXcmVersion",
						(),
						[
							187u8, 8u8, 74u8, 126u8, 80u8, 215u8, 177u8, 60u8, 223u8, 123u8, 196u8, 155u8, 166u8,
							66u8, 25u8, 164u8, 191u8, 66u8, 116u8, 131u8, 116u8, 188u8, 224u8, 122u8, 75u8,
							195u8, 246u8, 188u8, 83u8, 134u8, 49u8, 143u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::supported_version::SupportedVersion,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						(),
						[
							144u8, 218u8, 177u8, 254u8, 210u8, 8u8, 84u8, 149u8, 163u8, 162u8, 238u8, 37u8,
							157u8, 28u8, 140u8, 121u8, 201u8, 173u8, 204u8, 92u8, 133u8, 45u8, 156u8, 38u8, 61u8,
							51u8, 153u8, 161u8, 147u8, 146u8, 202u8, 24u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::supported_version::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::supported_version::Param0,
					>,
					types::supported_version::SupportedVersion,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							144u8, 218u8, 177u8, 254u8, 210u8, 8u8, 84u8, 149u8, 163u8, 162u8, 238u8, 37u8,
							157u8, 28u8, 140u8, 121u8, 201u8, 173u8, 204u8, 92u8, 133u8, 45u8, 156u8, 38u8, 61u8,
							51u8, 153u8, 161u8, 147u8, 146u8, 202u8, 24u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version(
					&self,
					_0: impl ::core::borrow::Borrow<types::supported_version::Param0>,
					_1: impl ::core::borrow::Borrow<types::supported_version::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::supported_version::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::supported_version::Param1,
						>,
					),
					types::supported_version::SupportedVersion,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							144u8, 218u8, 177u8, 254u8, 210u8, 8u8, 84u8, 149u8, 163u8, 162u8, 238u8, 37u8,
							157u8, 28u8, 140u8, 121u8, 201u8, 173u8, 204u8, 92u8, 133u8, 45u8, 156u8, 38u8, 61u8,
							51u8, 153u8, 161u8, 147u8, 146u8, 202u8, 24u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::version_notifiers::VersionNotifiers,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						(),
						[
							175u8, 206u8, 29u8, 14u8, 111u8, 123u8, 211u8, 109u8, 159u8, 131u8, 80u8, 149u8,
							216u8, 196u8, 181u8, 105u8, 117u8, 138u8, 80u8, 69u8, 237u8, 116u8, 195u8, 66u8,
							209u8, 102u8, 42u8, 126u8, 222u8, 176u8, 201u8, 49u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::version_notifiers::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::version_notifiers::Param0,
					>,
					types::version_notifiers::VersionNotifiers,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							175u8, 206u8, 29u8, 14u8, 111u8, 123u8, 211u8, 109u8, 159u8, 131u8, 80u8, 149u8,
							216u8, 196u8, 181u8, 105u8, 117u8, 138u8, 80u8, 69u8, 237u8, 116u8, 195u8, 66u8,
							209u8, 102u8, 42u8, 126u8, 222u8, 176u8, 201u8, 49u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers(
					&self,
					_0: impl ::core::borrow::Borrow<types::version_notifiers::Param0>,
					_1: impl ::core::borrow::Borrow<types::version_notifiers::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::version_notifiers::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::version_notifiers::Param1,
						>,
					),
					types::version_notifiers::VersionNotifiers,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							175u8, 206u8, 29u8, 14u8, 111u8, 123u8, 211u8, 109u8, 159u8, 131u8, 80u8, 149u8,
							216u8, 196u8, 181u8, 105u8, 117u8, 138u8, 80u8, 69u8, 237u8, 116u8, 195u8, 66u8,
							209u8, 102u8, 42u8, 126u8, 222u8, 176u8, 201u8, 49u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::version_notify_targets::VersionNotifyTargets,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						(),
						[
							113u8, 77u8, 150u8, 42u8, 82u8, 49u8, 195u8, 120u8, 96u8, 80u8, 152u8, 67u8, 27u8,
							142u8, 10u8, 74u8, 66u8, 134u8, 35u8, 202u8, 77u8, 187u8, 174u8, 22u8, 207u8, 199u8,
							57u8, 85u8, 53u8, 208u8, 146u8, 81u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::version_notify_targets::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::version_notify_targets::Param0,
					>,
					types::version_notify_targets::VersionNotifyTargets,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							113u8, 77u8, 150u8, 42u8, 82u8, 49u8, 195u8, 120u8, 96u8, 80u8, 152u8, 67u8, 27u8,
							142u8, 10u8, 74u8, 66u8, 134u8, 35u8, 202u8, 77u8, 187u8, 174u8, 22u8, 207u8, 199u8,
							57u8, 85u8, 53u8, 208u8, 146u8, 81u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets(
					&self,
					_0: impl ::core::borrow::Borrow<types::version_notify_targets::Param0>,
					_1: impl ::core::borrow::Borrow<types::version_notify_targets::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::version_notify_targets::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::version_notify_targets::Param1,
						>,
					),
					types::version_notify_targets::VersionNotifyTargets,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							113u8, 77u8, 150u8, 42u8, 82u8, 49u8, 195u8, 120u8, 96u8, 80u8, 152u8, 67u8, 27u8,
							142u8, 10u8, 74u8, 66u8, 134u8, 35u8, 202u8, 77u8, 187u8, 174u8, 22u8, 207u8, 199u8,
							57u8, 85u8, 53u8, 208u8, 146u8, 81u8,
						],
					)
				}
				#[doc = " Destinations whose latest XCM version we would like to know. Duplicates not allowed, and"]
				#[doc = " the `u32` counter is the number of times that a send to the destination has been attempted,"]
				#[doc = " which is used as a prioritization."]
				pub fn version_discovery_queue(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::version_discovery_queue::VersionDiscoveryQueue,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"VersionDiscoveryQueue",
						(),
						[
							95u8, 74u8, 97u8, 94u8, 40u8, 140u8, 175u8, 176u8, 224u8, 222u8, 83u8, 199u8, 170u8,
							102u8, 3u8, 77u8, 127u8, 208u8, 155u8, 122u8, 176u8, 51u8, 15u8, 253u8, 231u8, 245u8,
							91u8, 192u8, 60u8, 144u8, 101u8, 168u8,
						],
					)
				}
				#[doc = " The current migration's stage, if any."]
				pub fn current_migration(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::current_migration::CurrentMigration,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"CurrentMigration",
						(),
						[
							74u8, 138u8, 181u8, 162u8, 59u8, 251u8, 37u8, 28u8, 232u8, 51u8, 30u8, 152u8, 252u8,
							133u8, 95u8, 195u8, 47u8, 127u8, 21u8, 44u8, 62u8, 143u8, 170u8, 234u8, 160u8, 37u8,
							131u8, 179u8, 57u8, 241u8, 140u8, 124u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on a remote chain."]
				pub fn remote_locked_fungibles_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::remote_locked_fungibles::RemoteLockedFungibles,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						(),
						[
							247u8, 124u8, 77u8, 42u8, 208u8, 183u8, 99u8, 196u8, 50u8, 113u8, 250u8, 221u8,
							222u8, 170u8, 10u8, 60u8, 143u8, 172u8, 149u8, 198u8, 125u8, 154u8, 196u8, 196u8,
							145u8, 209u8, 68u8, 28u8, 241u8, 241u8, 201u8, 150u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on a remote chain."]
				pub fn remote_locked_fungibles_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::remote_locked_fungibles::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::remote_locked_fungibles::Param0,
					>,
					types::remote_locked_fungibles::RemoteLockedFungibles,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							247u8, 124u8, 77u8, 42u8, 208u8, 183u8, 99u8, 196u8, 50u8, 113u8, 250u8, 221u8,
							222u8, 170u8, 10u8, 60u8, 143u8, 172u8, 149u8, 198u8, 125u8, 154u8, 196u8, 196u8,
							145u8, 209u8, 68u8, 28u8, 241u8, 241u8, 201u8, 150u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on a remote chain."]
				pub fn remote_locked_fungibles_iter2(
					&self,
					_0: impl ::core::borrow::Borrow<types::remote_locked_fungibles::Param0>,
					_1: impl ::core::borrow::Borrow<types::remote_locked_fungibles::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::remote_locked_fungibles::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::remote_locked_fungibles::Param1,
						>,
					),
					types::remote_locked_fungibles::RemoteLockedFungibles,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							247u8, 124u8, 77u8, 42u8, 208u8, 183u8, 99u8, 196u8, 50u8, 113u8, 250u8, 221u8,
							222u8, 170u8, 10u8, 60u8, 143u8, 172u8, 149u8, 198u8, 125u8, 154u8, 196u8, 196u8,
							145u8, 209u8, 68u8, 28u8, 241u8, 241u8, 201u8, 150u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on a remote chain."]
				pub fn remote_locked_fungibles(
					&self,
					_0: impl ::core::borrow::Borrow<types::remote_locked_fungibles::Param0>,
					_1: impl ::core::borrow::Borrow<types::remote_locked_fungibles::Param1>,
					_2: impl ::core::borrow::Borrow<types::remote_locked_fungibles::Param2>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::remote_locked_fungibles::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::remote_locked_fungibles::Param1,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::remote_locked_fungibles::Param2,
						>,
					),
					types::remote_locked_fungibles::RemoteLockedFungibles,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_2.borrow()),
						),
						[
							247u8, 124u8, 77u8, 42u8, 208u8, 183u8, 99u8, 196u8, 50u8, 113u8, 250u8, 221u8,
							222u8, 170u8, 10u8, 60u8, 143u8, 172u8, 149u8, 198u8, 125u8, 154u8, 196u8, 196u8,
							145u8, 209u8, 68u8, 28u8, 241u8, 241u8, 201u8, 150u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on this chain."]
				pub fn locked_fungibles_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::locked_fungibles::LockedFungibles,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"LockedFungibles",
						(),
						[
							254u8, 234u8, 1u8, 27u8, 27u8, 32u8, 217u8, 24u8, 47u8, 30u8, 62u8, 80u8, 86u8,
							125u8, 120u8, 24u8, 143u8, 229u8, 161u8, 153u8, 240u8, 246u8, 80u8, 15u8, 49u8,
							189u8, 20u8, 204u8, 239u8, 198u8, 97u8, 174u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on this chain."]
				pub fn locked_fungibles(
					&self,
					_0: impl ::core::borrow::Borrow<types::locked_fungibles::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::locked_fungibles::Param0,
					>,
					types::locked_fungibles::LockedFungibles,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"LockedFungibles",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							254u8, 234u8, 1u8, 27u8, 27u8, 32u8, 217u8, 24u8, 47u8, 30u8, 62u8, 80u8, 86u8,
							125u8, 120u8, 24u8, 143u8, 229u8, 161u8, 153u8, 240u8, 246u8, 80u8, 15u8, 49u8,
							189u8, 20u8, 204u8, 239u8, 198u8, 97u8, 174u8,
						],
					)
				}
				#[doc = " Global suspension state of the XCM executor."]
				pub fn xcm_execution_suspended(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::xcm_execution_suspended::XcmExecutionSuspended,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"XcmExecutionSuspended",
						(),
						[
							182u8, 54u8, 69u8, 68u8, 78u8, 76u8, 103u8, 79u8, 47u8, 136u8, 99u8, 104u8, 128u8,
							129u8, 249u8, 54u8, 214u8, 136u8, 97u8, 48u8, 178u8, 42u8, 26u8, 27u8, 82u8, 24u8,
							33u8, 77u8, 33u8, 27u8, 20u8, 127u8,
						],
					)
				}
				#[doc = " Whether or not incoming XCMs (both executed locally and received) should be recorded."]
				#[doc = " Only one XCM program will be recorded at a time."]
				#[doc = " This is meant to be used in runtime APIs, and it's advised it stays false"]
				#[doc = " for all other use cases, so as to not degrade regular performance."]
				#[doc = ""]
				#[doc = " Only relevant if this pallet is being used as the [`xcm_executor::traits::RecordXcm`]"]
				#[doc = " implementation in the XCM executor configuration."]
				pub fn should_record_xcm(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::should_record_xcm::ShouldRecordXcm,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"ShouldRecordXcm",
						(),
						[
							77u8, 184u8, 154u8, 92u8, 185u8, 225u8, 131u8, 210u8, 55u8, 115u8, 3u8, 182u8, 191u8,
							132u8, 51u8, 136u8, 42u8, 136u8, 54u8, 36u8, 229u8, 229u8, 47u8, 88u8, 4u8, 175u8,
							136u8, 78u8, 226u8, 253u8, 13u8, 178u8,
						],
					)
				}
				#[doc = " If [`ShouldRecordXcm`] is set to true, then the last XCM program executed locally"]
				#[doc = " will be stored here."]
				#[doc = " Runtime APIs can fetch the XCM that was executed by accessing this value."]
				#[doc = ""]
				#[doc = " Only relevant if this pallet is being used as the [`xcm_executor::traits::RecordXcm`]"]
				#[doc = " implementation in the XCM executor configuration."]
				pub fn recorded_xcm(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::recorded_xcm::RecordedXcm,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"PolkadotXcm",
						"RecordedXcm",
						(),
						[
							20u8, 231u8, 100u8, 77u8, 9u8, 170u8, 144u8, 49u8, 131u8, 233u8, 184u8, 123u8, 186u8,
							56u8, 115u8, 3u8, 79u8, 234u8, 71u8, 93u8, 87u8, 172u8, 2u8, 3u8, 144u8, 151u8,
							135u8, 149u8, 106u8, 96u8, 125u8, 12u8,
						],
					)
				}
			}
		}
	}
	pub mod cumulus_xcm {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::cumulus_pallet_xcm::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Downward message is invalid XCM."]
			#[doc = "\\[ id \\]"]
			pub struct InvalidFormat(pub invalid_format::Field0);
			pub mod invalid_format {
				use super::runtime_types;
				pub type Field0 = [::core::primitive::u8; 32usize];
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Downward message is unsupported version of XCM."]
			#[doc = "\\[ id \\]"]
			pub struct UnsupportedVersion(pub unsupported_version::Field0);
			pub mod unsupported_version {
				use super::runtime_types;
				pub type Field0 = [::core::primitive::u8; 32usize];
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Downward message executed with the given outcome."]
			#[doc = "\\[ id, outcome \\]"]
			pub struct ExecutedDownward(pub executed_downward::Field0, pub executed_downward::Field1);
			pub mod executed_downward {
				use super::runtime_types;
				pub type Field0 = [::core::primitive::u8; 32usize];
				pub type Field1 = runtime_types::staging_xcm::v4::traits::Outcome;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "ExecutedDownward";
			}
		}
	}
	pub mod message_queue {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_message_queue::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_message_queue::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Remove a page which has no more messages remaining to be processed or is stale."]
				pub struct ReapPage {
					pub message_origin: reap_page::MessageOrigin,
					pub page_index: reap_page::PageIndex,
				}
				pub mod reap_page {
					use super::runtime_types;
					pub type MessageOrigin = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
					pub type PageIndex = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ReapPage {
					const PALLET: &'static str = "MessageQueue";
					const CALL: &'static str = "reap_page";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Execute an overweight message."]
				#[doc = ""]
				#[doc = "Temporary processing errors will be propagated whereas permanent errors are treated"]
				#[doc = "as success condition."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `message_origin`: The origin from which the message to be executed arrived."]
				#[doc = "- `page`: The page in the queue in which the message to be executed is sitting."]
				#[doc = "- `index`: The index into the queue of the message to be executed."]
				#[doc = "- `weight_limit`: The maximum amount of weight allowed to be consumed in the execution"]
				#[doc = "  of the message."]
				#[doc = ""]
				#[doc = "Benchmark complexity considerations: O(index + weight_limit)."]
				pub struct ExecuteOverweight {
					pub message_origin: execute_overweight::MessageOrigin,
					pub page: execute_overweight::Page,
					pub index: execute_overweight::Index,
					pub weight_limit: execute_overweight::WeightLimit,
				}
				pub mod execute_overweight {
					use super::runtime_types;
					pub type MessageOrigin = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
					pub type Page = ::core::primitive::u32;
					pub type Index = ::core::primitive::u32;
					pub type WeightLimit = runtime_types::sp_weights::weight_v2::Weight;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ExecuteOverweight {
					const PALLET: &'static str = "MessageQueue";
					const CALL: &'static str = "execute_overweight";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Remove a page which has no more messages remaining to be processed or is stale."]
				pub fn reap_page(
					&self,
					message_origin: types::reap_page::MessageOrigin,
					page_index: types::reap_page::PageIndex,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ReapPage> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"MessageQueue",
						"reap_page",
						types::ReapPage {
							message_origin,
							page_index,
						},
						[
							116u8, 17u8, 120u8, 238u8, 117u8, 222u8, 10u8, 166u8, 132u8, 181u8, 114u8, 150u8,
							242u8, 202u8, 31u8, 143u8, 212u8, 65u8, 145u8, 249u8, 27u8, 204u8, 137u8, 133u8,
							220u8, 187u8, 137u8, 90u8, 112u8, 55u8, 104u8, 163u8,
						],
					)
				}
				#[doc = "Execute an overweight message."]
				#[doc = ""]
				#[doc = "Temporary processing errors will be propagated whereas permanent errors are treated"]
				#[doc = "as success condition."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `message_origin`: The origin from which the message to be executed arrived."]
				#[doc = "- `page`: The page in the queue in which the message to be executed is sitting."]
				#[doc = "- `index`: The index into the queue of the message to be executed."]
				#[doc = "- `weight_limit`: The maximum amount of weight allowed to be consumed in the execution"]
				#[doc = "  of the message."]
				#[doc = ""]
				#[doc = "Benchmark complexity considerations: O(index + weight_limit)."]
				pub fn execute_overweight(
					&self,
					message_origin: types::execute_overweight::MessageOrigin,
					page: types::execute_overweight::Page,
					index: types::execute_overweight::Index,
					weight_limit: types::execute_overweight::WeightLimit,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ExecuteOverweight> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"MessageQueue",
						"execute_overweight",
						types::ExecuteOverweight {
							message_origin,
							page,
							index,
							weight_limit,
						},
						[
							177u8, 54u8, 82u8, 58u8, 94u8, 125u8, 241u8, 172u8, 52u8, 7u8, 236u8, 80u8, 66u8,
							99u8, 42u8, 199u8, 38u8, 195u8, 65u8, 118u8, 166u8, 246u8, 239u8, 195u8, 144u8,
							153u8, 155u8, 8u8, 224u8, 56u8, 106u8, 135u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_message_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Message discarded due to an error in the `MessageProcessor` (usually a format error)."]
			pub struct ProcessingFailed {
				pub id: processing_failed::Id,
				pub origin: processing_failed::Origin,
				pub error: processing_failed::Error,
			}
			pub mod processing_failed {
				use super::runtime_types;
				pub type Id = ::subxt::ext::subxt_core::utils::H256;
				pub type Origin = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
				pub type Error = runtime_types::frame_support::traits::messages::ProcessMessageError;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ProcessingFailed {
				const PALLET: &'static str = "MessageQueue";
				const EVENT: &'static str = "ProcessingFailed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Message is processed."]
			pub struct Processed {
				pub id: processed::Id,
				pub origin: processed::Origin,
				pub weight_used: processed::WeightUsed,
				pub success: processed::Success,
			}
			pub mod processed {
				use super::runtime_types;
				pub type Id = ::subxt::ext::subxt_core::utils::H256;
				pub type Origin = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
				pub type WeightUsed = runtime_types::sp_weights::weight_v2::Weight;
				pub type Success = ::core::primitive::bool;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for Processed {
				const PALLET: &'static str = "MessageQueue";
				const EVENT: &'static str = "Processed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Message placed in overweight queue."]
			pub struct OverweightEnqueued {
				pub id: overweight_enqueued::Id,
				pub origin: overweight_enqueued::Origin,
				pub page_index: overweight_enqueued::PageIndex,
				pub message_index: overweight_enqueued::MessageIndex,
			}
			pub mod overweight_enqueued {
				use super::runtime_types;
				pub type Id = [::core::primitive::u8; 32usize];
				pub type Origin = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
				pub type PageIndex = ::core::primitive::u32;
				pub type MessageIndex = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "MessageQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "This page was reaped."]
			pub struct PageReaped {
				pub origin: page_reaped::Origin,
				pub index: page_reaped::Index,
			}
			pub mod page_reaped {
				use super::runtime_types;
				pub type Origin = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
				pub type Index = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for PageReaped {
				const PALLET: &'static str = "MessageQueue";
				const EVENT: &'static str = "PageReaped";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod book_state_for {
					use super::runtime_types;
					pub type BookStateFor = runtime_types::pallet_message_queue::BookState<
						runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					>;
					pub type Param0 = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
				}
				pub mod service_head {
					use super::runtime_types;
					pub type ServiceHead = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
				}
				pub mod pages {
					use super::runtime_types;
					pub type Pages = runtime_types::pallet_message_queue::Page<::core::primitive::u32>;
					pub type Param0 = runtime_types::cumulus_primitives_core::AggregateMessageOrigin;
					pub type Param1 = ::core::primitive::u32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The index of the first and last (non-empty) pages."]
				pub fn book_state_for_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::book_state_for::BookStateFor,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"MessageQueue",
						"BookStateFor",
						(),
						[
							33u8, 240u8, 235u8, 59u8, 150u8, 42u8, 91u8, 248u8, 235u8, 52u8, 170u8, 52u8, 195u8,
							129u8, 6u8, 174u8, 57u8, 242u8, 30u8, 220u8, 232u8, 4u8, 246u8, 218u8, 162u8, 174u8,
							102u8, 95u8, 210u8, 92u8, 133u8, 143u8,
						],
					)
				}
				#[doc = " The index of the first and last (non-empty) pages."]
				pub fn book_state_for(
					&self,
					_0: impl ::core::borrow::Borrow<types::book_state_for::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::book_state_for::Param0,
					>,
					types::book_state_for::BookStateFor,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"MessageQueue",
						"BookStateFor",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							33u8, 240u8, 235u8, 59u8, 150u8, 42u8, 91u8, 248u8, 235u8, 52u8, 170u8, 52u8, 195u8,
							129u8, 6u8, 174u8, 57u8, 242u8, 30u8, 220u8, 232u8, 4u8, 246u8, 218u8, 162u8, 174u8,
							102u8, 95u8, 210u8, 92u8, 133u8, 143u8,
						],
					)
				}
				#[doc = " The origin at which we should begin servicing."]
				pub fn service_head(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::service_head::ServiceHead,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"MessageQueue",
						"ServiceHead",
						(),
						[
							104u8, 146u8, 240u8, 41u8, 171u8, 68u8, 20u8, 147u8, 212u8, 155u8, 59u8, 39u8, 174u8,
							186u8, 97u8, 250u8, 41u8, 247u8, 67u8, 190u8, 252u8, 167u8, 234u8, 36u8, 124u8,
							239u8, 163u8, 72u8, 223u8, 82u8, 82u8, 171u8,
						],
					)
				}
				#[doc = " The map of page indices to pages."]
				pub fn pages_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::pages::Pages,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"MessageQueue",
						"Pages",
						(),
						[
							45u8, 202u8, 18u8, 128u8, 31u8, 194u8, 175u8, 173u8, 99u8, 81u8, 161u8, 44u8, 32u8,
							183u8, 238u8, 181u8, 110u8, 240u8, 203u8, 12u8, 152u8, 58u8, 239u8, 190u8, 144u8,
							168u8, 210u8, 33u8, 121u8, 250u8, 137u8, 142u8,
						],
					)
				}
				#[doc = " The map of page indices to pages."]
				pub fn pages_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::pages::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::pages::Param0>,
					types::pages::Pages,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"MessageQueue",
						"Pages",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							45u8, 202u8, 18u8, 128u8, 31u8, 194u8, 175u8, 173u8, 99u8, 81u8, 161u8, 44u8, 32u8,
							183u8, 238u8, 181u8, 110u8, 240u8, 203u8, 12u8, 152u8, 58u8, 239u8, 190u8, 144u8,
							168u8, 210u8, 33u8, 121u8, 250u8, 137u8, 142u8,
						],
					)
				}
				#[doc = " The map of page indices to pages."]
				pub fn pages(
					&self,
					_0: impl ::core::borrow::Borrow<types::pages::Param0>,
					_1: impl ::core::borrow::Borrow<types::pages::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::pages::Param0>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::pages::Param1>,
					),
					types::pages::Pages,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"MessageQueue",
						"Pages",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							45u8, 202u8, 18u8, 128u8, 31u8, 194u8, 175u8, 173u8, 99u8, 81u8, 161u8, 44u8, 32u8,
							183u8, 238u8, 181u8, 110u8, 240u8, 203u8, 12u8, 152u8, 58u8, 239u8, 190u8, 144u8,
							168u8, 210u8, 33u8, 121u8, 250u8, 137u8, 142u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The size of the page; this implies the maximum message size which can be sent."]
				#[doc = ""]
				#[doc = " A good value depends on the expected message sizes, their weights, the weight that is"]
				#[doc = " available for processing them and the maximal needed message size. The maximal message"]
				#[doc = " size is slightly lower than this as defined by [`MaxMessageLenOf`]."]
				pub fn heap_size(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"MessageQueue",
						"HeapSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The maximum number of stale pages (i.e. of overweight messages) allowed before culling"]
				#[doc = " can happen. Once there are more stale pages than this, then historical pages may be"]
				#[doc = " dropped, even if they contain unprocessed overweight messages."]
				pub fn max_stale(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"MessageQueue",
						"MaxStale",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The amount of weight (if any) which should be provided to the message queue for"]
				#[doc = " servicing enqueued items `on_initialize`."]
				#[doc = ""]
				#[doc = " This may be legitimately `None` in the case that you will call"]
				#[doc = " `ServiceQueues::service_queues` manually or set [`Self::IdleMaxServiceWeight`] to have"]
				#[doc = " it run in `on_idle`."]
				pub fn service_weight(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
					::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				> {
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"MessageQueue",
						"ServiceWeight",
						[
							204u8, 140u8, 63u8, 167u8, 49u8, 8u8, 148u8, 163u8, 190u8, 224u8, 15u8, 103u8, 86u8,
							153u8, 248u8, 117u8, 223u8, 117u8, 210u8, 80u8, 205u8, 155u8, 40u8, 11u8, 59u8, 63u8,
							129u8, 156u8, 17u8, 83u8, 177u8, 250u8,
						],
					)
				}
				#[doc = " The maximum amount of weight (if any) to be used from remaining weight `on_idle` which"]
				#[doc = " should be provided to the message queue for servicing enqueued items `on_idle`."]
				#[doc = " Useful for parachains to process messages at the same block they are received."]
				#[doc = ""]
				#[doc = " If `None`, it will not call `ServiceQueues::service_queues` in `on_idle`."]
				pub fn idle_max_service_weight(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
					::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				> {
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"MessageQueue",
						"IdleMaxServiceWeight",
						[
							204u8, 140u8, 63u8, 167u8, 49u8, 8u8, 148u8, 163u8, 190u8, 224u8, 15u8, 103u8, 86u8,
							153u8, 248u8, 117u8, 223u8, 117u8, 210u8, 80u8, 205u8, 155u8, 40u8, 11u8, 59u8, 63u8,
							129u8, 156u8, 17u8, 83u8, 177u8, 250u8,
						],
					)
				}
			}
		}
	}
	pub mod oracle {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::orml_oracle::module::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::orml_oracle::module::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Feed the external value."]
				#[doc = ""]
				#[doc = "Require authorized operator."]
				pub struct FeedValues {
					pub values: feed_values::Values,
				}
				pub mod feed_values {
					use super::runtime_types;
					pub type Values = runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						(
							::subxt::ext::subxt_core::utils::AccountId32,
							::core::primitive::u64,
						),
						runtime_types::cyborg_primitives::oracle::ProcessStatus,
					)>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for FeedValues {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "feed_values";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Feed the external value."]
				#[doc = ""]
				#[doc = "Require authorized operator."]
				pub fn feed_values(
					&self,
					values: types::feed_values::Values,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::FeedValues> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Oracle",
						"feed_values",
						types::FeedValues { values },
						[
							178u8, 5u8, 38u8, 243u8, 207u8, 5u8, 228u8, 23u8, 222u8, 231u8, 120u8, 245u8, 147u8,
							181u8, 243u8, 107u8, 121u8, 57u8, 50u8, 71u8, 245u8, 241u8, 249u8, 168u8, 8u8, 211u8,
							3u8, 53u8, 157u8, 218u8, 90u8, 214u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::orml_oracle::module::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "New feed data is submitted."]
			pub struct NewFeedData {
				pub sender: new_feed_data::Sender,
				pub values: new_feed_data::Values,
			}
			pub mod new_feed_data {
				use super::runtime_types;
				pub type Sender = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Values = ::subxt::ext::subxt_core::alloc::vec::Vec<(
					(
						::subxt::ext::subxt_core::utils::AccountId32,
						::core::primitive::u64,
					),
					runtime_types::cyborg_primitives::oracle::ProcessStatus,
				)>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for NewFeedData {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "NewFeedData";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod raw_values {
					use super::runtime_types;
					pub type RawValues = runtime_types::orml_oracle::module::TimestampedValue<
						runtime_types::cyborg_primitives::oracle::ProcessStatus,
						::core::primitive::u64,
					>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param1 = (
						::subxt::ext::subxt_core::utils::AccountId32,
						::core::primitive::u64,
					);
				}
				pub mod values {
					use super::runtime_types;
					pub type Values = runtime_types::orml_oracle::module::TimestampedValue<
						runtime_types::cyborg_primitives::oracle::ProcessStatus,
						::core::primitive::u64,
					>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param1 = ::core::primitive::u64;
				}
				pub mod has_dispatched {
					use super::runtime_types;
					pub type HasDispatched = runtime_types::orml_utilities::ordered_set::OrderedSet<
						::subxt::ext::subxt_core::utils::AccountId32,
					>;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Raw values for each oracle operators"]
				pub fn raw_values_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::raw_values::RawValues,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Oracle",
						"RawValues",
						(),
						[
							208u8, 117u8, 207u8, 123u8, 101u8, 23u8, 126u8, 204u8, 44u8, 90u8, 72u8, 222u8,
							186u8, 193u8, 29u8, 155u8, 66u8, 239u8, 3u8, 150u8, 124u8, 169u8, 107u8, 130u8, 29u8,
							130u8, 136u8, 226u8, 200u8, 128u8, 17u8, 161u8,
						],
					)
				}
				#[doc = " Raw values for each oracle operators"]
				pub fn raw_values_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::raw_values::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::raw_values::Param0>,
					types::raw_values::RawValues,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Oracle",
						"RawValues",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							208u8, 117u8, 207u8, 123u8, 101u8, 23u8, 126u8, 204u8, 44u8, 90u8, 72u8, 222u8,
							186u8, 193u8, 29u8, 155u8, 66u8, 239u8, 3u8, 150u8, 124u8, 169u8, 107u8, 130u8, 29u8,
							130u8, 136u8, 226u8, 200u8, 128u8, 17u8, 161u8,
						],
					)
				}
				#[doc = " Raw values for each oracle operators"]
				pub fn raw_values(
					&self,
					_0: impl ::core::borrow::Borrow<types::raw_values::Param0>,
					_1: impl ::core::borrow::Borrow<types::raw_values::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::raw_values::Param0>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::raw_values::Param1>,
					),
					types::raw_values::RawValues,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Oracle",
						"RawValues",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							208u8, 117u8, 207u8, 123u8, 101u8, 23u8, 126u8, 204u8, 44u8, 90u8, 72u8, 222u8,
							186u8, 193u8, 29u8, 155u8, 66u8, 239u8, 3u8, 150u8, 124u8, 169u8, 107u8, 130u8, 29u8,
							130u8, 136u8, 226u8, 200u8, 128u8, 17u8, 161u8,
						],
					)
				}
				#[doc = " Up to date combined value from Raw Values"]
				pub fn values_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::values::Values,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Oracle",
						"Values",
						(),
						[
							248u8, 159u8, 209u8, 235u8, 18u8, 164u8, 201u8, 51u8, 235u8, 187u8, 126u8, 29u8,
							72u8, 18u8, 233u8, 173u8, 136u8, 5u8, 143u8, 187u8, 5u8, 190u8, 10u8, 65u8, 196u8,
							67u8, 118u8, 224u8, 170u8, 20u8, 69u8, 224u8,
						],
					)
				}
				#[doc = " Up to date combined value from Raw Values"]
				pub fn values_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::values::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::values::Param0>,
					types::values::Values,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Oracle",
						"Values",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							248u8, 159u8, 209u8, 235u8, 18u8, 164u8, 201u8, 51u8, 235u8, 187u8, 126u8, 29u8,
							72u8, 18u8, 233u8, 173u8, 136u8, 5u8, 143u8, 187u8, 5u8, 190u8, 10u8, 65u8, 196u8,
							67u8, 118u8, 224u8, 170u8, 20u8, 69u8, 224u8,
						],
					)
				}
				#[doc = " Up to date combined value from Raw Values"]
				pub fn values(
					&self,
					_0: impl ::core::borrow::Borrow<types::values::Param0>,
					_1: impl ::core::borrow::Borrow<types::values::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::values::Param0>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::values::Param1>,
					),
					types::values::Values,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Oracle",
						"Values",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							248u8, 159u8, 209u8, 235u8, 18u8, 164u8, 201u8, 51u8, 235u8, 187u8, 126u8, 29u8,
							72u8, 18u8, 233u8, 173u8, 136u8, 5u8, 143u8, 187u8, 5u8, 190u8, 10u8, 65u8, 196u8,
							67u8, 118u8, 224u8, 170u8, 20u8, 69u8, 224u8,
						],
					)
				}
				#[doc = " If an oracle operator has fed a value in this block"]
				pub fn has_dispatched(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::has_dispatched::HasDispatched,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Oracle",
						"HasDispatched",
						(),
						[
							86u8, 143u8, 165u8, 88u8, 160u8, 239u8, 65u8, 190u8, 215u8, 245u8, 110u8, 118u8,
							227u8, 89u8, 203u8, 78u8, 175u8, 121u8, 81u8, 40u8, 142u8, 99u8, 207u8, 13u8, 156u8,
							245u8, 166u8, 158u8, 36u8, 243u8, 164u8, 21u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The root operator account id, record all sudo feeds on this account."]
				pub fn root_operator_account_id(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
					::subxt::ext::subxt_core::utils::AccountId32,
				> {
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Oracle",
						"RootOperatorAccountId",
						[
							115u8, 233u8, 13u8, 223u8, 88u8, 20u8, 202u8, 139u8, 153u8, 28u8, 155u8, 157u8,
							224u8, 66u8, 3u8, 250u8, 23u8, 53u8, 88u8, 168u8, 211u8, 204u8, 122u8, 166u8, 248u8,
							23u8, 174u8, 225u8, 99u8, 108u8, 89u8, 135u8,
						],
					)
				}
				#[doc = " Maximum size of HasDispatched"]
				pub fn max_has_dispatched_size(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Oracle",
						"MaxHasDispatchedSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " Maximum size the vector used for feed values"]
				pub fn max_feed_values(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"Oracle",
						"MaxFeedValues",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
			}
		}
	}
	pub mod oracle_membership {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_membership::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_membership::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Add a member `who` to the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::AddOrigin`."]
				pub struct AddMember {
					pub who: add_member::Who,
				}
				pub mod add_member {
					use super::runtime_types;
					pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddMember {
					const PALLET: &'static str = "OracleMembership";
					const CALL: &'static str = "add_member";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Remove a member `who` from the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::RemoveOrigin`."]
				pub struct RemoveMember {
					pub who: remove_member::Who,
				}
				pub mod remove_member {
					use super::runtime_types;
					pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveMember {
					const PALLET: &'static str = "OracleMembership";
					const CALL: &'static str = "remove_member";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Swap out one member `remove` for another `add`."]
				#[doc = ""]
				#[doc = "May only be called from `T::SwapOrigin`."]
				#[doc = ""]
				#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
				pub struct SwapMember {
					pub remove: swap_member::Remove,
					pub add: swap_member::Add,
				}
				pub mod swap_member {
					use super::runtime_types;
					pub type Remove = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
					pub type Add = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SwapMember {
					const PALLET: &'static str = "OracleMembership";
					const CALL: &'static str = "swap_member";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
				#[doc = "pass `members` pre-sorted."]
				#[doc = ""]
				#[doc = "May only be called from `T::ResetOrigin`."]
				pub struct ResetMembers {
					pub members: reset_members::Members,
				}
				pub mod reset_members {
					use super::runtime_types;
					pub type Members =
						::subxt::ext::subxt_core::alloc::vec::Vec<::subxt::ext::subxt_core::utils::AccountId32>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ResetMembers {
					const PALLET: &'static str = "OracleMembership";
					const CALL: &'static str = "reset_members";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Swap out the sending member for some other key `new`."]
				#[doc = ""]
				#[doc = "May only be called from `Signed` origin of a current member."]
				#[doc = ""]
				#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
				pub struct ChangeKey {
					pub new: change_key::New,
				}
				pub mod change_key {
					use super::runtime_types;
					pub type New = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ChangeKey {
					const PALLET: &'static str = "OracleMembership";
					const CALL: &'static str = "change_key";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Set the prime member. Must be a current member."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub struct SetPrime {
					pub who: set_prime::Who,
				}
				pub mod set_prime {
					use super::runtime_types;
					pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
						::subxt::ext::subxt_core::utils::AccountId32,
						(),
					>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetPrime {
					const PALLET: &'static str = "OracleMembership";
					const CALL: &'static str = "set_prime";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Remove the prime member if it exists."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub struct ClearPrime;
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ClearPrime {
					const PALLET: &'static str = "OracleMembership";
					const CALL: &'static str = "clear_prime";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Add a member `who` to the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::AddOrigin`."]
				pub fn add_member(
					&self,
					who: types::add_member::Who,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AddMember> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"OracleMembership",
						"add_member",
						types::AddMember { who },
						[
							2u8, 131u8, 37u8, 217u8, 112u8, 46u8, 86u8, 165u8, 248u8, 244u8, 33u8, 236u8, 155u8,
							28u8, 163u8, 169u8, 213u8, 32u8, 70u8, 217u8, 97u8, 194u8, 138u8, 77u8, 133u8, 97u8,
							188u8, 49u8, 49u8, 31u8, 177u8, 206u8,
						],
					)
				}
				#[doc = "Remove a member `who` from the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::RemoveOrigin`."]
				pub fn remove_member(
					&self,
					who: types::remove_member::Who,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveMember> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"OracleMembership",
						"remove_member",
						types::RemoveMember { who },
						[
							78u8, 153u8, 97u8, 110u8, 121u8, 242u8, 112u8, 56u8, 195u8, 217u8, 10u8, 202u8,
							114u8, 134u8, 220u8, 237u8, 198u8, 109u8, 247u8, 85u8, 156u8, 88u8, 138u8, 79u8,
							189u8, 37u8, 230u8, 55u8, 1u8, 27u8, 89u8, 80u8,
						],
					)
				}
				#[doc = "Swap out one member `remove` for another `add`."]
				#[doc = ""]
				#[doc = "May only be called from `T::SwapOrigin`."]
				#[doc = ""]
				#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
				pub fn swap_member(
					&self,
					remove: types::swap_member::Remove,
					add: types::swap_member::Add,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SwapMember> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"OracleMembership",
						"swap_member",
						types::SwapMember { remove, add },
						[
							170u8, 68u8, 212u8, 185u8, 186u8, 38u8, 222u8, 227u8, 255u8, 119u8, 187u8, 170u8,
							247u8, 101u8, 138u8, 167u8, 232u8, 33u8, 116u8, 1u8, 229u8, 171u8, 94u8, 150u8,
							193u8, 51u8, 254u8, 106u8, 44u8, 96u8, 28u8, 88u8,
						],
					)
				}
				#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
				#[doc = "pass `members` pre-sorted."]
				#[doc = ""]
				#[doc = "May only be called from `T::ResetOrigin`."]
				pub fn reset_members(
					&self,
					members: types::reset_members::Members,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ResetMembers> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"OracleMembership",
						"reset_members",
						types::ResetMembers { members },
						[
							212u8, 144u8, 99u8, 156u8, 70u8, 4u8, 219u8, 227u8, 150u8, 25u8, 86u8, 8u8, 215u8,
							128u8, 193u8, 206u8, 33u8, 193u8, 71u8, 15u8, 20u8, 92u8, 99u8, 89u8, 174u8, 236u8,
							102u8, 82u8, 164u8, 234u8, 12u8, 45u8,
						],
					)
				}
				#[doc = "Swap out the sending member for some other key `new`."]
				#[doc = ""]
				#[doc = "May only be called from `Signed` origin of a current member."]
				#[doc = ""]
				#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
				pub fn change_key(
					&self,
					new: types::change_key::New,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ChangeKey> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"OracleMembership",
						"change_key",
						types::ChangeKey { new },
						[
							129u8, 233u8, 205u8, 107u8, 5u8, 50u8, 160u8, 60u8, 161u8, 248u8, 44u8, 53u8, 50u8,
							141u8, 169u8, 36u8, 182u8, 195u8, 173u8, 142u8, 121u8, 153u8, 249u8, 234u8, 253u8,
							64u8, 110u8, 51u8, 207u8, 127u8, 166u8, 108u8,
						],
					)
				}
				#[doc = "Set the prime member. Must be a current member."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn set_prime(
					&self,
					who: types::set_prime::Who,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetPrime> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"OracleMembership",
						"set_prime",
						types::SetPrime { who },
						[
							213u8, 60u8, 220u8, 4u8, 28u8, 111u8, 6u8, 128u8, 228u8, 150u8, 14u8, 182u8, 183u8,
							94u8, 120u8, 238u8, 15u8, 241u8, 107u8, 152u8, 182u8, 33u8, 154u8, 203u8, 172u8,
							217u8, 31u8, 212u8, 112u8, 158u8, 17u8, 188u8,
						],
					)
				}
				#[doc = "Remove the prime member if it exists."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn clear_prime(
					&self,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ClearPrime> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"OracleMembership",
						"clear_prime",
						types::ClearPrime {},
						[
							71u8, 213u8, 34u8, 23u8, 186u8, 63u8, 240u8, 216u8, 190u8, 251u8, 84u8, 109u8, 140u8,
							137u8, 210u8, 211u8, 242u8, 231u8, 212u8, 133u8, 151u8, 125u8, 25u8, 46u8, 210u8,
							53u8, 133u8, 222u8, 21u8, 107u8, 120u8, 52u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The given member was added; see the transaction for who."]
			pub struct MemberAdded;
			impl ::subxt::ext::subxt_core::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "OracleMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The given member was removed; see the transaction for who."]
			pub struct MemberRemoved;
			impl ::subxt::ext::subxt_core::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "OracleMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Two members were swapped; see the transaction for who."]
			pub struct MembersSwapped;
			impl ::subxt::ext::subxt_core::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "OracleMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "The membership was reset; see the transaction for who the new set is."]
			pub struct MembersReset;
			impl ::subxt::ext::subxt_core::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "OracleMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "One of the members' keys changed."]
			pub struct KeyChanged;
			impl ::subxt::ext::subxt_core::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "OracleMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Phantom member, never used."]
			pub struct Dummy;
			impl ::subxt::ext::subxt_core::events::StaticEvent for Dummy {
				const PALLET: &'static str = "OracleMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod members {
					use super::runtime_types;
					pub type Members = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::subxt::ext::subxt_core::utils::AccountId32,
					>;
				}
				pub mod prime {
					use super::runtime_types;
					pub type Prime = ::subxt::ext::subxt_core::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current membership, stored as an ordered Vec."]
				pub fn members(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::members::Members,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"OracleMembership",
						"Members",
						(),
						[
							109u8, 100u8, 14u8, 195u8, 213u8, 67u8, 44u8, 218u8, 84u8, 254u8, 76u8, 80u8, 210u8,
							155u8, 155u8, 30u8, 18u8, 169u8, 195u8, 92u8, 208u8, 223u8, 242u8, 97u8, 147u8, 20u8,
							168u8, 145u8, 254u8, 115u8, 225u8, 193u8,
						],
					)
				}
				#[doc = " The current prime member, if one exists."]
				pub fn prime(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::prime::Prime,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"OracleMembership",
						"Prime",
						(),
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8, 3u8, 103u8,
							14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8, 77u8, 95u8, 116u8, 143u8,
							250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod edge_connect {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum contains all possible errors that can occur when interacting with this pallet."]
		#[doc = "These errors will be returned in the `DispatchResult` when a function call fails."]
		pub type Error = runtime_types::pallet_edge_connect::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_edge_connect::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Registers a Worker with either a domain and initialize it with an inactive status."]
				pub struct RegisterWorker {
					pub domain: register_worker::Domain,
					pub latitude: register_worker::Latitude,
					pub longitude: register_worker::Longitude,
					pub ram: register_worker::Ram,
					pub storage: register_worker::Storage,
					pub cpu: register_worker::Cpu,
				}
				pub mod register_worker {
					use super::runtime_types;
					pub type Domain =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>;
					pub type Latitude = ::core::primitive::i32;
					pub type Longitude = ::core::primitive::i32;
					pub type Ram = ::core::primitive::u64;
					pub type Storage = ::core::primitive::u64;
					pub type Cpu = ::core::primitive::u16;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RegisterWorker {
					const PALLET: &'static str = "EdgeConnect";
					const CALL: &'static str = "register_worker";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Remove a worker from storage an deactivates it"]
				pub struct RemoveWorker {
					pub worker_id: remove_worker::WorkerId,
				}
				pub mod remove_worker {
					use super::runtime_types;
					pub type WorkerId = ::core::primitive::u64;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveWorker {
					const PALLET: &'static str = "EdgeConnect";
					const CALL: &'static str = "remove_worker";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Switches the visibility of a worker between active and inactive."]
				pub struct ToggleWorkerVisibility {
					pub worker_id: toggle_worker_visibility::WorkerId,
					pub visibility: toggle_worker_visibility::Visibility,
				}
				pub mod toggle_worker_visibility {
					use super::runtime_types;
					pub type WorkerId = ::core::primitive::u64;
					pub type Visibility = ::core::primitive::bool;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ToggleWorkerVisibility {
					const PALLET: &'static str = "EdgeConnect";
					const CALL: &'static str = "toggle_worker_visibility";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Registers a Worker with either a domain and initialize it with an inactive status."]
				pub fn register_worker(
					&self,
					domain: types::register_worker::Domain,
					latitude: types::register_worker::Latitude,
					longitude: types::register_worker::Longitude,
					ram: types::register_worker::Ram,
					storage: types::register_worker::Storage,
					cpu: types::register_worker::Cpu,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RegisterWorker> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"EdgeConnect",
						"register_worker",
						types::RegisterWorker {
							domain,
							latitude,
							longitude,
							ram,
							storage,
							cpu,
						},
						[
							222u8, 43u8, 103u8, 250u8, 92u8, 119u8, 239u8, 164u8, 54u8, 31u8, 96u8, 151u8, 40u8,
							116u8, 198u8, 226u8, 250u8, 91u8, 241u8, 147u8, 164u8, 118u8, 113u8, 111u8, 115u8,
							61u8, 131u8, 88u8, 184u8, 126u8, 188u8, 65u8,
						],
					)
				}
				#[doc = "Remove a worker from storage an deactivates it"]
				pub fn remove_worker(
					&self,
					worker_id: types::remove_worker::WorkerId,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveWorker> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"EdgeConnect",
						"remove_worker",
						types::RemoveWorker { worker_id },
						[
							121u8, 152u8, 4u8, 73u8, 56u8, 53u8, 37u8, 90u8, 182u8, 138u8, 164u8, 125u8, 180u8,
							122u8, 61u8, 14u8, 71u8, 61u8, 104u8, 166u8, 235u8, 243u8, 174u8, 197u8, 253u8, 8u8,
							64u8, 74u8, 142u8, 84u8, 7u8, 9u8,
						],
					)
				}
				#[doc = "Switches the visibility of a worker between active and inactive."]
				pub fn toggle_worker_visibility(
					&self,
					worker_id: types::toggle_worker_visibility::WorkerId,
					visibility: types::toggle_worker_visibility::Visibility,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ToggleWorkerVisibility>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"EdgeConnect",
						"toggle_worker_visibility",
						types::ToggleWorkerVisibility {
							worker_id,
							visibility,
						},
						[
							223u8, 254u8, 203u8, 97u8, 96u8, 112u8, 17u8, 196u8, 200u8, 29u8, 84u8, 178u8, 114u8,
							178u8, 200u8, 69u8, 153u8, 99u8, 55u8, 105u8, 209u8, 30u8, 254u8, 145u8, 71u8, 212u8,
							195u8, 128u8, 147u8, 161u8, 59u8, 253u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum contains the various events that can be emitted by this pallet."]
		#[doc = "Events are emitted when significant actions or state changes happen in the pallet."]
		pub type Event = runtime_types::pallet_edge_connect::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event emitted when a new worker is successfully registered."]
			#[doc = ""]
			#[doc = "- `creator`: The account ID of the worker's creator."]
			#[doc = "- `worker`: A tuple containing the account ID of the worker owner and the worker ID."]
			#[doc = "- `domain`: The domain associated with the"]
			pub struct WorkerRegistered {
				pub creator: worker_registered::Creator,
				pub worker: worker_registered::Worker,
				pub domain: worker_registered::Domain,
			}
			pub mod worker_registered {
				use super::runtime_types;
				pub type Creator = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Worker = (
					::subxt::ext::subxt_core::utils::AccountId32,
					::core::primitive::u64,
				);
				pub type Domain =
					runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for WorkerRegistered {
				const PALLET: &'static str = "EdgeConnect";
				const EVENT: &'static str = "WorkerRegistered";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event emitted when a worker is removed from the system."]
			#[doc = ""]
			#[doc = "- `creator`: The account ID of the worker's creator."]
			#[doc = "- `worker_id`: The ID of the worker that was removed."]
			pub struct WorkerRemoved {
				pub creator: worker_removed::Creator,
				pub worker_id: worker_removed::WorkerId,
			}
			pub mod worker_removed {
				use super::runtime_types;
				pub type Creator = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type WorkerId = ::core::primitive::u64;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for WorkerRemoved {
				const PALLET: &'static str = "EdgeConnect";
				const EVENT: &'static str = "WorkerRemoved";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event emitted when a worker's status is updated (e.g., toggling visibility)."]
			#[doc = ""]
			#[doc = "- `creator`: The account ID of the worker's creator."]
			#[doc = "- `worker_id`: The ID of the worker whose status was updated."]
			#[doc = "- `worker_status`: The new status of the worker, either active or inactive."]
			pub struct WorkerStatusUpdated {
				pub creator: worker_status_updated::Creator,
				pub worker_id: worker_status_updated::WorkerId,
				pub worker_status: worker_status_updated::WorkerStatus,
			}
			pub mod worker_status_updated {
				use super::runtime_types;
				pub type Creator = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type WorkerId = ::core::primitive::u64;
				pub type WorkerStatus = runtime_types::cyborg_primitives::worker::WorkerStatusType;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for WorkerStatusUpdated {
				const PALLET: &'static str = "EdgeConnect";
				const EVENT: &'static str = "WorkerStatusUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod account_workers {
					use super::runtime_types;
					pub type AccountWorkers = ::core::primitive::u64;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod worker_clusters {
					use super::runtime_types;
					pub type WorkerClusters = runtime_types::cyborg_primitives::worker::Worker<
						::subxt::ext::subxt_core::utils::AccountId32,
						::core::primitive::u32,
						::core::primitive::u64,
					>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param1 = ::core::primitive::u64;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " AccountWorkers Information, Storage map for associating an account ID with a worker ID. If no worker exists, the query returns None."]
				#[doc = " Keeps track of workerIds per account if any"]
				pub fn account_workers_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::account_workers::AccountWorkers,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"EdgeConnect",
						"AccountWorkers",
						(),
						[
							189u8, 133u8, 255u8, 21u8, 68u8, 183u8, 140u8, 39u8, 192u8, 115u8, 33u8, 246u8, 47u8,
							166u8, 33u8, 125u8, 182u8, 53u8, 117u8, 82u8, 232u8, 6u8, 163u8, 160u8, 173u8, 215u8,
							215u8, 224u8, 183u8, 112u8, 79u8, 10u8,
						],
					)
				}
				#[doc = " AccountWorkers Information, Storage map for associating an account ID with a worker ID. If no worker exists, the query returns None."]
				#[doc = " Keeps track of workerIds per account if any"]
				pub fn account_workers(
					&self,
					_0: impl ::core::borrow::Borrow<types::account_workers::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::account_workers::Param0,
					>,
					types::account_workers::AccountWorkers,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"EdgeConnect",
						"AccountWorkers",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							189u8, 133u8, 255u8, 21u8, 68u8, 183u8, 140u8, 39u8, 192u8, 115u8, 33u8, 246u8, 47u8,
							166u8, 33u8, 125u8, 182u8, 53u8, 117u8, 82u8, 232u8, 6u8, 163u8, 160u8, 173u8, 215u8,
							215u8, 224u8, 183u8, 112u8, 79u8, 10u8,
						],
					)
				}
				#[doc = " Worker Cluster information, Storage map to keep track of detailed worker cluster information for each (account ID, worker ID) pair."]
				pub fn worker_clusters_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::worker_clusters::WorkerClusters,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"EdgeConnect",
						"WorkerClusters",
						(),
						[
							41u8, 165u8, 156u8, 49u8, 93u8, 140u8, 227u8, 98u8, 59u8, 170u8, 147u8, 189u8, 217u8,
							207u8, 237u8, 195u8, 151u8, 143u8, 252u8, 237u8, 10u8, 150u8, 159u8, 157u8, 34u8,
							137u8, 167u8, 242u8, 9u8, 13u8, 192u8, 203u8,
						],
					)
				}
				#[doc = " Worker Cluster information, Storage map to keep track of detailed worker cluster information for each (account ID, worker ID) pair."]
				pub fn worker_clusters_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::worker_clusters::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::worker_clusters::Param0,
					>,
					types::worker_clusters::WorkerClusters,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"EdgeConnect",
						"WorkerClusters",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							41u8, 165u8, 156u8, 49u8, 93u8, 140u8, 227u8, 98u8, 59u8, 170u8, 147u8, 189u8, 217u8,
							207u8, 237u8, 195u8, 151u8, 143u8, 252u8, 237u8, 10u8, 150u8, 159u8, 157u8, 34u8,
							137u8, 167u8, 242u8, 9u8, 13u8, 192u8, 203u8,
						],
					)
				}
				#[doc = " Worker Cluster information, Storage map to keep track of detailed worker cluster information for each (account ID, worker ID) pair."]
				pub fn worker_clusters(
					&self,
					_0: impl ::core::borrow::Borrow<types::worker_clusters::Param0>,
					_1: impl ::core::borrow::Borrow<types::worker_clusters::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::worker_clusters::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::worker_clusters::Param1,
						>,
					),
					types::worker_clusters::WorkerClusters,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"EdgeConnect",
						"WorkerClusters",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							41u8, 165u8, 156u8, 49u8, 93u8, 140u8, 227u8, 98u8, 59u8, 170u8, 147u8, 189u8, 217u8,
							207u8, 237u8, 195u8, 151u8, 143u8, 252u8, 237u8, 10u8, 150u8, 159u8, 157u8, 34u8,
							137u8, 167u8, 242u8, 9u8, 13u8, 192u8, 203u8,
						],
					)
				}
			}
		}
	}
	pub mod task_management {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Errors inform users that something went wrong."]
		#[doc = "<https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>"]
		pub type Error = runtime_types::pallet_task_management::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_task_management::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Creates a new task and assigns it to a randomly selected worker."]
				pub struct TaskScheduler {
					pub task_data: task_scheduler::TaskData,
					pub compute_hours_deposit: task_scheduler::ComputeHoursDeposit,
					pub worker_owner: task_scheduler::WorkerOwner,
					pub worker_id: task_scheduler::WorkerId,
				}
				pub mod task_scheduler {
					use super::runtime_types;
					pub type TaskData =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>;
					pub type ComputeHoursDeposit = ::core::option::Option<::core::primitive::u32>;
					pub type WorkerOwner = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type WorkerId = ::core::primitive::u64;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TaskScheduler {
					const PALLET: &'static str = "TaskManagement";
					const CALL: &'static str = "task_scheduler";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Allows a worker to submit a completed task for verification by a verifier."]
				pub struct SubmitCompletedTask {
					pub task_id: submit_completed_task::TaskId,
					pub completed_hash: submit_completed_task::CompletedHash,
					pub result: submit_completed_task::Result,
				}
				pub mod submit_completed_task {
					use super::runtime_types;
					pub type TaskId = ::core::primitive::u64;
					pub type CompletedHash = ::subxt::ext::subxt_core::utils::H256;
					pub type Result =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SubmitCompletedTask {
					const PALLET: &'static str = "TaskManagement";
					const CALL: &'static str = "submit_completed_task";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Verifies whether the submitted completed task is correct."]
				#[doc = "If verification fails, a new resolver is assigned to review the task."]
				pub struct VerifyCompletedTask {
					pub task_id: verify_completed_task::TaskId,
					pub completed_hash: verify_completed_task::CompletedHash,
				}
				pub mod verify_completed_task {
					use super::runtime_types;
					pub type TaskId = ::core::primitive::u64;
					pub type CompletedHash = ::subxt::ext::subxt_core::utils::H256;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for VerifyCompletedTask {
					const PALLET: &'static str = "TaskManagement";
					const CALL: &'static str = "verify_completed_task";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Resolver finalizes the verification of a task in case of disputes."]
				pub struct ResolveCompletedTask {
					pub task_id: resolve_completed_task::TaskId,
					pub completed_hash: resolve_completed_task::CompletedHash,
				}
				pub mod resolve_completed_task {
					use super::runtime_types;
					pub type TaskId = ::core::primitive::u64;
					pub type CompletedHash = ::subxt::ext::subxt_core::utils::H256;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ResolveCompletedTask {
					const PALLET: &'static str = "TaskManagement";
					const CALL: &'static str = "resolve_completed_task";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Creates a new task and assigns it to a randomly selected worker."]
				pub fn task_scheduler(
					&self,
					task_data: types::task_scheduler::TaskData,
					compute_hours_deposit: types::task_scheduler::ComputeHoursDeposit,
					worker_owner: types::task_scheduler::WorkerOwner,
					worker_id: types::task_scheduler::WorkerId,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TaskScheduler> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"TaskManagement",
						"task_scheduler",
						types::TaskScheduler {
							task_data,
							compute_hours_deposit,
							worker_owner,
							worker_id,
						},
						[
							155u8, 166u8, 99u8, 159u8, 124u8, 131u8, 59u8, 178u8, 243u8, 187u8, 13u8, 36u8,
							202u8, 15u8, 50u8, 51u8, 31u8, 244u8, 212u8, 121u8, 174u8, 21u8, 37u8, 70u8, 201u8,
							253u8, 254u8, 102u8, 111u8, 239u8, 60u8, 116u8,
						],
					)
				}
				#[doc = "Allows a worker to submit a completed task for verification by a verifier."]
				pub fn submit_completed_task(
					&self,
					task_id: types::submit_completed_task::TaskId,
					completed_hash: types::submit_completed_task::CompletedHash,
					result: types::submit_completed_task::Result,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SubmitCompletedTask> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"TaskManagement",
						"submit_completed_task",
						types::SubmitCompletedTask {
							task_id,
							completed_hash,
							result,
						},
						[
							129u8, 95u8, 87u8, 46u8, 216u8, 46u8, 251u8, 143u8, 58u8, 183u8, 96u8, 248u8, 236u8,
							192u8, 252u8, 80u8, 23u8, 247u8, 238u8, 190u8, 150u8, 147u8, 181u8, 240u8, 17u8,
							151u8, 72u8, 30u8, 238u8, 25u8, 19u8, 124u8,
						],
					)
				}
				#[doc = "Verifies whether the submitted completed task is correct."]
				#[doc = "If verification fails, a new resolver is assigned to review the task."]
				pub fn verify_completed_task(
					&self,
					task_id: types::verify_completed_task::TaskId,
					completed_hash: types::verify_completed_task::CompletedHash,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::VerifyCompletedTask> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"TaskManagement",
						"verify_completed_task",
						types::VerifyCompletedTask {
							task_id,
							completed_hash,
						},
						[
							49u8, 141u8, 244u8, 197u8, 171u8, 49u8, 128u8, 166u8, 134u8, 161u8, 75u8, 228u8,
							151u8, 41u8, 22u8, 50u8, 59u8, 59u8, 105u8, 182u8, 161u8, 79u8, 24u8, 105u8, 178u8,
							162u8, 156u8, 71u8, 4u8, 211u8, 63u8, 25u8,
						],
					)
				}
				#[doc = "Resolver finalizes the verification of a task in case of disputes."]
				pub fn resolve_completed_task(
					&self,
					task_id: types::resolve_completed_task::TaskId,
					completed_hash: types::resolve_completed_task::CompletedHash,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ResolveCompletedTask> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"TaskManagement",
						"resolve_completed_task",
						types::ResolveCompletedTask {
							task_id,
							completed_hash,
						},
						[
							223u8, 251u8, 125u8, 162u8, 208u8, 57u8, 73u8, 105u8, 202u8, 103u8, 112u8, 31u8,
							21u8, 244u8, 57u8, 227u8, 190u8, 72u8, 89u8, 108u8, 85u8, 16u8, 104u8, 198u8, 191u8,
							155u8, 23u8, 232u8, 1u8, 109u8, 105u8, 158u8,
						],
					)
				}
			}
		}
		#[doc = "Pallets use events to inform users when important changes are made."]
		#[doc = "<https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>"]
		pub type Event = runtime_types::pallet_task_management::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A task has been scheduled and assigned to a worker."]
			pub struct TaskScheduled {
				pub assigned_worker: task_scheduled::AssignedWorker,
				pub task_owner: task_scheduled::TaskOwner,
				pub task_id: task_scheduled::TaskId,
				pub task: task_scheduled::Task,
			}
			pub mod task_scheduled {
				use super::runtime_types;
				pub type AssignedWorker = (
					::subxt::ext::subxt_core::utils::AccountId32,
					::core::primitive::u64,
				);
				pub type TaskOwner = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type TaskId = ::core::primitive::u64;
				pub type Task =
					runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for TaskScheduled {
				const PALLET: &'static str = "TaskManagement";
				const EVENT: &'static str = "TaskScheduled";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A completed task has been submitted for verification."]
			pub struct SubmittedCompletedTask {
				pub task_id: submitted_completed_task::TaskId,
				pub assigned_verifier: submitted_completed_task::AssignedVerifier,
			}
			pub mod submitted_completed_task {
				use super::runtime_types;
				pub type TaskId = ::core::primitive::u64;
				pub type AssignedVerifier = (
					::subxt::ext::subxt_core::utils::AccountId32,
					::core::primitive::u64,
				);
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for SubmittedCompletedTask {
				const PALLET: &'static str = "TaskManagement";
				const EVENT: &'static str = "SubmittedCompletedTask";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A resolver has been assigned to determine the correct result after verification failure."]
			pub struct VerifierResolverAssigned {
				pub task_id: verifier_resolver_assigned::TaskId,
				pub assigned_resolver: verifier_resolver_assigned::AssignedResolver,
			}
			pub mod verifier_resolver_assigned {
				use super::runtime_types;
				pub type TaskId = ::core::primitive::u64;
				pub type AssignedResolver = (
					::subxt::ext::subxt_core::utils::AccountId32,
					::core::primitive::u64,
				);
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for VerifierResolverAssigned {
				const PALLET: &'static str = "TaskManagement";
				const EVENT: &'static str = "VerifierResolverAssigned";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A completed task has been successfully verified."]
			pub struct VerifiedCompletedTask {
				pub task_id: verified_completed_task::TaskId,
			}
			pub mod verified_completed_task {
				use super::runtime_types;
				pub type TaskId = ::core::primitive::u64;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for VerifiedCompletedTask {
				const PALLET: &'static str = "TaskManagement";
				const EVENT: &'static str = "VerifiedCompletedTask";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A completed task has been successfully resolved by the resolver."]
			pub struct ResolvedCompletedTask {
				pub task_id: resolved_completed_task::TaskId,
			}
			pub mod resolved_completed_task {
				use super::runtime_types;
				pub type TaskId = ::core::primitive::u64;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ResolvedCompletedTask {
				const PALLET: &'static str = "TaskManagement";
				const EVENT: &'static str = "ResolvedCompletedTask";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "A task has been reassigned to a new worker."]
			pub struct TaskReassigned {
				pub task_id: task_reassigned::TaskId,
				pub assigned_executor: task_reassigned::AssignedExecutor,
			}
			pub mod task_reassigned {
				use super::runtime_types;
				pub type TaskId = ::core::primitive::u64;
				pub type AssignedExecutor = (
					::subxt::ext::subxt_core::utils::AccountId32,
					::core::primitive::u64,
				);
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for TaskReassigned {
				const PALLET: &'static str = "TaskManagement";
				const EVENT: &'static str = "TaskReassigned";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod task_status {
					use super::runtime_types;
					pub type TaskStatus = runtime_types::cyborg_primitives::task::TaskStatusType;
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod task_allocations {
					use super::runtime_types;
					pub type TaskAllocations = (
						::subxt::ext::subxt_core::utils::AccountId32,
						::core::primitive::u64,
					);
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod task_owners {
					use super::runtime_types;
					pub type TaskOwners = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod next_task_id {
					use super::runtime_types;
					pub type NextTaskId = ::core::primitive::u64;
				}
				pub mod tasks {
					use super::runtime_types;
					pub type Tasks = runtime_types::cyborg_primitives::task::TaskInfo<
						::subxt::ext::subxt_core::utils::AccountId32,
						::core::primitive::u32,
					>;
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod task_verifications {
					use super::runtime_types;
					pub type TaskVerifications = runtime_types::cyborg_primitives::task::Verifications<
						::subxt::ext::subxt_core::utils::AccountId32,
					>;
					pub type Param0 = ::core::primitive::u64;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Status of tasks within the system."]
				pub fn task_status_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::task_status::TaskStatus,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"TaskStatus",
						(),
						[
							208u8, 58u8, 223u8, 17u8, 86u8, 207u8, 109u8, 247u8, 99u8, 155u8, 42u8, 195u8, 193u8,
							127u8, 84u8, 25u8, 190u8, 16u8, 31u8, 54u8, 16u8, 225u8, 152u8, 62u8, 68u8, 197u8,
							100u8, 226u8, 210u8, 224u8, 147u8, 249u8,
						],
					)
				}
				#[doc = " Status of tasks within the system."]
				pub fn task_status(
					&self,
					_0: impl ::core::borrow::Borrow<types::task_status::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::task_status::Param0>,
					types::task_status::TaskStatus,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"TaskStatus",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							208u8, 58u8, 223u8, 17u8, 86u8, 207u8, 109u8, 247u8, 99u8, 155u8, 42u8, 195u8, 193u8,
							127u8, 84u8, 25u8, 190u8, 16u8, 31u8, 54u8, 16u8, 225u8, 152u8, 62u8, 68u8, 197u8,
							100u8, 226u8, 210u8, 224u8, 147u8, 249u8,
						],
					)
				}
				#[doc = " Allocation of tasks to workers."]
				pub fn task_allocations_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::task_allocations::TaskAllocations,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"TaskAllocations",
						(),
						[
							245u8, 108u8, 202u8, 240u8, 1u8, 7u8, 254u8, 252u8, 205u8, 32u8, 226u8, 108u8, 107u8,
							55u8, 163u8, 188u8, 103u8, 214u8, 223u8, 104u8, 110u8, 216u8, 222u8, 23u8, 127u8,
							140u8, 122u8, 221u8, 250u8, 26u8, 168u8, 236u8,
						],
					)
				}
				#[doc = " Allocation of tasks to workers."]
				pub fn task_allocations(
					&self,
					_0: impl ::core::borrow::Borrow<types::task_allocations::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::task_allocations::Param0,
					>,
					types::task_allocations::TaskAllocations,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"TaskAllocations",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							245u8, 108u8, 202u8, 240u8, 1u8, 7u8, 254u8, 252u8, 205u8, 32u8, 226u8, 108u8, 107u8,
							55u8, 163u8, 188u8, 103u8, 214u8, 223u8, 104u8, 110u8, 216u8, 222u8, 23u8, 127u8,
							140u8, 122u8, 221u8, 250u8, 26u8, 168u8, 236u8,
						],
					)
				}
				#[doc = " Owners of the tasks."]
				pub fn task_owners_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::task_owners::TaskOwners,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"TaskOwners",
						(),
						[
							193u8, 214u8, 88u8, 174u8, 55u8, 53u8, 92u8, 229u8, 149u8, 158u8, 243u8, 60u8, 90u8,
							237u8, 170u8, 23u8, 191u8, 102u8, 220u8, 241u8, 55u8, 155u8, 80u8, 162u8, 190u8,
							30u8, 158u8, 3u8, 71u8, 17u8, 178u8, 136u8,
						],
					)
				}
				#[doc = " Owners of the tasks."]
				pub fn task_owners(
					&self,
					_0: impl ::core::borrow::Borrow<types::task_owners::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::task_owners::Param0>,
					types::task_owners::TaskOwners,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"TaskOwners",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							193u8, 214u8, 88u8, 174u8, 55u8, 53u8, 92u8, 229u8, 149u8, 158u8, 243u8, 60u8, 90u8,
							237u8, 170u8, 23u8, 191u8, 102u8, 220u8, 241u8, 55u8, 155u8, 80u8, 162u8, 190u8,
							30u8, 158u8, 3u8, 71u8, 17u8, 178u8, 136u8,
						],
					)
				}
				#[doc = " The next task ID to be assigned."]
				pub fn next_task_id(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::next_task_id::NextTaskId,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"NextTaskId",
						(),
						[
							247u8, 252u8, 134u8, 80u8, 112u8, 141u8, 174u8, 176u8, 104u8, 211u8, 165u8, 46u8,
							201u8, 209u8, 196u8, 203u8, 133u8, 252u8, 96u8, 29u8, 125u8, 16u8, 181u8, 134u8,
							117u8, 255u8, 30u8, 236u8, 85u8, 219u8, 225u8, 183u8,
						],
					)
				}
				#[doc = " Task metadata and information."]
				pub fn tasks_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::tasks::Tasks,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"Tasks",
						(),
						[
							194u8, 214u8, 104u8, 47u8, 128u8, 165u8, 89u8, 244u8, 159u8, 177u8, 39u8, 215u8,
							119u8, 51u8, 58u8, 51u8, 214u8, 81u8, 137u8, 99u8, 132u8, 101u8, 181u8, 44u8, 115u8,
							29u8, 71u8, 215u8, 122u8, 235u8, 3u8, 12u8,
						],
					)
				}
				#[doc = " Task metadata and information."]
				pub fn tasks(
					&self,
					_0: impl ::core::borrow::Borrow<types::tasks::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<types::tasks::Param0>,
					types::tasks::Tasks,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"Tasks",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							194u8, 214u8, 104u8, 47u8, 128u8, 165u8, 89u8, 244u8, 159u8, 177u8, 39u8, 215u8,
							119u8, 51u8, 58u8, 51u8, 214u8, 81u8, 137u8, 99u8, 132u8, 101u8, 181u8, 44u8, 115u8,
							29u8, 71u8, 215u8, 122u8, 235u8, 3u8, 12u8,
						],
					)
				}
				#[doc = " Private task verifications"]
				pub fn task_verifications_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::task_verifications::TaskVerifications,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"TaskVerifications",
						(),
						[
							9u8, 232u8, 72u8, 220u8, 208u8, 79u8, 171u8, 196u8, 190u8, 85u8, 62u8, 234u8, 187u8,
							88u8, 45u8, 212u8, 220u8, 155u8, 148u8, 201u8, 8u8, 92u8, 163u8, 225u8, 95u8, 117u8,
							212u8, 7u8, 87u8, 192u8, 83u8, 105u8,
						],
					)
				}
				#[doc = " Private task verifications"]
				pub fn task_verifications(
					&self,
					_0: impl ::core::borrow::Borrow<types::task_verifications::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::task_verifications::Param0,
					>,
					types::task_verifications::TaskVerifications,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"TaskManagement",
						"TaskVerifications",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							9u8, 232u8, 72u8, 220u8, 208u8, 79u8, 171u8, 196u8, 190u8, 85u8, 62u8, 234u8, 187u8,
							88u8, 45u8, 212u8, 220u8, 155u8, 148u8, 201u8, 8u8, 92u8, 163u8, 225u8, 95u8, 117u8,
							212u8, 7u8, 87u8, 192u8, 83u8, 105u8,
						],
					)
				}
			}
		}
	}
	pub mod status_aggregator {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Event` enum contains the various events that can be emitted by this pallet."]
		#[doc = "Events are emitted when significant actions or state changes happen in the pallet."]
		pub type Event = runtime_types::pallet_status_aggregator::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event emitted when the worker status is updated based on aggregated data from the oracle."]
			#[doc = "This provides the new online and availability status for the worker and the block number where the status was last updated."]
			#[doc = ""]
			#[doc = "- `worker`: A tuple containing the worker's account ID and the worker ID."]
			#[doc = "- `online`: A boolean indicating whether the worker is online."]
			#[doc = "- `available`: A boolean indicating whether the worker is available."]
			#[doc = "- `last_block_processed`: The block number at which the worker's status was last updated."]
			pub struct UpdateFromAggregatedWorkerInfo {
				pub worker: update_from_aggregated_worker_info::Worker,
				pub online: update_from_aggregated_worker_info::Online,
				pub available: update_from_aggregated_worker_info::Available,
				pub last_block_processed: update_from_aggregated_worker_info::LastBlockProcessed,
			}
			pub mod update_from_aggregated_worker_info {
				use super::runtime_types;
				pub type Worker = (
					::subxt::ext::subxt_core::utils::AccountId32,
					::core::primitive::u64,
				);
				pub type Online = ::core::primitive::bool;
				pub type Available = ::core::primitive::bool;
				pub type LastBlockProcessed = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for UpdateFromAggregatedWorkerInfo {
				const PALLET: &'static str = "StatusAggregator";
				const EVENT: &'static str = "UpdateFromAggregatedWorkerInfo";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event emitted when the last block is updated after clearing data for the current period."]
			#[doc = "This indicates that data from the oracle has been successfully processed and cleared for the given block range."]
			#[doc = ""]
			#[doc = "- `block_number`: The block number at which the clearing occurred."]
			pub struct LastBlockUpdated {
				pub block_number: last_block_updated::BlockNumber,
			}
			pub mod last_block_updated {
				use super::runtime_types;
				pub type BlockNumber = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for LastBlockUpdated {
				const PALLET: &'static str = "StatusAggregator";
				const EVENT: &'static str = "LastBlockUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod last_cleared_block {
					use super::runtime_types;
					pub type LastClearedBlock = ::core::primitive::u32;
				}
				pub mod worker_status_entries_per_period {
					use super::runtime_types;
					pub type WorkerStatusEntriesPerPeriod =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_status_aggregator::StatusInstance<::core::primitive::u32>,
						>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param1 = ::core::primitive::u64;
				}
				pub mod submitted_per_period {
					use super::runtime_types;
					pub type SubmittedPerPeriod = ::core::primitive::bool;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param1 = (
						::subxt::ext::subxt_core::utils::AccountId32,
						::core::primitive::u64,
					);
				}
				pub mod resulting_worker_status_percentages {
					use super::runtime_types;
					pub type ResultingWorkerStatusPercentages =
						runtime_types::pallet_status_aggregator::ProcessStatusPercentages<
							::core::primitive::u32,
						>;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param1 = ::core::primitive::u64;
				}
				pub mod resulting_worker_status {
					use super::runtime_types;
					pub type ResultingWorkerStatus = runtime_types::cyborg_primitives::oracle::ProcessStatus;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
					pub type Param1 = ::core::primitive::u64;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Stores the last block number that the pallet processed for clearing data."]
				#[doc = " This is used to track the last time data was aggregated and cleared by the pallet's hooks."]
				pub fn last_cleared_block(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::last_cleared_block::LastClearedBlock,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"LastClearedBlock",
						(),
						[
							60u8, 122u8, 97u8, 119u8, 236u8, 45u8, 253u8, 27u8, 250u8, 33u8, 184u8, 121u8, 104u8,
							126u8, 113u8, 211u8, 93u8, 53u8, 3u8, 152u8, 85u8, 83u8, 124u8, 210u8, 250u8, 26u8,
							71u8, 177u8, 146u8, 29u8, 21u8, 103u8,
						],
					)
				}
				#[doc = " Stores the status entries (online/offline, available/unavailable) for each worker over a specific period."]
				#[doc = " The status is provided by different oracle feeders, and the data is collected and aggregated to calculate"]
				#[doc = " the overall status for each worker."]
				#[doc = ""]
				#[doc = " - The storage key is a tuple of `(T::AccountId, WorkerId)`, which uniquely identifies the worker."]
				#[doc = " - The value is a bounded vector of `StatusInstance`, which contains the worker's status over time."]
				pub fn worker_status_entries_per_period_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::worker_status_entries_per_period::WorkerStatusEntriesPerPeriod,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"WorkerStatusEntriesPerPeriod",
						(),
						[
							125u8, 35u8, 173u8, 69u8, 254u8, 201u8, 49u8, 53u8, 252u8, 19u8, 132u8, 23u8, 52u8,
							123u8, 6u8, 246u8, 111u8, 191u8, 144u8, 88u8, 55u8, 25u8, 143u8, 113u8, 167u8, 185u8,
							84u8, 165u8, 108u8, 221u8, 228u8, 108u8,
						],
					)
				}
				#[doc = " Stores the status entries (online/offline, available/unavailable) for each worker over a specific period."]
				#[doc = " The status is provided by different oracle feeders, and the data is collected and aggregated to calculate"]
				#[doc = " the overall status for each worker."]
				#[doc = ""]
				#[doc = " - The storage key is a tuple of `(T::AccountId, WorkerId)`, which uniquely identifies the worker."]
				#[doc = " - The value is a bounded vector of `StatusInstance`, which contains the worker's status over time."]
				pub fn worker_status_entries_per_period_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::worker_status_entries_per_period::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::worker_status_entries_per_period::Param0,
					>,
					types::worker_status_entries_per_period::WorkerStatusEntriesPerPeriod,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"WorkerStatusEntriesPerPeriod",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							125u8, 35u8, 173u8, 69u8, 254u8, 201u8, 49u8, 53u8, 252u8, 19u8, 132u8, 23u8, 52u8,
							123u8, 6u8, 246u8, 111u8, 191u8, 144u8, 88u8, 55u8, 25u8, 143u8, 113u8, 167u8, 185u8,
							84u8, 165u8, 108u8, 221u8, 228u8, 108u8,
						],
					)
				}
				#[doc = " Stores the status entries (online/offline, available/unavailable) for each worker over a specific period."]
				#[doc = " The status is provided by different oracle feeders, and the data is collected and aggregated to calculate"]
				#[doc = " the overall status for each worker."]
				#[doc = ""]
				#[doc = " - The storage key is a tuple of `(T::AccountId, WorkerId)`, which uniquely identifies the worker."]
				#[doc = " - The value is a bounded vector of `StatusInstance`, which contains the worker's status over time."]
				pub fn worker_status_entries_per_period(
					&self,
					_0: impl ::core::borrow::Borrow<types::worker_status_entries_per_period::Param0>,
					_1: impl ::core::borrow::Borrow<types::worker_status_entries_per_period::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::worker_status_entries_per_period::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::worker_status_entries_per_period::Param1,
						>,
					),
					types::worker_status_entries_per_period::WorkerStatusEntriesPerPeriod,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"WorkerStatusEntriesPerPeriod",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							125u8, 35u8, 173u8, 69u8, 254u8, 201u8, 49u8, 53u8, 252u8, 19u8, 132u8, 23u8, 52u8,
							123u8, 6u8, 246u8, 111u8, 191u8, 144u8, 88u8, 55u8, 25u8, 143u8, 113u8, 167u8, 185u8,
							84u8, 165u8, 108u8, 221u8, 228u8, 108u8,
						],
					)
				}
				#[doc = " Tracks whether a specific oracle provider has submitted worker status data during the current period."]
				#[doc = " This is used to prevent multiple submissions from the same oracle provider within a period."]
				#[doc = ""]
				#[doc = " - The key is a tuple of the oracle provider's account and the worker `(T::AccountId, (T::AccountId, WorkerId))`."]
				#[doc = " - The value is a boolean indicating whether the oracle has already submitted data."]
				pub fn submitted_per_period_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::submitted_per_period::SubmittedPerPeriod,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"SubmittedPerPeriod",
						(),
						[
							112u8, 129u8, 239u8, 88u8, 175u8, 33u8, 46u8, 167u8, 99u8, 189u8, 181u8, 183u8,
							169u8, 112u8, 252u8, 117u8, 191u8, 238u8, 78u8, 25u8, 210u8, 238u8, 60u8, 45u8,
							216u8, 143u8, 204u8, 181u8, 70u8, 86u8, 167u8, 39u8,
						],
					)
				}
				#[doc = " Tracks whether a specific oracle provider has submitted worker status data during the current period."]
				#[doc = " This is used to prevent multiple submissions from the same oracle provider within a period."]
				#[doc = ""]
				#[doc = " - The key is a tuple of the oracle provider's account and the worker `(T::AccountId, (T::AccountId, WorkerId))`."]
				#[doc = " - The value is a boolean indicating whether the oracle has already submitted data."]
				pub fn submitted_per_period_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::submitted_per_period::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::submitted_per_period::Param0,
					>,
					types::submitted_per_period::SubmittedPerPeriod,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"SubmittedPerPeriod",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							112u8, 129u8, 239u8, 88u8, 175u8, 33u8, 46u8, 167u8, 99u8, 189u8, 181u8, 183u8,
							169u8, 112u8, 252u8, 117u8, 191u8, 238u8, 78u8, 25u8, 210u8, 238u8, 60u8, 45u8,
							216u8, 143u8, 204u8, 181u8, 70u8, 86u8, 167u8, 39u8,
						],
					)
				}
				#[doc = " Tracks whether a specific oracle provider has submitted worker status data during the current period."]
				#[doc = " This is used to prevent multiple submissions from the same oracle provider within a period."]
				#[doc = ""]
				#[doc = " - The key is a tuple of the oracle provider's account and the worker `(T::AccountId, (T::AccountId, WorkerId))`."]
				#[doc = " - The value is a boolean indicating whether the oracle has already submitted data."]
				pub fn submitted_per_period(
					&self,
					_0: impl ::core::borrow::Borrow<types::submitted_per_period::Param0>,
					_1: impl ::core::borrow::Borrow<types::submitted_per_period::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::submitted_per_period::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::submitted_per_period::Param1,
						>,
					),
					types::submitted_per_period::SubmittedPerPeriod,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"SubmittedPerPeriod",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							112u8, 129u8, 239u8, 88u8, 175u8, 33u8, 46u8, 167u8, 99u8, 189u8, 181u8, 183u8,
							169u8, 112u8, 252u8, 117u8, 191u8, 238u8, 78u8, 25u8, 210u8, 238u8, 60u8, 45u8,
							216u8, 143u8, 204u8, 181u8, 70u8, 86u8, 167u8, 39u8,
						],
					)
				}
				#[doc = " Stores the resulting percentage status (online and available) for each worker after aggregation."]
				#[doc = " This is calculated by taking the status data submitted during the period and determining the"]
				#[doc = " percentage of time the worker was online and available."]
				#[doc = ""]
				#[doc = " - The key is `(T::AccountId, WorkerId)`, representing the worker."]
				#[doc = " - The value is `ProcessStatusPercentages`, which contains the percentages and the block number of the last processed status."]
				pub fn resulting_worker_status_percentages_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::resulting_worker_status_percentages::ResultingWorkerStatusPercentages,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"ResultingWorkerStatusPercentages",
						(),
						[
							189u8, 12u8, 214u8, 195u8, 105u8, 164u8, 124u8, 31u8, 50u8, 190u8, 90u8, 95u8, 15u8,
							103u8, 34u8, 17u8, 11u8, 196u8, 169u8, 207u8, 39u8, 94u8, 165u8, 199u8, 129u8, 14u8,
							183u8, 137u8, 235u8, 0u8, 233u8, 61u8,
						],
					)
				}
				#[doc = " Stores the resulting percentage status (online and available) for each worker after aggregation."]
				#[doc = " This is calculated by taking the status data submitted during the period and determining the"]
				#[doc = " percentage of time the worker was online and available."]
				#[doc = ""]
				#[doc = " - The key is `(T::AccountId, WorkerId)`, representing the worker."]
				#[doc = " - The value is `ProcessStatusPercentages`, which contains the percentages and the block number of the last processed status."]
				pub fn resulting_worker_status_percentages_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::resulting_worker_status_percentages::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::resulting_worker_status_percentages::Param0,
					>,
					types::resulting_worker_status_percentages::ResultingWorkerStatusPercentages,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"ResultingWorkerStatusPercentages",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							189u8, 12u8, 214u8, 195u8, 105u8, 164u8, 124u8, 31u8, 50u8, 190u8, 90u8, 95u8, 15u8,
							103u8, 34u8, 17u8, 11u8, 196u8, 169u8, 207u8, 39u8, 94u8, 165u8, 199u8, 129u8, 14u8,
							183u8, 137u8, 235u8, 0u8, 233u8, 61u8,
						],
					)
				}
				#[doc = " Stores the resulting percentage status (online and available) for each worker after aggregation."]
				#[doc = " This is calculated by taking the status data submitted during the period and determining the"]
				#[doc = " percentage of time the worker was online and available."]
				#[doc = ""]
				#[doc = " - The key is `(T::AccountId, WorkerId)`, representing the worker."]
				#[doc = " - The value is `ProcessStatusPercentages`, which contains the percentages and the block number of the last processed status."]
				pub fn resulting_worker_status_percentages(
					&self,
					_0: impl ::core::borrow::Borrow<types::resulting_worker_status_percentages::Param0>,
					_1: impl ::core::borrow::Borrow<types::resulting_worker_status_percentages::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::resulting_worker_status_percentages::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::resulting_worker_status_percentages::Param1,
						>,
					),
					types::resulting_worker_status_percentages::ResultingWorkerStatusPercentages,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"ResultingWorkerStatusPercentages",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							189u8, 12u8, 214u8, 195u8, 105u8, 164u8, 124u8, 31u8, 50u8, 190u8, 90u8, 95u8, 15u8,
							103u8, 34u8, 17u8, 11u8, 196u8, 169u8, 207u8, 39u8, 94u8, 165u8, 199u8, 129u8, 14u8,
							183u8, 137u8, 235u8, 0u8, 233u8, 61u8,
						],
					)
				}
				#[doc = " Stores the final status (online/offline and available/unavailable) for each worker based on the percentage thresholds."]
				#[doc = " The final status is determined based on the configured threshold values for uptime."]
				#[doc = ""]
				#[doc = " - The key is `(T::AccountId, WorkerId)`, representing the worker."]
				#[doc = " - The value is `ProcessStatus`, which contains the final online and available status for the worker."]
				pub fn resulting_worker_status_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::resulting_worker_status::ResultingWorkerStatus,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"ResultingWorkerStatus",
						(),
						[
							10u8, 199u8, 157u8, 48u8, 141u8, 103u8, 65u8, 126u8, 171u8, 34u8, 222u8, 63u8, 192u8,
							188u8, 154u8, 56u8, 4u8, 194u8, 215u8, 224u8, 214u8, 234u8, 45u8, 2u8, 139u8, 39u8,
							58u8, 6u8, 160u8, 248u8, 112u8, 204u8,
						],
					)
				}
				#[doc = " Stores the final status (online/offline and available/unavailable) for each worker based on the percentage thresholds."]
				#[doc = " The final status is determined based on the configured threshold values for uptime."]
				#[doc = ""]
				#[doc = " - The key is `(T::AccountId, WorkerId)`, representing the worker."]
				#[doc = " - The value is `ProcessStatus`, which contains the final online and available status for the worker."]
				pub fn resulting_worker_status_iter1(
					&self,
					_0: impl ::core::borrow::Borrow<types::resulting_worker_status::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::resulting_worker_status::Param0,
					>,
					types::resulting_worker_status::ResultingWorkerStatus,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"ResultingWorkerStatus",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							10u8, 199u8, 157u8, 48u8, 141u8, 103u8, 65u8, 126u8, 171u8, 34u8, 222u8, 63u8, 192u8,
							188u8, 154u8, 56u8, 4u8, 194u8, 215u8, 224u8, 214u8, 234u8, 45u8, 2u8, 139u8, 39u8,
							58u8, 6u8, 160u8, 248u8, 112u8, 204u8,
						],
					)
				}
				#[doc = " Stores the final status (online/offline and available/unavailable) for each worker based on the percentage thresholds."]
				#[doc = " The final status is determined based on the configured threshold values for uptime."]
				#[doc = ""]
				#[doc = " - The key is `(T::AccountId, WorkerId)`, representing the worker."]
				#[doc = " - The value is `ProcessStatus`, which contains the final online and available status for the worker."]
				pub fn resulting_worker_status(
					&self,
					_0: impl ::core::borrow::Borrow<types::resulting_worker_status::Param0>,
					_1: impl ::core::borrow::Borrow<types::resulting_worker_status::Param1>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::resulting_worker_status::Param0,
						>,
						::subxt::ext::subxt_core::storage::address::StaticStorageKey<
							types::resulting_worker_status::Param1,
						>,
					),
					types::resulting_worker_status::ResultingWorkerStatus,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"StatusAggregator",
						"ResultingWorkerStatus",
						(
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							10u8, 199u8, 157u8, 48u8, 141u8, 103u8, 65u8, 126u8, 171u8, 34u8, 222u8, 63u8, 192u8,
							188u8, 154u8, 56u8, 4u8, 194u8, 215u8, 224u8, 214u8, 234u8, 45u8, 2u8, 139u8, 39u8,
							58u8, 6u8, 160u8, 248u8, 112u8, 204u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Maximum number of blocks or block range used to calculate average status"]
				pub fn max_block_range_period(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"StatusAggregator",
						"MaxBlockRangePeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The percentage of active oracle entries needed to determine online status for worker"]
				pub fn threshold_uptime_status(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u8>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"StatusAggregator",
						"ThresholdUptimeStatus",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8, 28u8, 91u8, 221u8,
							64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8, 114u8, 97u8, 79u8, 62u8, 212u8,
							202u8, 114u8, 237u8, 228u8, 183u8, 165u8,
						],
					)
				}
				#[doc = " Maximum number of status entries by unique oracle feeders for a worker per period"]
				pub fn max_aggregate_param_length(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"StatusAggregator",
						"MaxAggregateParamLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
			}
		}
	}
	pub mod payment {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Pallet Errors"]
		pub type Error = runtime_types::pallet_payment::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_payment::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Allows the admin (root) to set the price per compute hour."]
				pub struct SetPricePerHour {
					pub new_price: set_price_per_hour::NewPrice,
				}
				pub mod set_price_per_hour {
					use super::runtime_types;
					pub type NewPrice = ::core::primitive::u128;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetPricePerHour {
					const PALLET: &'static str = "Payment";
					const CALL: &'static str = "set_price_per_hour";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Allows the admin (root) to set the service provider's account."]
				pub struct SetServiceProviderAccount {
					pub new_account: set_service_provider_account::NewAccount,
				}
				pub mod set_service_provider_account {
					use super::runtime_types;
					pub type NewAccount = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetServiceProviderAccount {
					const PALLET: &'static str = "Payment";
					const CALL: &'static str = "set_service_provider_account";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Allows a user to purchase compute hours."]
				pub struct PurchaseComputeHours {
					pub hours: purchase_compute_hours::Hours,
				}
				pub mod purchase_compute_hours {
					use super::runtime_types;
					pub type Hours = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for PurchaseComputeHours {
					const PALLET: &'static str = "Payment";
					const CALL: &'static str = "purchase_compute_hours";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Allows a user to consume compute hours."]
				pub struct ConsumeComputeHours {
					pub hours: consume_compute_hours::Hours,
				}
				pub mod consume_compute_hours {
					use super::runtime_types;
					pub type Hours = ::core::primitive::u32;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ConsumeComputeHours {
					const PALLET: &'static str = "Payment";
					const CALL: &'static str = "consume_compute_hours";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Allows the admin (root) to set the price per compute hour."]
				pub fn set_price_per_hour(
					&self,
					new_price: types::set_price_per_hour::NewPrice,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetPricePerHour> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Payment",
						"set_price_per_hour",
						types::SetPricePerHour { new_price },
						[
							154u8, 194u8, 241u8, 49u8, 133u8, 115u8, 113u8, 146u8, 134u8, 73u8, 63u8, 108u8,
							230u8, 186u8, 19u8, 165u8, 215u8, 150u8, 46u8, 110u8, 171u8, 162u8, 31u8, 196u8,
							185u8, 27u8, 137u8, 195u8, 106u8, 67u8, 246u8, 38u8,
						],
					)
				}
				#[doc = "Allows the admin (root) to set the service provider's account."]
				pub fn set_service_provider_account(
					&self,
					new_account: types::set_service_provider_account::NewAccount,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetServiceProviderAccount>
				{
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Payment",
						"set_service_provider_account",
						types::SetServiceProviderAccount { new_account },
						[
							24u8, 86u8, 140u8, 39u8, 234u8, 118u8, 6u8, 98u8, 41u8, 11u8, 125u8, 166u8, 118u8,
							252u8, 214u8, 114u8, 185u8, 229u8, 242u8, 158u8, 27u8, 86u8, 114u8, 146u8, 59u8,
							238u8, 3u8, 25u8, 35u8, 125u8, 27u8, 88u8,
						],
					)
				}
				#[doc = "Allows a user to purchase compute hours."]
				pub fn purchase_compute_hours(
					&self,
					hours: types::purchase_compute_hours::Hours,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::PurchaseComputeHours> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Payment",
						"purchase_compute_hours",
						types::PurchaseComputeHours { hours },
						[
							213u8, 167u8, 114u8, 169u8, 206u8, 244u8, 199u8, 185u8, 109u8, 23u8, 162u8, 86u8,
							87u8, 75u8, 244u8, 144u8, 127u8, 141u8, 160u8, 125u8, 117u8, 176u8, 30u8, 115u8,
							134u8, 119u8, 203u8, 68u8, 181u8, 49u8, 108u8, 63u8,
						],
					)
				}
				#[doc = "Allows a user to consume compute hours."]
				pub fn consume_compute_hours(
					&self,
					hours: types::consume_compute_hours::Hours,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ConsumeComputeHours> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"Payment",
						"consume_compute_hours",
						types::ConsumeComputeHours { hours },
						[
							151u8, 107u8, 100u8, 231u8, 175u8, 150u8, 36u8, 168u8, 249u8, 168u8, 145u8, 141u8,
							204u8, 183u8, 177u8, 153u8, 79u8, 233u8, 25u8, 112u8, 103u8, 117u8, 232u8, 142u8,
							231u8, 45u8, 0u8, 121u8, 120u8, 78u8, 235u8, 110u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event triggered when compute hours are consumed."]
			pub struct HoursConsumed(pub hours_consumed::Field0, pub hours_consumed::Field1);
			pub mod hours_consumed {
				use super::runtime_types;
				pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Field1 = ::core::primitive::u32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for HoursConsumed {
				const PALLET: &'static str = "Payment";
				const EVENT: &'static str = "HoursConsumed";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event triggered when compute hours are purchased."]
			pub struct HoursPurchased(
				pub hours_purchased::Field0,
				pub hours_purchased::Field1,
				pub hours_purchased::Field2,
			);
			pub mod hours_purchased {
				use super::runtime_types;
				pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
				pub type Field1 = ::core::primitive::u32;
				pub type Field2 = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for HoursPurchased {
				const PALLET: &'static str = "Payment";
				const EVENT: &'static str = "HoursPurchased";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event triggered when the admin sets a new price per hour."]
			pub struct PricePerHourSet(pub price_per_hour_set::Field0);
			pub mod price_per_hour_set {
				use super::runtime_types;
				pub type Field0 = ::core::primitive::u128;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for PricePerHourSet {
				const PALLET: &'static str = "Payment";
				const EVENT: &'static str = "PricePerHourSet";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			#[doc = "Event triggered when the admin sets a new service provider account."]
			pub struct ServiceProviderAccountSet(pub service_provider_account_set::Field0);
			pub mod service_provider_account_set {
				use super::runtime_types;
				pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for ServiceProviderAccountSet {
				const PALLET: &'static str = "Payment";
				const EVENT: &'static str = "ServiceProviderAccountSet";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod compute_hours {
					use super::runtime_types;
					pub type ComputeHours = ::core::primitive::u32;
					pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
				}
				pub mod price_per_hour {
					use super::runtime_types;
					pub type PricePerHour = ::core::primitive::u128;
				}
				pub mod service_provider_account {
					use super::runtime_types;
					pub type ServiceProviderAccount = ::subxt::ext::subxt_core::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn compute_hours_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::compute_hours::ComputeHours,
					(),
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Payment",
						"ComputeHours",
						(),
						[
							173u8, 228u8, 141u8, 96u8, 119u8, 57u8, 146u8, 50u8, 123u8, 69u8, 123u8, 206u8, 38u8,
							37u8, 243u8, 255u8, 157u8, 127u8, 140u8, 63u8, 199u8, 77u8, 134u8, 237u8, 218u8,
							159u8, 39u8, 1u8, 29u8, 175u8, 64u8, 7u8,
						],
					)
				}
				pub fn compute_hours(
					&self,
					_0: impl ::core::borrow::Borrow<types::compute_hours::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::compute_hours::Param0,
					>,
					types::compute_hours::ComputeHours,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Payment",
						"ComputeHours",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							173u8, 228u8, 141u8, 96u8, 119u8, 57u8, 146u8, 50u8, 123u8, 69u8, 123u8, 206u8, 38u8,
							37u8, 243u8, 255u8, 157u8, 127u8, 140u8, 63u8, 199u8, 77u8, 134u8, 237u8, 218u8,
							159u8, 39u8, 1u8, 29u8, 175u8, 64u8, 7u8,
						],
					)
				}
				pub fn price_per_hour(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::price_per_hour::PricePerHour,
					::subxt::ext::subxt_core::utils::Yes,
					::subxt::ext::subxt_core::utils::Yes,
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Payment",
						"PricePerHour",
						(),
						[
							198u8, 231u8, 132u8, 102u8, 163u8, 193u8, 105u8, 211u8, 85u8, 39u8, 192u8, 63u8,
							206u8, 152u8, 170u8, 120u8, 92u8, 222u8, 5u8, 55u8, 35u8, 70u8, 127u8, 191u8, 44u8,
							237u8, 204u8, 251u8, 228u8, 68u8, 133u8, 180u8,
						],
					)
				}
				pub fn service_provider_account(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::service_provider_account::ServiceProviderAccount,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"Payment",
						"ServiceProviderAccount",
						(),
						[
							253u8, 46u8, 234u8, 103u8, 73u8, 90u8, 132u8, 23u8, 19u8, 30u8, 228u8, 188u8, 27u8,
							171u8, 93u8, 55u8, 249u8, 164u8, 34u8, 219u8, 197u8, 188u8, 216u8, 30u8, 15u8, 111u8,
							119u8, 62u8, 20u8, 44u8, 249u8, 193u8,
						],
					)
				}
			}
		}
	}
	pub mod zk_verifier {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_zk_verifier::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_zk_verifier::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Store a verification key."]
				pub struct SetupVerification {
					pub task_id: setup_verification::TaskId,
					pub pub_input: setup_verification::PubInput,
					pub vec_vk: setup_verification::VecVk,
				}
				pub mod setup_verification {
					use super::runtime_types;
					pub type TaskId = ::core::primitive::u64;
					pub type PubInput = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
					pub type VecVk = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetupVerification {
					const PALLET: &'static str = "ZKVerifier";
					const CALL: &'static str = "setup_verification";
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Verify a proof."]
				pub struct Verify {
					pub task_id: verify::TaskId,
					pub vec_proof: verify::VecProof,
				}
				pub mod verify {
					use super::runtime_types;
					pub type TaskId = ::core::primitive::u64;
					pub type VecProof = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Verify {
					const PALLET: &'static str = "ZKVerifier";
					const CALL: &'static str = "verify";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Store a verification key."]
				pub fn setup_verification(
					&self,
					task_id: types::setup_verification::TaskId,
					pub_input: types::setup_verification::PubInput,
					vec_vk: types::setup_verification::VecVk,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetupVerification> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"ZKVerifier",
						"setup_verification",
						types::SetupVerification {
							task_id,
							pub_input,
							vec_vk,
						},
						[
							177u8, 72u8, 115u8, 185u8, 50u8, 165u8, 218u8, 8u8, 159u8, 96u8, 59u8, 158u8, 129u8,
							196u8, 31u8, 230u8, 81u8, 199u8, 159u8, 65u8, 236u8, 72u8, 45u8, 102u8, 36u8, 73u8,
							121u8, 96u8, 36u8, 51u8, 180u8, 35u8,
						],
					)
				}
				#[doc = "Verify a proof."]
				pub fn verify(
					&self,
					task_id: types::verify::TaskId,
					vec_proof: types::verify::VecProof,
				) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Verify> {
					::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
						"ZKVerifier",
						"verify",
						types::Verify { task_id, vec_proof },
						[
							146u8, 41u8, 132u8, 126u8, 27u8, 213u8, 107u8, 110u8, 104u8, 73u8, 138u8, 70u8,
							212u8, 11u8, 29u8, 97u8, 64u8, 96u8, 113u8, 3u8, 23u8, 95u8, 160u8, 26u8, 56u8, 11u8,
							67u8, 132u8, 205u8, 239u8, 96u8, 25u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_zk_verifier::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct VerificationSetupCompleted;
			impl ::subxt::ext::subxt_core::events::StaticEvent for VerificationSetupCompleted {
				const PALLET: &'static str = "ZKVerifier";
				const EVENT: &'static str = "VerificationSetupCompleted";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct VerificationProofSet;
			impl ::subxt::ext::subxt_core::events::StaticEvent for VerificationProofSet {
				const PALLET: &'static str = "ZKVerifier";
				const EVENT: &'static str = "VerificationProofSet";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct VerificationSuccess {
				pub who: verification_success::Who,
			}
			pub mod verification_success {
				use super::runtime_types;
				pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
			}
			impl ::subxt::ext::subxt_core::events::StaticEvent for VerificationSuccess {
				const PALLET: &'static str = "ZKVerifier";
				const EVENT: &'static str = "VerificationSuccess";
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct VerificationFailed;
			impl ::subxt::ext::subxt_core::events::StaticEvent for VerificationFailed {
				const PALLET: &'static str = "ZKVerifier";
				const EVENT: &'static str = "VerificationFailed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod public_input_storage {
					use super::runtime_types;
					pub type PublicInputStorage =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>;
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod proof_storage {
					use super::runtime_types;
					pub type ProofStorage =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>;
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod verification_key_storage {
					use super::runtime_types;
					pub type VerificationKeyStorage =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>;
					pub type Param0 = ::core::primitive::u64;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Storing a public input."]
				pub fn public_input_storage_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::public_input_storage::PublicInputStorage,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ZKVerifier",
						"PublicInputStorage",
						(),
						[
							249u8, 4u8, 35u8, 14u8, 165u8, 80u8, 102u8, 240u8, 80u8, 149u8, 141u8, 143u8, 38u8,
							181u8, 39u8, 36u8, 171u8, 156u8, 209u8, 244u8, 108u8, 69u8, 185u8, 119u8, 109u8,
							111u8, 97u8, 153u8, 5u8, 249u8, 2u8, 63u8,
						],
					)
				}
				#[doc = " Storing a public input."]
				pub fn public_input_storage(
					&self,
					_0: impl ::core::borrow::Borrow<types::public_input_storage::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::public_input_storage::Param0,
					>,
					types::public_input_storage::PublicInputStorage,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ZKVerifier",
						"PublicInputStorage",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							249u8, 4u8, 35u8, 14u8, 165u8, 80u8, 102u8, 240u8, 80u8, 149u8, 141u8, 143u8, 38u8,
							181u8, 39u8, 36u8, 171u8, 156u8, 209u8, 244u8, 108u8, 69u8, 185u8, 119u8, 109u8,
							111u8, 97u8, 153u8, 5u8, 249u8, 2u8, 63u8,
						],
					)
				}
				#[doc = " Storing a proof."]
				pub fn proof_storage_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::proof_storage::ProofStorage,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ZKVerifier",
						"ProofStorage",
						(),
						[
							74u8, 59u8, 210u8, 179u8, 82u8, 140u8, 173u8, 39u8, 1u8, 238u8, 39u8, 31u8, 39u8,
							89u8, 58u8, 232u8, 191u8, 41u8, 184u8, 100u8, 125u8, 109u8, 252u8, 129u8, 128u8,
							251u8, 12u8, 172u8, 100u8, 159u8, 124u8, 179u8,
						],
					)
				}
				#[doc = " Storing a proof."]
				pub fn proof_storage(
					&self,
					_0: impl ::core::borrow::Borrow<types::proof_storage::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::proof_storage::Param0,
					>,
					types::proof_storage::ProofStorage,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ZKVerifier",
						"ProofStorage",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							74u8, 59u8, 210u8, 179u8, 82u8, 140u8, 173u8, 39u8, 1u8, 238u8, 39u8, 31u8, 39u8,
							89u8, 58u8, 232u8, 191u8, 41u8, 184u8, 100u8, 125u8, 109u8, 252u8, 129u8, 128u8,
							251u8, 12u8, 172u8, 100u8, 159u8, 124u8, 179u8,
						],
					)
				}
				#[doc = " Storing a verification key."]
				pub fn verification_key_storage_iter(
					&self,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					(),
					types::verification_key_storage::VerificationKeyStorage,
					(),
					(),
					::subxt::ext::subxt_core::utils::Yes,
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ZKVerifier",
						"VerificationKeyStorage",
						(),
						[
							165u8, 213u8, 217u8, 59u8, 220u8, 114u8, 59u8, 94u8, 35u8, 150u8, 51u8, 15u8, 230u8,
							85u8, 96u8, 109u8, 196u8, 249u8, 12u8, 28u8, 240u8, 112u8, 178u8, 244u8, 215u8,
							159u8, 225u8, 142u8, 131u8, 193u8, 37u8, 170u8,
						],
					)
				}
				#[doc = " Storing a verification key."]
				pub fn verification_key_storage(
					&self,
					_0: impl ::core::borrow::Borrow<types::verification_key_storage::Param0>,
				) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
					::subxt::ext::subxt_core::storage::address::StaticStorageKey<
						types::verification_key_storage::Param0,
					>,
					types::verification_key_storage::VerificationKeyStorage,
					::subxt::ext::subxt_core::utils::Yes,
					(),
					(),
				> {
					::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
						"ZKVerifier",
						"VerificationKeyStorage",
						::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							165u8, 213u8, 217u8, 59u8, 220u8, 114u8, 59u8, 94u8, 35u8, 150u8, 51u8, 15u8, 230u8,
							85u8, 96u8, 109u8, 196u8, 249u8, 12u8, 28u8, 240u8, 112u8, 178u8, 244u8, 215u8,
							159u8, 225u8, 142u8, 131u8, 193u8, 37u8, 170u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn max_public_inputs_length(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"ZKVerifier",
						"MaxPublicInputsLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The maximum length of the proof."]
				pub fn max_proof_length(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"ZKVerifier",
						"MaxProofLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
				#[doc = " The maximum length of the verification key."]
				pub fn max_verification_key_length(
					&self,
				) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
				{
					::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
						"ZKVerifier",
						"MaxVerificationKeyLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8,
							76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8,
							200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
						],
					)
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_btree_set {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct BoundedBTreeSet<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
			}
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct BoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct WeakBoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
			}
		}
		pub mod cumulus_pallet_parachain_system {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the current validation data."]
					#[doc = ""]
					#[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
					#[doc = "phase if the call was not invoked."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Inherent`"]
					#[doc = ""]
					#[doc = "As a side effect, this function upgrades the current validation function"]
					#[doc = "if the appropriate time has come."]
					set_validation_data {
						data: runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
					},
					#[codec(index = 1)]
					sudo_send_upward_message {
						message: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
					#[doc = "later."]
					#[doc = ""]
					#[doc = "The `check_version` parameter sets a boolean flag for whether or not the runtime's spec"]
					#[doc = "version and name should be verified on upgrade. Since the authorization only has a hash,"]
					#[doc = "it cannot actually perform the verification."]
					#[doc = ""]
					#[doc = "This call requires Root origin."]
					authorize_upgrade {
						code_hash: ::subxt::ext::subxt_core::utils::H256,
						check_version: ::core::primitive::bool,
					},
					#[codec(index = 3)]
					#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
					#[doc = ""]
					#[doc = "If the authorization required a version check, this call will ensure the spec name"]
					#[doc = "remains unchanged and that the spec version has increased."]
					#[doc = ""]
					#[doc = "Note that this function will not apply the new `code`, but only attempt to schedule the"]
					#[doc = "upgrade with the Relay Chain."]
					#[doc = ""]
					#[doc = "All origins are allowed."]
					enact_authorized_upgrade {
						code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to upgrade validation function while existing upgrade pending."]
					OverlappingUpgrades,
					#[codec(index = 1)]
					#[doc = "Polkadot currently prohibits this parachain from upgrading its validation function."]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					#[doc = "The supplied validation function has compiled into a blob larger than Polkadot is"]
					#[doc = "willing to run."]
					TooBig,
					#[codec(index = 3)]
					#[doc = "The inherent which supplies the validation data did not run this block."]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					#[doc = "The inherent which supplies the host configuration did not run this block."]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					#[doc = "No validation function upgrade is currently scheduled."]
					NotScheduled,
					#[codec(index = 6)]
					#[doc = "No code upgrade has been authorized."]
					NothingAuthorized,
					#[codec(index = 7)]
					#[doc = "The given code upgrade has not been authorized."]
					Unauthorized,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The validation function has been scheduled to apply."]
					ValidationFunctionStored,
					#[codec(index = 1)]
					#[doc = "The validation function was applied as of the contained relay chain block number."]
					ValidationFunctionApplied {
						relay_chain_block_num: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "The relay-chain aborted the upgrade process."]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					#[doc = "Some downward messages have been received and will be processed."]
					DownwardMessagesReceived { count: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Downward messages were processed using the given weight."]
					DownwardMessagesProcessed {
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						dmq_head: ::subxt::ext::subxt_core::utils::H256,
					},
					#[codec(index = 5)]
					#[doc = "An upward message was sent to the relay chain."]
					UpwardMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct MessagingStateSnapshot { pub dmq_mqc_head : :: subxt :: ext :: subxt_core :: utils :: H256 , pub relay_dispatch_queue_remaining_capacity : runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: RelayDispatchQueueRemainingCapacity , pub ingress_channels : :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < (runtime_types :: polkadot_parachain_primitives :: primitives :: Id , runtime_types :: polkadot_primitives :: v7 :: AbridgedHrmpChannel ,) > , pub egress_channels : :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < (runtime_types :: polkadot_parachain_primitives :: primitives :: Id , runtime_types :: polkadot_primitives :: v7 :: AbridgedHrmpChannel ,) > , }
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct RelayDispatchQueueRemainingCapacity {
					pub remaining_count: ::core::primitive::u32,
					pub remaining_size: ::core::primitive::u32,
				}
			}
			pub mod unincluded_segment {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Ancestor<_0> {
					pub used_bandwidth:
						runtime_types::cumulus_pallet_parachain_system::unincluded_segment::UsedBandwidth,
					pub para_head_hash: ::core::option::Option<_0>,
					pub consumed_go_ahead_signal:
						::core::option::Option<runtime_types::polkadot_primitives::v7::UpgradeGoAhead>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct HrmpChannelUpdate {
					pub msg_count: ::core::primitive::u32,
					pub total_bytes: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct SegmentTracker<_0> {
					pub used_bandwidth:
						runtime_types::cumulus_pallet_parachain_system::unincluded_segment::UsedBandwidth,
					pub hrmp_watermark: ::core::option::Option<::core::primitive::u32>,
					pub consumed_go_ahead_signal:
						::core::option::Option<runtime_types::polkadot_primitives::v7::UpgradeGoAhead>,
					#[codec(skip)]
					pub __ignore: ::core::marker::PhantomData<_0>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct UsedBandwidth {
					pub ump_msg_count: ::core::primitive::u32,
					pub ump_total_bytes: ::core::primitive::u32,
					pub hrmp_outgoing: ::subxt::ext::subxt_core::utils::KeyedVec<
						runtime_types::polkadot_parachain_primitives::primitives::Id,
						runtime_types::cumulus_pallet_parachain_system::unincluded_segment::HrmpChannelUpdate,
					>,
				}
			}
		}
		pub mod cumulus_pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Downward message is invalid XCM."]
					#[doc = "\\[ id \\]"]
					InvalidFormat([::core::primitive::u8; 32usize]),
					#[codec(index = 1)]
					#[doc = "Downward message is unsupported version of XCM."]
					#[doc = "\\[ id \\]"]
					UnsupportedVersion([::core::primitive::u8; 32usize]),
					#[codec(index = 2)]
					#[doc = "Downward message executed with the given outcome."]
					#[doc = "\\[ id, outcome \\]"]
					ExecutedDownward(
						[::core::primitive::u8; 32usize],
						runtime_types::staging_xcm::v4::traits::Outcome,
					),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 1)]
					#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ControllerOrigin`."]
					suspend_xcm_execution,
					#[codec(index = 2)]
					#[doc = "Resumes all XCM executions for the XCMP queue."]
					#[doc = ""]
					#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ControllerOrigin`."]
					resume_xcm_execution,
					#[codec(index = 3)]
					#[doc = "Overwrites the number of pages which must be in the queue for the other side to be"]
					#[doc = "told to suspend their sending."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
					update_suspend_threshold { new: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Overwrites the number of pages which must be in the queue after which we drop any"]
					#[doc = "further messages from the channel."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
					update_drop_threshold { new: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "Overwrites the number of pages which the queue must be reduced to before it signals"]
					#[doc = "that message sending may recommence after it has been suspended."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
					update_resume_threshold { new: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Setting the queue config failed since one of its values was invalid."]
					BadQueueConfig,
					#[codec(index = 1)]
					#[doc = "The execution is already suspended."]
					AlreadySuspended,
					#[codec(index = 2)]
					#[doc = "The execution is already resumed."]
					AlreadyResumed,
					#[codec(index = 3)]
					#[doc = "There are too many active outbound channels."]
					TooManyActiveOutboundChannels,
					#[codec(index = 4)]
					#[doc = "The message is too big."]
					TooBig,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An HRMP message was sent to a sibling parachain."]
					XcmpMessageSent {
						message_hash: [::core::primitive::u8; 32usize],
					},
				}
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct OutboundChannelDetails {
				pub recipient: runtime_types::polkadot_parachain_primitives::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
				pub signals_exist: ::core::primitive::bool,
				pub first_index: ::core::primitive::u16,
				pub last_index: ::core::primitive::u16,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum OutboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct QueueConfigData {
				pub suspend_threshold: ::core::primitive::u32,
				pub drop_threshold: ::core::primitive::u32,
				pub resume_threshold: ::core::primitive::u32,
			}
		}
		pub mod cumulus_primitives_core {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum AggregateMessageOrigin {
				#[codec(index = 0)]
				Here,
				#[codec(index = 1)]
				Parent,
				#[codec(index = 2)]
				Sibling(runtime_types::polkadot_parachain_primitives::primitives::Id),
			}
		}
		pub mod cumulus_primitives_parachain_inherent {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct MessageQueueChain(pub ::subxt::ext::subxt_core::utils::H256);
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct ParachainInherentData {
				pub validation_data: runtime_types::polkadot_primitives::v7::PersistedValidationData<
					::subxt::ext::subxt_core::utils::H256,
					::core::primitive::u32,
				>,
				pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
				pub downward_messages: ::subxt::ext::subxt_core::alloc::vec::Vec<
					runtime_types::polkadot_core_primitives::InboundDownwardMessage<::core::primitive::u32>,
				>,
				pub horizontal_messages: ::subxt::ext::subxt_core::utils::KeyedVec<
					runtime_types::polkadot_parachain_primitives::primitives::Id,
					::subxt::ext::subxt_core::alloc::vec::Vec<
						runtime_types::polkadot_core_primitives::InboundHrmpMessage<::core::primitive::u32>,
					>,
				>,
			}
		}
		pub mod cumulus_primitives_storage_weight_reclaim {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct StorageWeightReclaim;
		}
		pub mod cyborg_primitives {
			use super::runtime_types;
			pub mod oracle {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct ProcessStatus {
					pub online: ::core::primitive::bool,
					pub available: ::core::primitive::bool,
				}
			}
			pub mod task {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct TaskInfo<_0, _1> {
					pub task_owner: _0,
					pub create_block: _1,
					pub metadata:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					pub time_elapsed: ::core::option::Option<_1>,
					pub average_cpu_percentage_use: ::core::option::Option<::core::primitive::u8>,
					pub task_type: runtime_types::cyborg_primitives::task::TaskType,
					pub result: ::core::option::Option<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					>,
					pub compute_hours_deposit: ::core::option::Option<_1>,
					pub consume_compute_hours: ::core::option::Option<_1>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum TaskStatusType {
					#[codec(index = 0)]
					Assigned,
					#[codec(index = 1)]
					PendingValidation,
					#[codec(index = 2)]
					Completed,
					#[codec(index = 3)]
					Expired,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum TaskType {
					#[codec(index = 0)]
					Docker,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct VerificationHashes<_0> {
					pub account: _0,
					pub completed_hash: ::core::option::Option<::subxt::ext::subxt_core::utils::H256>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Verifications<_0> {
					pub executor: runtime_types::cyborg_primitives::task::VerificationHashes<_0>,
					pub verifier:
						::core::option::Option<runtime_types::cyborg_primitives::task::VerificationHashes<_0>>,
					pub resolver:
						::core::option::Option<runtime_types::cyborg_primitives::task::VerificationHashes<_0>>,
				}
			}
			pub mod worker {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Location {
					pub latitude: ::core::primitive::i32,
					pub longitude: ::core::primitive::i32,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Worker<_0, _1, _2> {
					pub id: ::core::primitive::u64,
					pub owner: _0,
					pub location: runtime_types::cyborg_primitives::worker::Location,
					pub specs: runtime_types::cyborg_primitives::worker::WorkerSpecs,
					pub reputation: ::core::primitive::u8,
					pub start_block: _1,
					pub status: runtime_types::cyborg_primitives::worker::WorkerStatusType,
					pub status_last_updated: _1,
					pub api: runtime_types::cyborg_primitives::worker::WorkerAPI,
					pub last_status_check: _2,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct WorkerAPI {
					pub domain:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct WorkerSpecs {
					pub ram: ::core::primitive::u64,
					pub storage: ::core::primitive::u64,
					pub cpu: ::core::primitive::u16,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum WorkerStatusType {
					#[codec(index = 0)]
					Active,
					#[codec(index = 1)]
					Busy,
					#[codec(index = 2)]
					Inactive,
				}
			}
		}
		pub mod cyborg_runtime {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec(index = 2)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 3)]
				ParachainInfo(runtime_types::staging_parachain_info::pallet::Call),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 15)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 32)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 33)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Call),
				#[codec(index = 40)]
				Oracle(runtime_types::orml_oracle::module::Call),
				#[codec(index = 41)]
				OracleMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 42)]
				EdgeConnect(runtime_types::pallet_edge_connect::pallet::Call),
				#[codec(index = 43)]
				TaskManagement(runtime_types::pallet_task_management::pallet::Call),
				#[codec(index = 45)]
				Payment(runtime_types::pallet_payment::pallet::Call),
				#[codec(index = 46)]
				ZKVerifier(runtime_types::pallet_zk_verifier::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum RuntimeError {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Error),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Error),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Error),
				#[codec(index = 15)]
				Sudo(runtime_types::pallet_sudo::pallet::Error),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Error),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Error),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Error),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Error),
				#[codec(index = 33)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Error),
				#[codec(index = 40)]
				Oracle(runtime_types::orml_oracle::module::Error),
				#[codec(index = 41)]
				OracleMembership(runtime_types::pallet_membership::pallet::Error),
				#[codec(index = 42)]
				EdgeConnect(runtime_types::pallet_edge_connect::pallet::Error),
				#[codec(index = 43)]
				TaskManagement(runtime_types::pallet_task_management::pallet::Error),
				#[codec(index = 45)]
				Payment(runtime_types::pallet_payment::pallet::Error),
				#[codec(index = 46)]
				ZKVerifier(runtime_types::pallet_zk_verifier::pallet::Error),
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 11)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 15)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 32)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 33)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Event),
				#[codec(index = 40)]
				Oracle(runtime_types::orml_oracle::module::Event),
				#[codec(index = 41)]
				OracleMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 42)]
				EdgeConnect(runtime_types::pallet_edge_connect::pallet::Event),
				#[codec(index = 43)]
				TaskManagement(runtime_types::pallet_task_management::pallet::Event),
				#[codec(index = 44)]
				StatusAggregator(runtime_types::pallet_status_aggregator::pallet::Event),
				#[codec(index = 45)]
				Payment(runtime_types::pallet_payment::pallet::Event),
				#[codec(index = 46)]
				ZKVerifier(runtime_types::pallet_zk_verifier::pallet::Event),
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum RuntimeFreezeReason {}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum RuntimeHoldReason {}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct SessionKeys {
				pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
			}
		}
		pub mod frame_metadata_hash_extension {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct CheckMetadataHash {
				pub mode: runtime_types::frame_metadata_hash_extension::Mode,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum Mode {
				#[codec(index = 0)]
				Disabled,
				#[codec(index = 1)]
				Enabled,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod messages {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum ProcessMessageError {
						#[codec(index = 0)]
						BadFormat,
						#[codec(index = 1)]
						Corrupt,
						#[codec(index = 2)]
						Unsupported,
						#[codec(index = 3)]
						Overweight(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 4)]
						Yield,
						#[codec(index = 5)]
						StackLimitReached,
					}
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
							:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
							:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
						#[codec(dumb_trait_bound)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<::core::primitive::u32>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic: ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total: ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved: ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Make some on-chain remark."]
					#[doc = ""]
					#[doc = "Can be executed by every `origin`."]
					remark {
						remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Set the number of pages in the WebAssembly environment's heap."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					#[doc = "Set the new runtime code."]
					set_code {
						code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					#[doc = "Set the new runtime code without doing any checks of the given `code`."]
					#[doc = ""]
					#[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
					#[doc = "version!"]
					set_code_without_checks {
						code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 4)]
					#[doc = "Set some items of storage."]
					set_storage {
						items: ::subxt::ext::subxt_core::alloc::vec::Vec<(
							::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
							::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					#[doc = "Kill some items from storage."]
					kill_storage {
						keys: ::subxt::ext::subxt_core::alloc::vec::Vec<
							::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						>,
					},
					#[codec(index = 6)]
					#[doc = "Kill all storage items with a key that starts with the given prefix."]
					#[doc = ""]
					#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
					#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
					kill_prefix {
						prefix: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "Make some on-chain remark and emit event."]
					remark_with_event {
						remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 9)]
					#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
					#[doc = "later."]
					#[doc = ""]
					#[doc = "This call requires Root origin."]
					authorize_upgrade {
						code_hash: ::subxt::ext::subxt_core::utils::H256,
					},
					#[codec(index = 10)]
					#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
					#[doc = "later."]
					#[doc = ""]
					#[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
					#[doc = "example that the spec name remains the same and that the version number increases. Not"]
					#[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
					#[doc = ""]
					#[doc = "This call requires Root origin."]
					authorize_upgrade_without_checks {
						code_hash: ::subxt::ext::subxt_core::utils::H256,
					},
					#[codec(index = 11)]
					#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
					#[doc = ""]
					#[doc = "If the authorization required a version check, this call will ensure the spec name"]
					#[doc = "remains unchanged and that the spec version has increased."]
					#[doc = ""]
					#[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
					#[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
					#[doc = ""]
					#[doc = "All origins are allowed."]
					apply_authorized_upgrade {
						code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The name of specification does not match between the current runtime"]
					#[doc = "and the new runtime."]
					InvalidSpecName,
					#[codec(index = 1)]
					#[doc = "The specification version is not allowed to decrease between the current runtime"]
					#[doc = "and the new runtime."]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					#[doc = "Failed to extract the runtime version from the new runtime."]
					#[doc = ""]
					#[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					#[doc = "Suicide called when the account has non-default composite data."]
					NonDefaultComposite,
					#[codec(index = 4)]
					#[doc = "There is a non-zero reference count preventing the account from being purged."]
					NonZeroRefCount,
					#[codec(index = 5)]
					#[doc = "The origin filter prevent the call to be dispatched."]
					CallFiltered,
					#[codec(index = 6)]
					#[doc = "A multi-block migration is ongoing and prevents the current code from being replaced."]
					MultiBlockMigrationsOngoing,
					#[codec(index = 7)]
					#[doc = "No upgrade authorized."]
					NothingAuthorized,
					#[codec(index = 8)]
					#[doc = "The submitted code is not authorized."]
					Unauthorized,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Event for the System pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully."]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					#[doc = "An extrinsic failed."]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new account was created."]
					NewAccount {
						account: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 4)]
					#[doc = "An account was reaped."]
					KilledAccount {
						account: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened."]
					Remarked {
						sender: ::subxt::ext::subxt_core::utils::AccountId32,
						hash: ::subxt::ext::subxt_core::utils::H256,
					},
					#[codec(index = 6)]
					#[doc = "An upgrade was authorized."]
					UpgradeAuthorized {
						code_hash: ::subxt::ext::subxt_core::utils::H256,
						check_version: ::core::primitive::bool,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: ::core::primitive::u32,
				pub providers: ::core::primitive::u32,
				pub sufficients: ::core::primitive::u32,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct CodeUpgradeAuthorization {
				pub code_hash: ::subxt::ext::subxt_core::utils::H256,
				pub check_version: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod orml_oracle {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Feed the external value."]
					#[doc = ""]
					#[doc = "Require authorized operator."]
					feed_values {
						values: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							(
								::subxt::ext::subxt_core::utils::AccountId32,
								::core::primitive::u64,
							),
							runtime_types::cyborg_primitives::oracle::ProcessStatus,
						)>,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender does not have permission"]
					NoPermission,
					#[codec(index = 1)]
					#[doc = "Feeder has already feeded at this block"]
					AlreadyFeeded,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New feed data is submitted."]
					NewFeedData {
						sender: ::subxt::ext::subxt_core::utils::AccountId32,
						values: ::subxt::ext::subxt_core::alloc::vec::Vec<(
							(
								::subxt::ext::subxt_core::utils::AccountId32,
								::core::primitive::u64,
							),
							runtime_types::cyborg_primitives::oracle::ProcessStatus,
						)>,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct TimestampedValue<_0, _1> {
					pub value: _0,
					pub timestamp: _1,
				}
			}
		}
		pub mod orml_utilities {
			use super::runtime_types;
			pub mod ordered_set {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct OrderedSet<_0>(
					pub runtime_types::bounded_collections::bounded_vec::BoundedVec<_0>,
				);
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "If the sender's account is below the existential deposit as a result"]
					#[doc = "of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
					transfer_allow_death {
						dest: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
					#[doc = "may be specified."]
					force_transfer {
						source: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						dest: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
					#[doc = "kill the origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
					#[doc = ""]
					#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
					transfer_keep_alive {
						dest: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Transfer the entire transferable balance from the caller account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
					#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
					#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
					#[doc = "you might need to prepare the account by removing any reference counters, storage"]
					#[doc = "deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be Signed."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
					#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
					#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true)."]
					transfer_all {
						dest: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Upgrade a specified account."]
					#[doc = ""]
					#[doc = "- `origin`: Must be `Signed`."]
					#[doc = "- `who`: The account to be upgraded."]
					#[doc = ""]
					#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
					#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
					#[doc = "possibility of churn)."]
					upgrade_accounts {
						who: ::subxt::ext::subxt_core::alloc::vec::Vec<
							::subxt::ext::subxt_core::utils::AccountId32,
						>,
					},
					#[codec(index = 8)]
					#[doc = "Set the regular balance of a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					force_set_balance {
						who: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Adjust the total issuance in a saturating way."]
					#[doc = ""]
					#[doc = "Can only be called by root and always needs a positive `delta`."]
					#[doc = ""]
					#[doc = "# Example"]
					force_adjust_total_issuance {
						direction: runtime_types::pallet_balances::types::AdjustmentDirection,
						#[codec(compact)]
						delta: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					#[doc = "Burn the specified liquid free balance from the origin account."]
					#[doc = ""]
					#[doc = "If the origin's account ends up below the existential deposit as a result"]
					#[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
					#[doc = ""]
					#[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
					#[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
					burn {
						#[codec(compact)]
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value."]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal."]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value."]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit."]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account."]
					Expendability,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account."]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist."]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed `MaxReserves`."]
					TooManyReserves,
					#[codec(index = 8)]
					#[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
					TooManyHolds,
					#[codec(index = 9)]
					#[doc = "Number of freezes exceed `MaxFreezes`."]
					TooManyFreezes,
					#[codec(index = 10)]
					#[doc = "The issuance cannot be modified since it is already deactivated."]
					IssuanceDeactivated,
					#[codec(index = 11)]
					#[doc = "The delta cannot be zero."]
					DeltaZero,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::ext::subxt_core::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::ext::subxt_core::utils::AccountId32,
						to: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						free: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::ext::subxt_core::utils::AccountId32,
						to: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					#[doc = "Some amount was minted into an account."]
					Minted {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					#[doc = "Some amount was burned from an account."]
					Burned {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "Some amount was suspended from an account (it can be restored later)."]
					Suspended {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 13)]
					#[doc = "Some amount was restored into an account."]
					Restored {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 14)]
					#[doc = "An account was upgraded."]
					Upgraded {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 15)]
					#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					#[doc = "Some balance was locked."]
					Locked {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 18)]
					#[doc = "Some balance was unlocked."]
					Unlocked {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 19)]
					#[doc = "Some balance was frozen."]
					Frozen {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 20)]
					#[doc = "Some balance was thawed."]
					Thawed {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 21)]
					#[doc = "The `TotalIssuance` was forcefully changed."]
					TotalIssuanceForced {
						old: ::core::primitive::u128,
						new: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct AccountData<_0> {
					pub free: _0,
					pub reserved: _0,
					pub frozen: _0,
					pub flags: runtime_types::pallet_balances::types::ExtraFlags,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum AdjustmentDirection {
					#[codec(index = 0)]
					Increase,
					#[codec(index = 1)]
					Decrease,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct BalanceLock<_0> {
					pub id: [::core::primitive::u8; 8usize],
					pub amount: _0,
					pub reasons: runtime_types::pallet_balances::types::Reasons,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct ExtraFlags(pub ::core::primitive::u128);
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct IdAmount<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum Reasons {
					#[codec(index = 0)]
					Fee,
					#[codec(index = 1)]
					Misc,
					#[codec(index = 2)]
					All,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct ReserveData<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
			}
		}
		pub mod pallet_collator_selection {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the list of invulnerable (fixed) collators. These collators must do some"]
					#[doc = "preparation, namely to have registered session keys."]
					#[doc = ""]
					#[doc = "The call will remove any accounts that have not registered keys from the set. That is,"]
					#[doc = "it is non-atomic; the caller accepts all `AccountId`s passed in `new` _individually_ as"]
					#[doc = "acceptable Invulnerables, and is not proposing a _set_ of new Invulnerables."]
					#[doc = ""]
					#[doc = "This call does not maintain mutual exclusivity of `Invulnerables` and `Candidates`. It"]
					#[doc = "is recommended to use a batch of `add_invulnerable` and `remove_invulnerable` instead. A"]
					#[doc = "`batch_all` can also be used to enforce atomicity. If any candidates are included in"]
					#[doc = "`new`, they should be removed with `remove_invulnerable_candidate` after execution."]
					#[doc = ""]
					#[doc = "Must be called by the `UpdateOrigin`."]
					set_invulnerables {
						new: ::subxt::ext::subxt_core::alloc::vec::Vec<
							::subxt::ext::subxt_core::utils::AccountId32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "Set the ideal number of non-invulnerable collators. If lowering this number, then the"]
					#[doc = "number of running collators could be higher than this figure. Aside from that edge case,"]
					#[doc = "there should be no other way to have more candidates than the desired number."]
					#[doc = ""]
					#[doc = "The origin for this call must be the `UpdateOrigin`."]
					set_desired_candidates { max: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "Set the candidacy bond amount."]
					#[doc = ""]
					#[doc = "If the candidacy bond is increased by this call, all current candidates which have a"]
					#[doc = "deposit lower than the new bond will be kicked from the list and get their deposits"]
					#[doc = "back."]
					#[doc = ""]
					#[doc = "The origin for this call must be the `UpdateOrigin`."]
					set_candidacy_bond { bond: ::core::primitive::u128 },
					#[codec(index = 3)]
					#[doc = "Register this account as a collator candidate. The account must (a) already have"]
					#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
					#[doc = ""]
					#[doc = "This call is not available to `Invulnerable` collators."]
					register_as_candidate,
					#[codec(index = 4)]
					#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
					#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
					#[doc = ""]
					#[doc = "This call will fail if the total number of candidates would drop below"]
					#[doc = "`MinEligibleCollators`."]
					leave_intent,
					#[codec(index = 5)]
					#[doc = "Add a new account `who` to the list of `Invulnerables` collators. `who` must have"]
					#[doc = "registered session keys. If `who` is a candidate, they will be removed."]
					#[doc = ""]
					#[doc = "The origin for this call must be the `UpdateOrigin`."]
					add_invulnerable {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 6)]
					#[doc = "Remove an account `who` from the list of `Invulnerables` collators. `Invulnerables` must"]
					#[doc = "be sorted."]
					#[doc = ""]
					#[doc = "The origin for this call must be the `UpdateOrigin`."]
					remove_invulnerable {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 7)]
					#[doc = "Update the candidacy bond of collator candidate `origin` to a new amount `new_deposit`."]
					#[doc = ""]
					#[doc = "Setting a `new_deposit` that is lower than the current deposit while `origin` is"]
					#[doc = "occupying a top-`DesiredCandidates` slot is not allowed."]
					#[doc = ""]
					#[doc = "This call will fail if `origin` is not a collator candidate, the updated bond is lower"]
					#[doc = "than the minimum candidacy bond, and/or the amount cannot be reserved."]
					update_bond {
						new_deposit: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "The caller `origin` replaces a candidate `target` in the collator candidate list by"]
					#[doc = "reserving `deposit`. The amount `deposit` reserved by the caller must be greater than"]
					#[doc = "the existing bond of the target it is trying to replace."]
					#[doc = ""]
					#[doc = "This call will fail if the caller is already a collator candidate or invulnerable, the"]
					#[doc = "caller does not have registered session keys, the target is not a collator candidate,"]
					#[doc = "and/or the `deposit` amount cannot be reserved."]
					take_candidate_slot {
						deposit: ::core::primitive::u128,
						target: ::subxt::ext::subxt_core::utils::AccountId32,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
					pub deposit: _1,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The pallet has too many candidates."]
					TooManyCandidates,
					#[codec(index = 1)]
					#[doc = "Leaving would result in too few candidates."]
					TooFewEligibleCollators,
					#[codec(index = 2)]
					#[doc = "Account is already a candidate."]
					AlreadyCandidate,
					#[codec(index = 3)]
					#[doc = "Account is not a candidate."]
					NotCandidate,
					#[codec(index = 4)]
					#[doc = "There are too many Invulnerables."]
					TooManyInvulnerables,
					#[codec(index = 5)]
					#[doc = "Account is already an Invulnerable."]
					AlreadyInvulnerable,
					#[codec(index = 6)]
					#[doc = "Account is not an Invulnerable."]
					NotInvulnerable,
					#[codec(index = 7)]
					#[doc = "Account has no associated validator ID."]
					NoAssociatedValidatorId,
					#[codec(index = 8)]
					#[doc = "Validator ID is not yet registered."]
					ValidatorNotRegistered,
					#[codec(index = 9)]
					#[doc = "Could not insert in the candidate list."]
					InsertToCandidateListFailed,
					#[codec(index = 10)]
					#[doc = "Could not remove from the candidate list."]
					RemoveFromCandidateListFailed,
					#[codec(index = 11)]
					#[doc = "New deposit amount would be below the minimum candidacy bond."]
					DepositTooLow,
					#[codec(index = 12)]
					#[doc = "Could not update the candidate list."]
					UpdateCandidateListFailed,
					#[codec(index = 13)]
					#[doc = "Deposit amount is too low to take the target's slot in the candidate list."]
					InsufficientBond,
					#[codec(index = 14)]
					#[doc = "The target account to be replaced in the candidate list is not a candidate."]
					TargetIsNotCandidate,
					#[codec(index = 15)]
					#[doc = "The updated deposit amount is equal to the amount already reserved."]
					IdenticalDeposit,
					#[codec(index = 16)]
					#[doc = "Cannot lower candidacy bond while occupying a future collator slot in the list."]
					InvalidUnreserve,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New Invulnerables were set."]
					NewInvulnerables {
						invulnerables: ::subxt::ext::subxt_core::alloc::vec::Vec<
							::subxt::ext::subxt_core::utils::AccountId32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "A new Invulnerable was added."]
					InvulnerableAdded {
						account_id: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 2)]
					#[doc = "An Invulnerable was removed."]
					InvulnerableRemoved {
						account_id: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 3)]
					#[doc = "The number of desired candidates was set."]
					NewDesiredCandidates {
						desired_candidates: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "The candidacy bond was set."]
					NewCandidacyBond {
						bond_amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "A new candidate joined."]
					CandidateAdded {
						account_id: ::subxt::ext::subxt_core::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Bond of a candidate updated."]
					CandidateBondUpdated {
						account_id: ::subxt::ext::subxt_core::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					#[doc = "A candidate was removed."]
					CandidateRemoved {
						account_id: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 8)]
					#[doc = "An account was replaced in the candidate list by another one."]
					CandidateReplaced {
						old: ::subxt::ext::subxt_core::utils::AccountId32,
						new: ::subxt::ext::subxt_core::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "An account was unable to be added to the Invulnerables because they did not have keys"]
					#[doc = "registered. Other Invulnerables may have been set."]
					InvalidInvulnerableSkipped {
						account_id: ::subxt::ext::subxt_core::utils::AccountId32,
					},
				}
			}
		}
		pub mod pallet_edge_connect {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Registers a Worker with either a domain and initialize it with an inactive status."]
					register_worker {
						domain:
							runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
						latitude: ::core::primitive::i32,
						longitude: ::core::primitive::i32,
						ram: ::core::primitive::u64,
						storage: ::core::primitive::u64,
						cpu: ::core::primitive::u16,
					},
					#[codec(index = 1)]
					#[doc = "Remove a worker from storage an deactivates it"]
					remove_worker { worker_id: ::core::primitive::u64 },
					#[codec(index = 2)]
					#[doc = "Switches the visibility of a worker between active and inactive."]
					toggle_worker_visibility {
						worker_id: ::core::primitive::u64,
						visibility: ::core::primitive::bool,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum contains all possible errors that can occur when interacting with this pallet."]
				#[doc = "These errors will be returned in the `DispatchResult` when a function call fails."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Error indicating that either the IP address or the domain was missing when attempting to register a worker."]
					WorkerRegisterMissingIpOrDomain,
					#[codec(index = 1)]
					#[doc = "Error indicating that the worker already exists and cannot be registered again."]
					WorkerExists,
					#[codec(index = 2)]
					#[doc = "Error indicating that the worker does not exist in the system when trying to perform actions (e.g., removal or status update)."]
					WorkerDoesNotExist,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum contains the various events that can be emitted by this pallet."]
				#[doc = "Events are emitted when significant actions or state changes happen in the pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Event emitted when a new worker is successfully registered."]
					#[doc = ""]
					#[doc = "- `creator`: The account ID of the worker's creator."]
					#[doc = "- `worker`: A tuple containing the account ID of the worker owner and the worker ID."]
					#[doc = "- `domain`: The domain associated with the"]
					WorkerRegistered {
						creator: ::subxt::ext::subxt_core::utils::AccountId32,
						worker: (
							::subxt::ext::subxt_core::utils::AccountId32,
							::core::primitive::u64,
						),
						domain:
							runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Event emitted when a worker is removed from the system."]
					#[doc = ""]
					#[doc = "- `creator`: The account ID of the worker's creator."]
					#[doc = "- `worker_id`: The ID of the worker that was removed."]
					WorkerRemoved {
						creator: ::subxt::ext::subxt_core::utils::AccountId32,
						worker_id: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					#[doc = "Event emitted when a worker's status is updated (e.g., toggling visibility)."]
					#[doc = ""]
					#[doc = "- `creator`: The account ID of the worker's creator."]
					#[doc = "- `worker_id`: The ID of the worker whose status was updated."]
					#[doc = "- `worker_status`: The new status of the worker, either active or inactive."]
					WorkerStatusUpdated {
						creator: ::subxt::ext::subxt_core::utils::AccountId32,
						worker_id: ::core::primitive::u64,
						worker_status: runtime_types::cyborg_primitives::worker::WorkerStatusType,
					},
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Add a member `who` to the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::AddOrigin`."]
					add_member {
						who: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
					},
					#[codec(index = 1)]
					#[doc = "Remove a member `who` from the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::RemoveOrigin`."]
					remove_member {
						who: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
					},
					#[codec(index = 2)]
					#[doc = "Swap out one member `remove` for another `add`."]
					#[doc = ""]
					#[doc = "May only be called from `T::SwapOrigin`."]
					#[doc = ""]
					#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
					swap_member {
						remove: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						add: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
					},
					#[codec(index = 3)]
					#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
					#[doc = "pass `members` pre-sorted."]
					#[doc = ""]
					#[doc = "May only be called from `T::ResetOrigin`."]
					reset_members {
						members: ::subxt::ext::subxt_core::alloc::vec::Vec<
							::subxt::ext::subxt_core::utils::AccountId32,
						>,
					},
					#[codec(index = 4)]
					#[doc = "Swap out the sending member for some other key `new`."]
					#[doc = ""]
					#[doc = "May only be called from `Signed` origin of a current member."]
					#[doc = ""]
					#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
					change_key {
						new: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
					},
					#[codec(index = 5)]
					#[doc = "Set the prime member. Must be a current member."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					set_prime {
						who: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
					},
					#[codec(index = 6)]
					#[doc = "Remove the prime member if it exists."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					clear_prime,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Already a member."]
					AlreadyMember,
					#[codec(index = 1)]
					#[doc = "Not a member."]
					NotMember,
					#[codec(index = 2)]
					#[doc = "Too many members."]
					TooManyMembers,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The given member was added; see the transaction for who."]
					MemberAdded,
					#[codec(index = 1)]
					#[doc = "The given member was removed; see the transaction for who."]
					MemberRemoved,
					#[codec(index = 2)]
					#[doc = "Two members were swapped; see the transaction for who."]
					MembersSwapped,
					#[codec(index = 3)]
					#[doc = "The membership was reset; see the transaction for who the new set is."]
					MembersReset,
					#[codec(index = 4)]
					#[doc = "One of the members' keys changed."]
					KeyChanged,
					#[codec(index = 5)]
					#[doc = "Phantom member, never used."]
					Dummy,
				}
			}
		}
		pub mod pallet_message_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Remove a page which has no more messages remaining to be processed or is stale."]
					reap_page {
						message_origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						page_index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Execute an overweight message."]
					#[doc = ""]
					#[doc = "Temporary processing errors will be propagated whereas permanent errors are treated"]
					#[doc = "as success condition."]
					#[doc = ""]
					#[doc = "- `origin`: Must be `Signed`."]
					#[doc = "- `message_origin`: The origin from which the message to be executed arrived."]
					#[doc = "- `page`: The page in the queue in which the message to be executed is sitting."]
					#[doc = "- `index`: The index into the queue of the message to be executed."]
					#[doc = "- `weight_limit`: The maximum amount of weight allowed to be consumed in the execution"]
					#[doc = "  of the message."]
					#[doc = ""]
					#[doc = "Benchmark complexity considerations: O(index + weight_limit)."]
					execute_overweight {
						message_origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						page: ::core::primitive::u32,
						index: ::core::primitive::u32,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Page is not reapable because it has items remaining to be processed and is not old"]
					#[doc = "enough."]
					NotReapable,
					#[codec(index = 1)]
					#[doc = "Page to be reaped does not exist."]
					NoPage,
					#[codec(index = 2)]
					#[doc = "The referenced message could not be found."]
					NoMessage,
					#[codec(index = 3)]
					#[doc = "The message was already processed and cannot be processed again."]
					AlreadyProcessed,
					#[codec(index = 4)]
					#[doc = "The message is queued for future execution."]
					Queued,
					#[codec(index = 5)]
					#[doc = "There is temporarily not enough weight to continue servicing messages."]
					InsufficientWeight,
					#[codec(index = 6)]
					#[doc = "This message is temporarily unprocessable."]
					#[doc = ""]
					#[doc = "Such errors are expected, but not guaranteed, to resolve themselves eventually through"]
					#[doc = "retrying."]
					TemporarilyUnprocessable,
					#[codec(index = 7)]
					#[doc = "The queue is paused and no message can be executed from it."]
					#[doc = ""]
					#[doc = "This can change at any time and may resolve in the future by re-trying."]
					QueuePaused,
					#[codec(index = 8)]
					#[doc = "Another call is in progress and needs to finish before this call can happen."]
					RecursiveDisallowed,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Message discarded due to an error in the `MessageProcessor` (usually a format error)."]
					ProcessingFailed {
						id: ::subxt::ext::subxt_core::utils::H256,
						origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						error: runtime_types::frame_support::traits::messages::ProcessMessageError,
					},
					#[codec(index = 1)]
					#[doc = "Message is processed."]
					Processed {
						id: ::subxt::ext::subxt_core::utils::H256,
						origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						success: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					#[doc = "Message placed in overweight queue."]
					OverweightEnqueued {
						id: [::core::primitive::u8; 32usize],
						origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						page_index: ::core::primitive::u32,
						message_index: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "This page was reaped."]
					PageReaped {
						origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						index: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct BookState<_0> {
				pub begin: ::core::primitive::u32,
				pub end: ::core::primitive::u32,
				pub count: ::core::primitive::u32,
				pub ready_neighbours:
					::core::option::Option<runtime_types::pallet_message_queue::Neighbours<_0>>,
				pub message_count: ::core::primitive::u64,
				pub size: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct Neighbours<_0> {
				pub prev: _0,
				pub next: _0,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct Page<_0> {
				pub remaining: _0,
				pub remaining_size: _0,
				pub first_index: _0,
				pub first: _0,
				pub last: _0,
				pub heap:
					runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
			}
		}
		pub mod pallet_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Allows the admin (root) to set the price per compute hour."]
					set_price_per_hour { new_price: ::core::primitive::u128 },
					#[codec(index = 1)]
					#[doc = "Allows the admin (root) to set the service provider's account."]
					set_service_provider_account {
						new_account: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 2)]
					#[doc = "Allows a user to purchase compute hours."]
					purchase_compute_hours { hours: ::core::primitive::u32 },
					#[codec(index = 3)]
					#[doc = "Allows a user to consume compute hours."]
					consume_compute_hours { hours: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Pallet Errors"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The user's balance is insufficient for the transaction."]
					InsufficientBalance,
					#[codec(index = 1)]
					#[doc = "The user does not have enough compute hours to consume."]
					InsufficientComputeHours,
					#[codec(index = 2)]
					#[doc = "The user has provided an invalid input for the number of hours (e.g., 0)."]
					InvalidHoursInput,
					#[codec(index = 3)]
					#[doc = "The service provider account is not found."]
					ServiceProviderAccountNotFound,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Event triggered when compute hours are consumed."]
					HoursConsumed(
						::subxt::ext::subxt_core::utils::AccountId32,
						::core::primitive::u32,
					),
					#[codec(index = 1)]
					#[doc = "Event triggered when compute hours are purchased."]
					HoursPurchased(
						::subxt::ext::subxt_core::utils::AccountId32,
						::core::primitive::u32,
						::core::primitive::u128,
					),
					#[codec(index = 2)]
					#[doc = "Event triggered when the admin sets a new price per hour."]
					PricePerHourSet(::core::primitive::u128),
					#[codec(index = 3)]
					#[doc = "Event triggered when the admin sets a new service provider account."]
					ServiceProviderAccountSet(::subxt::ext::subxt_core::utils::AccountId32),
				}
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Sets the session key(s) of the function caller to `keys`."]
					#[doc = "Allows an account to set its session key prior to becoming a validator."]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be signed."]
					#[doc = ""]
					#[doc = "## Complexity"]
					#[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
					#[doc = "  fixed."]
					set_keys {
						keys: runtime_types::cyborg_runtime::SessionKeys,
						proof: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Removes any session key(s) of the function caller."]
					#[doc = ""]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
					#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
					#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
					#[doc = "usually means being a stash account)."]
					#[doc = ""]
					#[doc = "## Complexity"]
					#[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
					#[doc = "  `T::Keys::key_ids()` which is fixed."]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Error for the session pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid ownership proof."]
					InvalidProof,
					#[codec(index = 1)]
					#[doc = "No associated validator ID for account."]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					#[doc = "Registered duplicate key."]
					DuplicatedKey,
					#[codec(index = 3)]
					#[doc = "No keys are associated with this account."]
					NoKeys,
					#[codec(index = 4)]
					#[doc = "Key setting account is not live, so it's impossible to associate keys."]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New session has happened. Note that the argument is the session index, not the"]
					#[doc = "block number as the type might suggest."]
					NewSession {
						session_index: ::core::primitive::u32,
					},
				}
			}
		}
		pub mod pallet_status_aggregator {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum contains the various events that can be emitted by this pallet."]
				#[doc = "Events are emitted when significant actions or state changes happen in the pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Event emitted when the worker status is updated based on aggregated data from the oracle."]
					#[doc = "This provides the new online and availability status for the worker and the block number where the status was last updated."]
					#[doc = ""]
					#[doc = "- `worker`: A tuple containing the worker's account ID and the worker ID."]
					#[doc = "- `online`: A boolean indicating whether the worker is online."]
					#[doc = "- `available`: A boolean indicating whether the worker is available."]
					#[doc = "- `last_block_processed`: The block number at which the worker's status was last updated."]
					UpdateFromAggregatedWorkerInfo {
						worker: (
							::subxt::ext::subxt_core::utils::AccountId32,
							::core::primitive::u64,
						),
						online: ::core::primitive::bool,
						available: ::core::primitive::bool,
						last_block_processed: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Event emitted when the last block is updated after clearing data for the current period."]
					#[doc = "This indicates that data from the oracle has been successfully processed and cleared for the given block range."]
					#[doc = ""]
					#[doc = "- `block_number`: The block number at which the clearing occurred."]
					LastBlockUpdated {
						block_number: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct ProcessStatusPercentages<_0> {
				pub online: ::core::primitive::u8,
				pub available: ::core::primitive::u8,
				pub last_block_processed: _0,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct StatusInstance<_0> {
				pub is_online: ::core::primitive::bool,
				pub is_available: ::core::primitive::bool,
				pub block: _0,
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					sudo {
						call: ::subxt::ext::subxt_core::alloc::boxed::Box<
							runtime_types::cyborg_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 1)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = "This function does not check the weight of the call, and instead allows the"]
					#[doc = "Sudo user to specify the weight of the call."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					sudo_unchecked_weight {
						call: ::subxt::ext::subxt_core::alloc::boxed::Box<
							runtime_types::cyborg_runtime::RuntimeCall,
						>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
					#[doc = "key."]
					set_key {
						new: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
					},
					#[codec(index = 3)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
					#[doc = "a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					sudo_as {
						who: ::subxt::ext::subxt_core::utils::MultiAddress<
							::subxt::ext::subxt_core::utils::AccountId32,
							(),
						>,
						call: ::subxt::ext::subxt_core::alloc::boxed::Box<
							runtime_types::cyborg_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 4)]
					#[doc = "Permanently removes the sudo key."]
					#[doc = ""]
					#[doc = "**This cannot be un-done.**"]
					remove_key,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Error for the Sudo pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account."]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo call just took place."]
					Sudid {
						sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The sudo key has been updated."]
					KeyChanged {
						old: ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
						new: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 2)]
					#[doc = "The key was permanently removed."]
					KeyRemoved,
					#[codec(index = 3)]
					#[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
					SudoAsDone {
						sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_task_management {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Creates a new task and assigns it to a randomly selected worker."]
					task_scheduler {
						task_data:
							runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
						compute_hours_deposit: ::core::option::Option<::core::primitive::u32>,
						worker_owner: ::subxt::ext::subxt_core::utils::AccountId32,
						worker_id: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					#[doc = "Allows a worker to submit a completed task for verification by a verifier."]
					submit_completed_task {
						task_id: ::core::primitive::u64,
						completed_hash: ::subxt::ext::subxt_core::utils::H256,
						result:
							runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					#[doc = "Verifies whether the submitted completed task is correct."]
					#[doc = "If verification fails, a new resolver is assigned to review the task."]
					verify_completed_task {
						task_id: ::core::primitive::u64,
						completed_hash: ::subxt::ext::subxt_core::utils::H256,
					},
					#[codec(index = 3)]
					#[doc = "Resolver finalizes the verification of a task in case of disputes."]
					resolve_completed_task {
						task_id: ::core::primitive::u64,
						completed_hash: ::subxt::ext::subxt_core::utils::H256,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Errors inform users that something went wrong."]
				#[doc = "<https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The provided task ID does not exist."]
					UnassignedTaskId,
					#[codec(index = 1)]
					#[doc = "The caller is not the task owner."]
					InvalidTaskOwner,
					#[codec(index = 2)]
					#[doc = "A task must be assigned before it can proceed to the next step."]
					RequireAssignedTask,
					#[codec(index = 3)]
					#[doc = "A verifier must be assigned to the task."]
					RequireAssignedVerifier,
					#[codec(index = 4)]
					#[doc = "A completed task's hash must be provided by the assigned verifier."]
					RequireAssignedVerifierCompletedHash,
					#[codec(index = 5)]
					#[doc = "A resolver must be assigned to review the task."]
					RequireAssignedResolver,
					#[codec(index = 6)]
					#[doc = "No workers are available for the task."]
					NoWorkersAvailable,
					#[codec(index = 7)]
					#[doc = "The task verification process cannot be found."]
					TaskVerificationNotFound,
					#[codec(index = 8)]
					#[doc = "No new workers are available for the task reassignment."]
					NoNewWorkersAvailable,
					#[codec(index = 9)]
					#[doc = "A compute hour deposit is required to schedule or proceed with the task."]
					RequireComputeHoursDeposit,
					#[codec(index = 10)]
					#[doc = "The user has insufficient compute hours balance for the requested deposit."]
					InsufficientComputeHours,
					#[codec(index = 11)]
					#[doc = "The worker, to which the task should be assigned does not exist."]
					WorkerDoesNotExist,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Pallets use events to inform users when important changes are made."]
				#[doc = "<https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A task has been scheduled and assigned to a worker."]
					TaskScheduled {
						assigned_worker: (
							::subxt::ext::subxt_core::utils::AccountId32,
							::core::primitive::u64,
						),
						task_owner: ::subxt::ext::subxt_core::utils::AccountId32,
						task_id: ::core::primitive::u64,
						task:
							runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "A completed task has been submitted for verification."]
					SubmittedCompletedTask {
						task_id: ::core::primitive::u64,
						assigned_verifier: (
							::subxt::ext::subxt_core::utils::AccountId32,
							::core::primitive::u64,
						),
					},
					#[codec(index = 2)]
					#[doc = "A resolver has been assigned to determine the correct result after verification failure."]
					VerifierResolverAssigned {
						task_id: ::core::primitive::u64,
						assigned_resolver: (
							::subxt::ext::subxt_core::utils::AccountId32,
							::core::primitive::u64,
						),
					},
					#[codec(index = 3)]
					#[doc = "A completed task has been successfully verified."]
					VerifiedCompletedTask { task_id: ::core::primitive::u64 },
					#[codec(index = 4)]
					#[doc = "A completed task has been successfully resolved by the resolver."]
					ResolvedCompletedTask { task_id: ::core::primitive::u64 },
					#[codec(index = 5)]
					#[doc = "A task has been reassigned to a new worker."]
					TaskReassigned {
						task_id: ::core::primitive::u64,
						assigned_executor: (
							::subxt::ext::subxt_core::utils::AccountId32,
							::core::primitive::u64,
						),
					},
				}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the current time."]
					#[doc = ""]
					#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
					#[doc = "phase, if this call hasn't been invoked by that time."]
					#[doc = ""]
					#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
					#[doc = "[`Config::MinimumPeriod`]."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _None_."]
					#[doc = ""]
					#[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
					#[doc = "that changing the complexity of this call could result exhausting the resources in a"]
					#[doc = "block to execute any other calls."]
					#[doc = ""]
					#[doc = "## Complexity"]
					#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
					#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
					#[doc = "  `on_finalize`)"]
					#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
					#[doc = "has been paid by `who`."]
					TransactionFeePaid {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						message: ::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "**This function is deprecated: Use `limited_teleport_assets` instead.**"]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
					#[doc = "with all fees taken as needed from the asset."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
					#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
					#[doc = "  relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
					#[doc = "  generally be an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
					#[doc = "  fee on the `dest` chain."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					teleport_assets {
						dest:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						beneficiary:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						assets:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
					#[doc = "destination or remote reserve."]
					#[doc = ""]
					#[doc = "`assets` must have same reserve location and may not be teleportable to `dest`."]
					#[doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"]
					#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
					#[doc = "   assets to `beneficiary`."]
					#[doc = " - `assets` have destination reserve: burn local assets and forward a notification to"]
					#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
					#[doc = "   deposit them to `beneficiary`."]
					#[doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"]
					#[doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"]
					#[doc = "   to mint and deposit reserve-based assets to `beneficiary`."]
					#[doc = ""]
					#[doc = "**This function is deprecated: Use `limited_reserve_transfer_assets` instead.**"]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
					#[doc = "with all fees taken as needed from the asset."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
					#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
					#[doc = "  relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
					#[doc = "  generally be an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
					#[doc = "  fee on the `dest` (and possibly reserve) chains."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					reserve_transfer_assets {
						dest:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						beneficiary:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						assets:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Execute an XCM message from a local, signed, origin."]
					#[doc = ""]
					#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
					#[doc = "partially."]
					#[doc = ""]
					#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than"]
					#[doc = "the maximum amount of weight that the message could take to be executed, then no"]
					#[doc = "execution attempt will be made."]
					execute {
						message: ::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedXcm>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					#[doc = "Extoll that a particular destination can be communicated with through a particular"]
					#[doc = "version of XCM."]
					#[doc = ""]
					#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
					#[doc = "- `location`: The destination that is being described."]
					#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
					force_xcm_version {
						location: ::subxt::ext::subxt_core::alloc::boxed::Box<
							runtime_types::staging_xcm::v4::location::Location,
						>,
						version: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
					#[doc = "version a destination can accept is unknown)."]
					#[doc = ""]
					#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
					#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 6)]
					#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
					#[doc = ""]
					#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
					#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
					force_subscribe_version_notify {
						location:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
					},
					#[codec(index = 7)]
					#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
					#[doc = "version changes."]
					#[doc = ""]
					#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
					#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
					#[doc = "  notifications which we no longer desire."]
					force_unsubscribe_version_notify {
						location:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
					},
					#[codec(index = 8)]
					#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
					#[doc = "destination or remote reserve."]
					#[doc = ""]
					#[doc = "`assets` must have same reserve location and may not be teleportable to `dest`."]
					#[doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"]
					#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
					#[doc = "   assets to `beneficiary`."]
					#[doc = " - `assets` have destination reserve: burn local assets and forward a notification to"]
					#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
					#[doc = "   deposit them to `beneficiary`."]
					#[doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"]
					#[doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"]
					#[doc = "   to mint and deposit reserve-based assets to `beneficiary`."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
					#[doc = "at risk."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
					#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
					#[doc = "  relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
					#[doc = "  generally be an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
					#[doc = "  fee on the `dest` (and possibly reserve) chains."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_reserve_transfer_assets {
						dest:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						beneficiary:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						assets:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 9)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
					#[doc = "at risk."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
					#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
					#[doc = "  relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
					#[doc = "  generally be an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
					#[doc = "  fee on the `dest` chain."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_teleport_assets {
						dest:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						beneficiary:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						assets:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 10)]
					#[doc = "Set or unset the global suspension state of the XCM executor."]
					#[doc = ""]
					#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
					#[doc = "- `suspended`: `true` to suspend, `false` to resume."]
					force_suspension { suspended: ::core::primitive::bool },
					#[codec(index = 11)]
					#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
					#[doc = "destination or remote reserve, or through teleports."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item` (hence referred to as `fees`), up to enough to pay for"]
					#[doc = "`weight_limit` of weight. If more weight is needed than `weight_limit`, then the"]
					#[doc = "operation will fail and the sent assets may be at risk."]
					#[doc = ""]
					#[doc = "`assets` (excluding `fees`) must have same reserve location or otherwise be teleportable"]
					#[doc = "to `dest`, no limitations imposed on `fees`."]
					#[doc = " - for local reserve: transfer assets to sovereign account of destination chain and"]
					#[doc = "   forward a notification XCM to `dest` to mint and deposit reserve-based assets to"]
					#[doc = "   `beneficiary`."]
					#[doc = " - for destination reserve: burn local assets and forward a notification to `dest` chain"]
					#[doc = "   to withdraw the reserve assets from this chain's sovereign account and deposit them"]
					#[doc = "   to `beneficiary`."]
					#[doc = " - for remote reserve: burn local assets, forward XCM to reserve chain to move reserves"]
					#[doc = "   from this chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint"]
					#[doc = "   and deposit reserve-based assets to `beneficiary`."]
					#[doc = " - for teleports: burn local assets and forward XCM to `dest` chain to mint/teleport"]
					#[doc = "   assets and deposit them to `beneficiary`."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent,"]
					#[doc = "  Parachain(..))` to send from parachain to parachain, or `X1(Parachain(..))` to send"]
					#[doc = "  from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
					#[doc = "  generally be an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
					#[doc = "  fee on the `dest` (and possibly reserve) chains."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					transfer_assets {
						dest:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						beneficiary:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						assets:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 12)]
					#[doc = "Claims assets trapped on this pallet because of leftover assets during XCM execution."]
					#[doc = ""]
					#[doc = "- `origin`: Anyone can call this extrinsic."]
					#[doc = "- `assets`: The exact assets that were trapped. Use the version to specify what version"]
					#[doc = "was the latest when they were trapped."]
					#[doc = "- `beneficiary`: The location/account where the claimed assets will be deposited."]
					claim_assets {
						assets:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedAssets>,
						beneficiary:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
					},
					#[codec(index = 13)]
					#[doc = "Transfer assets from the local chain to the destination chain using explicit transfer"]
					#[doc = "types for assets and fees."]
					#[doc = ""]
					#[doc = "`assets` must have same reserve location or may be teleportable to `dest`. Caller must"]
					#[doc = "provide the `assets_transfer_type` to be used for `assets`:"]
					#[doc = " - `TransferType::LocalReserve`: transfer assets to sovereign account of destination"]
					#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
					#[doc = "   assets to `beneficiary`."]
					#[doc = " - `TransferType::DestinationReserve`: burn local assets and forward a notification to"]
					#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
					#[doc = "   deposit them to `beneficiary`."]
					#[doc = " - `TransferType::RemoteReserve(reserve)`: burn local assets, forward XCM to `reserve`"]
					#[doc = "   chain to move reserves from this chain's SA to `dest` chain's SA, and forward another"]
					#[doc = "   XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`. Typically"]
					#[doc = "   the remote `reserve` is Asset Hub."]
					#[doc = " - `TransferType::Teleport`: burn local assets and forward XCM to `dest` chain to"]
					#[doc = "   mint/teleport assets and deposit them to `beneficiary`."]
					#[doc = ""]
					#[doc = "On the destination chain, as well as any intermediary hops, `BuyExecution` is used to"]
					#[doc = "buy execution using transferred `assets` identified by `remote_fees_id`."]
					#[doc = "Make sure enough of the specified `remote_fees_id` asset is included in the given list"]
					#[doc = "of `assets`. `remote_fees_id` should be enough to pay for `weight_limit`. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
					#[doc = "at risk."]
					#[doc = ""]
					#[doc = "`remote_fees_id` may use different transfer type than rest of `assets` and can be"]
					#[doc = "specified through `fees_transfer_type`."]
					#[doc = ""]
					#[doc = "The caller needs to specify what should happen to the transferred assets once they reach"]
					#[doc = "the `dest` chain. This is done through the `custom_xcm_on_dest` parameter, which"]
					#[doc = "contains the instructions to execute on `dest` as a final step."]
					#[doc = "  This is usually as simple as:"]
					#[doc = "  `Xcm(vec![DepositAsset { assets: Wild(AllCounted(assets.len())), beneficiary }])`,"]
					#[doc = "  but could be something more exotic like sending the `assets` even further."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
					#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
					#[doc = "  relay to parachain, or `(parents: 2, (GlobalConsensus(..), ..))` to send from"]
					#[doc = "  parachain across a bridge to another ecosystem destination."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
					#[doc = "  fee on the `dest` (and possibly reserve) chains."]
					#[doc = "- `assets_transfer_type`: The XCM `TransferType` used to transfer the `assets`."]
					#[doc = "- `remote_fees_id`: One of the included `assets` to be be used to pay fees."]
					#[doc = "- `fees_transfer_type`: The XCM `TransferType` used to transfer the `fees` assets."]
					#[doc = "- `custom_xcm_on_dest`: The XCM to be executed on `dest` chain as the last step of the"]
					#[doc = "  transfer, which also determines what happens to the assets on the destination chain."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					transfer_assets_using_type_and_then {
						dest:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedLocation>,
						assets:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedAssets>,
						assets_transfer_type: ::subxt::ext::subxt_core::alloc::boxed::Box<
							runtime_types::staging_xcm_executor::traits::asset_transfer::TransferType,
						>,
						remote_fees_id:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedAssetId>,
						fees_transfer_type: ::subxt::ext::subxt_core::alloc::boxed::Box<
							runtime_types::staging_xcm_executor::traits::asset_transfer::TransferType,
						>,
						custom_xcm_on_dest:
							::subxt::ext::subxt_core::alloc::boxed::Box<runtime_types::xcm::VersionedXcm>,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
					#[doc = "to it."]
					Unreachable,
					#[codec(index = 1)]
					#[doc = "There was some other issue (i.e. not to do with routing) in sending the message."]
					#[doc = "Perhaps a lack of space for buffering the message."]
					SendFailure,
					#[codec(index = 2)]
					#[doc = "The message execution fails the filter."]
					Filtered,
					#[codec(index = 3)]
					#[doc = "The message's weight could not be determined."]
					UnweighableMessage,
					#[codec(index = 4)]
					#[doc = "The destination `Location` provided cannot be inverted."]
					DestinationNotInvertible,
					#[codec(index = 5)]
					#[doc = "The assets to be sent are empty."]
					Empty,
					#[codec(index = 6)]
					#[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
					CannotReanchor,
					#[codec(index = 7)]
					#[doc = "Too many assets have been attempted for transfer."]
					TooManyAssets,
					#[codec(index = 8)]
					#[doc = "Origin is invalid for sending."]
					InvalidOrigin,
					#[codec(index = 9)]
					#[doc = "The version of the `Versioned` value used is not able to be interpreted."]
					BadVersion,
					#[codec(index = 10)]
					#[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
					#[doc = "desired version of XCM)."]
					BadLocation,
					#[codec(index = 11)]
					#[doc = "The referenced subscription could not be found."]
					NoSubscription,
					#[codec(index = 12)]
					#[doc = "The location is invalid since it already has a subscription from us."]
					AlreadySubscribed,
					#[codec(index = 13)]
					#[doc = "Could not check-out the assets for teleportation to the destination chain."]
					CannotCheckOutTeleport,
					#[codec(index = 14)]
					#[doc = "The owner does not own (all) of the asset that they wish to do the operation on."]
					LowBalance,
					#[codec(index = 15)]
					#[doc = "The asset owner has too many locks on the asset."]
					TooManyLocks,
					#[codec(index = 16)]
					#[doc = "The given account is not an identifiable sovereign account for any location."]
					AccountNotSovereign,
					#[codec(index = 17)]
					#[doc = "The operation required fees to be paid which the initiator could not meet."]
					FeesNotMet,
					#[codec(index = 18)]
					#[doc = "A remote lock with the corresponding data could not be found."]
					LockNotFound,
					#[codec(index = 19)]
					#[doc = "The unlock operation cannot succeed because there are still consumers of the lock."]
					InUse,
					#[codec(index = 21)]
					#[doc = "Invalid asset, reserve chain could not be determined for it."]
					InvalidAssetUnknownReserve,
					#[codec(index = 22)]
					#[doc = "Invalid asset, do not support remote asset reserves with different fees reserves."]
					InvalidAssetUnsupportedReserve,
					#[codec(index = 23)]
					#[doc = "Too many assets with different reserve locations have been attempted for transfer."]
					TooManyReserves,
					#[codec(index = 24)]
					#[doc = "Local XCM execution incomplete."]
					LocalExecutionIncomplete,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Execution of an XCM message was attempted."]
					Attempted {
						outcome: runtime_types::staging_xcm::v4::traits::Outcome,
					},
					#[codec(index = 1)]
					#[doc = "A XCM message was sent."]
					Sent {
						origin: runtime_types::staging_xcm::v4::location::Location,
						destination: runtime_types::staging_xcm::v4::location::Location,
						message: runtime_types::staging_xcm::v4::Xcm,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 2)]
					#[doc = "Query response received which does not match a registered query. This may be because a"]
					#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
					#[doc = "because the query timed out."]
					UnexpectedResponse {
						origin: runtime_types::staging_xcm::v4::location::Location,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 3)]
					#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
					#[doc = "no registered notification call."]
					ResponseReady {
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v4::Response,
					},
					#[codec(index = 4)]
					#[doc = "Query response has been received and query is removed. The registered notification has"]
					#[doc = "been dispatched and executed successfully."]
					Notified {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 5)]
					#[doc = "Query response has been received and query is removed. The registered notification"]
					#[doc = "could not be dispatched because the dispatch weight is greater than the maximum weight"]
					#[doc = "originally budgeted by this runtime for the query result."]
					NotifyOverweight {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
						actual_weight: runtime_types::sp_weights::weight_v2::Weight,
						max_budgeted_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 6)]
					#[doc = "Query response has been received and query is removed. There was a general error with"]
					#[doc = "dispatching the notification call."]
					NotifyDispatchError {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 7)]
					#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
					#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
					#[doc = "is not `(origin, QueryId, Response)`."]
					NotifyDecodeFailed {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 8)]
					#[doc = "Expected query response has been received but the origin location of the response does"]
					#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
					#[doc = "be received and acted upon."]
					InvalidResponder {
						origin: runtime_types::staging_xcm::v4::location::Location,
						query_id: ::core::primitive::u64,
						expected_location:
							::core::option::Option<runtime_types::staging_xcm::v4::location::Location>,
					},
					#[codec(index = 9)]
					#[doc = "Expected query response has been received but the expected origin location placed in"]
					#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
					#[doc = ""]
					#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
					#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
					#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
					#[doc = "needed."]
					InvalidResponderVersion {
						origin: runtime_types::staging_xcm::v4::location::Location,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 10)]
					#[doc = "Received query response has been read and removed."]
					ResponseTaken { query_id: ::core::primitive::u64 },
					#[codec(index = 11)]
					#[doc = "Some assets have been placed in an asset trap."]
					AssetsTrapped {
						hash: ::subxt::ext::subxt_core::utils::H256,
						origin: runtime_types::staging_xcm::v4::location::Location,
						assets: runtime_types::xcm::VersionedAssets,
					},
					#[codec(index = 12)]
					#[doc = "An XCM version change notification message has been attempted to be sent."]
					#[doc = ""]
					#[doc = "The cost of sending it (borne by the chain) is included."]
					VersionChangeNotified {
						destination: runtime_types::staging_xcm::v4::location::Location,
						result: ::core::primitive::u32,
						cost: runtime_types::staging_xcm::v4::asset::Assets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 13)]
					#[doc = "The supported version of a location has been changed. This might be through an"]
					#[doc = "automatic notification or a manual intervention."]
					SupportedVersionChanged {
						location: runtime_types::staging_xcm::v4::location::Location,
						version: ::core::primitive::u32,
					},
					#[codec(index = 14)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "sending the notification to it."]
					NotifyTargetSendFail {
						location: runtime_types::staging_xcm::v4::location::Location,
						query_id: ::core::primitive::u64,
						error: runtime_types::xcm::v3::traits::Error,
					},
					#[codec(index = 15)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "migrating the location to our new XCM format."]
					NotifyTargetMigrationFail {
						location: runtime_types::xcm::VersionedLocation,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 16)]
					#[doc = "Expected query response has been received but the expected querier location placed in"]
					#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
					#[doc = ""]
					#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
					#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
					#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
					#[doc = "needed."]
					InvalidQuerierVersion {
						origin: runtime_types::staging_xcm::v4::location::Location,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 17)]
					#[doc = "Expected query response has been received but the querier location of the response does"]
					#[doc = "not match the expected. The query remains registered for a later, valid, response to"]
					#[doc = "be received and acted upon."]
					InvalidQuerier {
						origin: runtime_types::staging_xcm::v4::location::Location,
						query_id: ::core::primitive::u64,
						expected_querier: runtime_types::staging_xcm::v4::location::Location,
						maybe_actual_querier:
							::core::option::Option<runtime_types::staging_xcm::v4::location::Location>,
					},
					#[codec(index = 18)]
					#[doc = "A remote has requested XCM version change notification from us and we have honored it."]
					#[doc = "A version information message is sent to them and its cost is included."]
					VersionNotifyStarted {
						destination: runtime_types::staging_xcm::v4::location::Location,
						cost: runtime_types::staging_xcm::v4::asset::Assets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 19)]
					#[doc = "We have requested that a remote chain send us XCM version change notifications."]
					VersionNotifyRequested {
						destination: runtime_types::staging_xcm::v4::location::Location,
						cost: runtime_types::staging_xcm::v4::asset::Assets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 20)]
					#[doc = "We have requested that a remote chain stops sending us XCM version change"]
					#[doc = "notifications."]
					VersionNotifyUnrequested {
						destination: runtime_types::staging_xcm::v4::location::Location,
						cost: runtime_types::staging_xcm::v4::asset::Assets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 21)]
					#[doc = "Fees were paid from a location for an operation (often for using `SendXcm`)."]
					FeesPaid {
						paying: runtime_types::staging_xcm::v4::location::Location,
						fees: runtime_types::staging_xcm::v4::asset::Assets,
					},
					#[codec(index = 22)]
					#[doc = "Some assets have been claimed from an asset trap"]
					AssetsClaimed {
						hash: ::subxt::ext::subxt_core::utils::H256,
						origin: runtime_types::staging_xcm::v4::location::Location,
						assets: runtime_types::xcm::VersionedAssets,
					},
					#[codec(index = 23)]
					#[doc = "A XCM version migration finished."]
					VersionMigrationFinished { version: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum QueryStatus<_0> {
					#[codec(index = 0)]
					Pending {
						responder: runtime_types::xcm::VersionedLocation,
						maybe_match_querier: ::core::option::Option<runtime_types::xcm::VersionedLocation>,
						maybe_notify: ::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
						timeout: _0,
					},
					#[codec(index = 1)]
					VersionNotifier {
						origin: runtime_types::xcm::VersionedLocation,
						is_active: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					Ready {
						response: runtime_types::xcm::VersionedResponse,
						at: _0,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct RemoteLockedFungibleRecord<_0> {
					pub amount: ::core::primitive::u128,
					pub owner: runtime_types::xcm::VersionedLocation,
					pub locker: runtime_types::xcm::VersionedLocation,
					pub consumers: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						_0,
						::core::primitive::u128,
					)>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum VersionMigrationStage {
					#[codec(index = 0)]
					MigrateSupportedVersion,
					#[codec(index = 1)]
					MigrateVersionNotifiers,
					#[codec(index = 2)]
					NotifyCurrentTargets(
						::core::option::Option<
							::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						>,
					),
					#[codec(index = 3)]
					MigrateAndNotifyOldTargets,
				}
			}
		}
		pub mod pallet_zk_verifier {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Store a verification key."]
					setup_verification {
						task_id: ::core::primitive::u64,
						pub_input: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						vec_vk: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Verify a proof."]
					verify {
						task_id: ::core::primitive::u64,
						vec_proof: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Public inputs mismatch"]
					PublicInputsMismatch,
					#[codec(index = 1)]
					#[doc = "Public inputs vector is to long."]
					TooLongPublicInputs,
					#[codec(index = 2)]
					#[doc = "The verification key is to long."]
					TooLongVerificationKey,
					#[codec(index = 3)]
					#[doc = "The proof is too long."]
					TooLongProof,
					#[codec(index = 4)]
					#[doc = "The proof is too short."]
					ProofIsEmpty,
					#[codec(index = 5)]
					#[doc = "Verification key, not set."]
					VerificationKeyIsNotSet,
					#[codec(index = 6)]
					#[doc = "Malformed key"]
					MalformedVerificationKey,
					#[codec(index = 7)]
					#[doc = "Malformed proof"]
					MalformedProof,
					#[codec(index = 8)]
					#[doc = "Malformed public inputs"]
					MalformedPublicInputs,
					#[codec(index = 9)]
					#[doc = "Curve is not supported"]
					NotSupportedCurve,
					#[codec(index = 10)]
					#[doc = "Protocol is not supported"]
					NotSupportedProtocol,
					#[codec(index = 11)]
					#[doc = "There was error during proof verification"]
					ProofVerificationError,
					#[codec(index = 12)]
					#[doc = "Proof creation error"]
					ProofCreationError,
					#[codec(index = 13)]
					#[doc = "Verification Key creation error"]
					VerificationKeyCreationError,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					VerificationSetupCompleted,
					#[codec(index = 1)]
					VerificationProofSet,
					#[codec(index = 2)]
					VerificationSuccess {
						who: ::subxt::ext::subxt_core::utils::AccountId32,
					},
					#[codec(index = 3)]
					VerificationFailed,
				}
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain_primitives {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct HeadData(pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Id(pub ::core::primitive::u32);
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v7 {
				use super::runtime_types;
				pub mod async_backing {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct AsyncBackingParams {
						pub max_candidate_depth: ::core::primitive::u32,
						pub allowed_ancestry_len: ::core::primitive::u32,
					}
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct AbridgedHostConfiguration {
					pub max_code_size: ::core::primitive::u32,
					pub max_head_data_size: ::core::primitive::u32,
					pub max_upward_queue_count: ::core::primitive::u32,
					pub max_upward_queue_size: ::core::primitive::u32,
					pub max_upward_message_size: ::core::primitive::u32,
					pub max_upward_message_num_per_candidate: ::core::primitive::u32,
					pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
					pub validation_upgrade_cooldown: ::core::primitive::u32,
					pub validation_upgrade_delay: ::core::primitive::u32,
					pub async_backing_params:
						runtime_types::polkadot_primitives::v7::async_backing::AsyncBackingParams,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::ext::subxt_core::utils::H256>,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain_primitives::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum UpgradeGoAhead {
					#[codec(index = 0)]
					Abort,
					#[codec(index = 1)]
					GoAhead,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_consensus_aura {
			use super::runtime_types;
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct Public(pub [::core::primitive::u8; 32usize]);
				}
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct Digest {
						pub logs: ::subxt::ext::subxt_core::alloc::vec::Vec<
							runtime_types::sp_runtime::generic::digest::DigestItem,
						>,
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519([::core::primitive::u8; 64usize]),
				#[codec(index = 1)]
				Sr25519([::core::primitive::u8; 64usize]),
				#[codec(index = 2)]
				Ecdsa([::core::primitive::u8; 65usize]),
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_trie {
			use super::runtime_types;
			pub mod storage_proof {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct StorageProof {
					pub trie_nodes: ::subxt::ext::subxt_core::alloc::vec::Vec<
						::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
					>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct RuntimeVersion {
				pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
				pub impl_name: ::subxt::ext::subxt_core::alloc::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis: ::subxt::ext::subxt_core::alloc::vec::Vec<(
					[::core::primitive::u8; 8usize],
					::core::primitive::u32,
				)>,
				pub transaction_version: ::core::primitive::u32,
				pub state_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
		pub mod staging_parachain_info {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
			}
		}
		pub mod staging_xcm {
			use super::runtime_types;
			pub mod v3 {
				use super::runtime_types;
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v3::junctions::Junctions,
					}
				}
			}
			pub mod v4 {
				use super::runtime_types;
				pub mod asset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct Asset {
						pub id: runtime_types::staging_xcm::v4::asset::AssetId,
						pub fun: runtime_types::staging_xcm::v4::asset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum AssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::staging_xcm::v4::asset::Assets),
						#[codec(index = 1)]
						Wild(runtime_types::staging_xcm::v4::asset::WildAsset),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct AssetId(pub runtime_types::staging_xcm::v4::location::Location);
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct Assets(
						pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
							runtime_types::staging_xcm::v4::asset::Asset,
						>,
					);
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::staging_xcm::v4::asset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum WildAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::staging_xcm::v4::asset::AssetId,
							fun: runtime_types::staging_xcm::v4::asset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::staging_xcm::v4::asset::AssetId,
							fun: runtime_types::staging_xcm::v4::asset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
				}
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: ::core::option::Option<runtime_types::staging_xcm::v4::junction::NetworkId>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: ::core::option::Option<runtime_types::staging_xcm::v4::junction::NetworkId>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: ::core::option::Option<runtime_types::staging_xcm::v4::junction::NetworkId>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::staging_xcm::v4::junction::NetworkId),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 4)]
						Westend,
						#[codec(index = 5)]
						Rococo,
						#[codec(index = 6)]
						Wococo,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
						#[codec(index = 10)]
						PolkadotBulletin,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1([runtime_types::staging_xcm::v4::junction::Junction; 1usize]),
						#[codec(index = 2)]
						X2([runtime_types::staging_xcm::v4::junction::Junction; 2usize]),
						#[codec(index = 3)]
						X3([runtime_types::staging_xcm::v4::junction::Junction; 3usize]),
						#[codec(index = 4)]
						X4([runtime_types::staging_xcm::v4::junction::Junction; 4usize]),
						#[codec(index = 5)]
						X5([runtime_types::staging_xcm::v4::junction::Junction; 5usize]),
						#[codec(index = 6)]
						X6([runtime_types::staging_xcm::v4::junction::Junction; 6usize]),
						#[codec(index = 7)]
						X7([runtime_types::staging_xcm::v4::junction::Junction; 7usize]),
						#[codec(index = 8)]
						X8([runtime_types::staging_xcm::v4::junction::Junction; 8usize]),
					}
				}
				pub mod location {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct Location {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::staging_xcm::v4::junctions::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete {
							used: runtime_types::sp_weights::weight_v2::Weight,
						},
						#[codec(index = 1)]
						Incomplete {
							used: runtime_types::sp_weights::weight_v2::Weight,
							error: runtime_types::xcm::v3::traits::Error,
						},
						#[codec(index = 2)]
						Error {
							error: runtime_types::xcm::v3::traits::Error,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v4::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<runtime_types::staging_xcm::v4::location::Location>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						beneficiary: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v3::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::staging_xcm::v4::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::staging_xcm::v4::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						beneficiary: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::staging_xcm::v4::asset::AssetFilter,
						want: runtime_types::staging_xcm::v4::asset::Assets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						reserve: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::staging_xcm::v4::QueryResponseInfo,
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::staging_xcm::v4::asset::Asset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::staging_xcm::v4::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::staging_xcm::v4::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						ticket: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 30)]
					ExpectOrigin(::core::option::Option<runtime_types::staging_xcm::v4::location::Location>),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::staging_xcm::v4::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						module_name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::staging_xcm::v4::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::staging_xcm::v4::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::staging_xcm::v4::junction::NetworkId,
						destination: runtime_types::staging_xcm::v4::junctions::Junctions,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						unlocker: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						target: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						owner: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						locker: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 43)]
					SetFeesMode {
						jit_withdraw: ::core::primitive::bool,
					},
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v4::location::Location),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin:
							::core::option::Option<runtime_types::staging_xcm::v4::location::Location>,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					pub module_name:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::staging_xcm::v4::location::Location,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::staging_xcm::v4::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Xcm(
					pub ::subxt::ext::subxt_core::alloc::vec::Vec<runtime_types::staging_xcm::v4::Instruction>,
				);
			}
		}
		pub mod staging_xcm_executor {
			use super::runtime_types;
			pub mod traits {
				use super::runtime_types;
				pub mod asset_transfer {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum TransferType {
						#[codec(index = 0)]
						Teleport,
						#[codec(index = 1)]
						LocalReserve,
						#[codec(index = 2)]
						DestinationReserve,
						#[codec(index = 3)]
						RemoteReserve(runtime_types::xcm::VersionedLocation),
					}
				}
			}
		}
		pub mod xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct DoubleEncoded {
					pub encoded: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: runtime_types::xcm::v2::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::xcm::v2::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: runtime_types::xcm::v2::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey(
							runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v2::BodyId,
							part: runtime_types::xcm::v2::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v2::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
						#[codec(index = 6)]
						Blob(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v2::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v2::multiasset::AssetId,
						pub fun: runtime_types::xcm::v2::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v2::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v2::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct MultiAssets(
						pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
							runtime_types::xcm::v2::multiasset::MultiAsset,
						>,
					);
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v2::multiasset::AssetId,
							fun: runtime_types::xcm::v2::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v2::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v2::multilocation::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						MultiLocationFull,
						#[codec(index = 5)]
						MultiLocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						UnhandledXcmVersion,
						#[codec(index = 23)]
						WeightLimitReached(::core::primitive::u64),
						#[codec(index = 24)]
						Barrier,
						#[codec(index = 25)]
						WeightNotComputable,
					}
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum BodyId {
					#[codec(index = 0)]
					Unit,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Index(#[codec(compact)] ::core::primitive::u32),
					#[codec(index = 3)]
					Executive,
					#[codec(index = 4)]
					Technical,
					#[codec(index = 5)]
					Legislative,
					#[codec(index = 6)]
					Judicial,
					#[codec(index = 7)]
					Defense,
					#[codec(index = 8)]
					Administration,
					#[codec(index = 9)]
					Treasury,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum BodyPart {
					#[codec(index = 0)]
					Voice,
					#[codec(index = 1)]
					Members {
						#[codec(compact)]
						count: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Fraction {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					AtLeastProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					MoreThanProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v2::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v2::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum NetworkId {
					#[codec(index = 0)]
					Any,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Polkadot,
					#[codec(index = 3)]
					Kusama,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v2::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Xcm(
					pub ::subxt::ext::subxt_core::alloc::vec::Vec<runtime_types::xcm::v2::Instruction>,
				);
			}
			pub mod v3 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Moniker([::core::primitive::u8; 4usize]),
						#[codec(index = 2)]
						Index(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						Executive,
						#[codec(index = 4)]
						Technical,
						#[codec(index = 5)]
						Legislative,
						#[codec(index = 6)]
						Judicial,
						#[codec(index = 7)]
						Defense,
						#[codec(index = 8)]
						Administration,
						#[codec(index = 9)]
						Treasury,
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum BodyPart {
						#[codec(index = 0)]
						Voice,
						#[codec(index = 1)]
						Members {
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec(index = 2)]
						Fraction {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						AtLeastProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						MoreThanProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 4)]
						Westend,
						#[codec(index = 5)]
						Rococo,
						#[codec(index = 6)]
						Wococo,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
						#[codec(index = 10)]
						PolkadotBulletin,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v3::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v3::multiasset::AssetId,
						pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub struct MultiAssets(
						pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
							runtime_types::xcm::v3::multiasset::MultiAsset,
						>,
					);
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
						:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
						:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
					#[codec(dumb_trait_bound)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						LocationFull,
						#[codec(index = 5)]
						LocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						ExpectationFalse,
						#[codec(index = 23)]
						PalletNotFound,
						#[codec(index = 24)]
						NameMismatch,
						#[codec(index = 25)]
						VersionIncompatible,
						#[codec(index = 26)]
						HoldingWouldOverflow,
						#[codec(index = 27)]
						ExportError,
						#[codec(index = 28)]
						ReanchorFailed,
						#[codec(index = 29)]
						NoDeal,
						#[codec(index = 30)]
						FeesNotMet,
						#[codec(index = 31)]
						LockError,
						#[codec(index = 32)]
						NoPermission,
						#[codec(index = 33)]
						Unanchored,
						#[codec(index = 34)]
						NotDepositable,
						#[codec(index = 35)]
						UnhandledXcmVersion,
						#[codec(index = 36)]
						WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 37)]
						Barrier,
						#[codec(index = 38)]
						WeightNotComputable,
						#[codec(index = 39)]
						ExceedsStackLimit,
					}
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier:
							::core::option::Option<runtime_types::staging_xcm::v3::multilocation::MultiLocation>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v3::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<runtime_types::staging_xcm::v3::multilocation::MultiLocation>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						module_name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode {
						jit_withdraw: ::core::primitive::bool,
					},
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin:
							::core::option::Option<runtime_types::staging_xcm::v3::multilocation::MultiLocation>,
					},
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum MaybeErrorCode {
					#[codec(index = 0)]
					Success,
					#[codec(index = 1)]
					Error(runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>),
					#[codec(index = 2)]
					TruncatedError(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					),
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					pub module_name:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::xcm::v3::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(runtime_types::sp_weights::weight_v2::Weight),
				}
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
				pub struct Xcm(
					pub ::subxt::ext::subxt_core::alloc::vec::Vec<runtime_types::xcm::v3::Instruction>,
				);
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum VersionedAssetId {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::AssetId),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::asset::AssetId),
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum VersionedAssets {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multiasset::MultiAssets),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAssets),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::asset::Assets),
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum VersionedLocation {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multilocation::MultiLocation),
				#[codec(index = 3)]
				V3(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::location::Location),
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum VersionedResponse {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Response),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Response),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::Response),
			}
			#[derive(
				:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
				:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
				:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
			#[codec(dumb_trait_bound)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
			pub enum VersionedXcm {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::Xcm),
			}
		}
	}
}
