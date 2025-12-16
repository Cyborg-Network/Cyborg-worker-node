use derive_more::From;

/// A type alias for a `Result` with the custom error enum `Error`.
pub type Result<T> = core::result::Result<T, Error>;

/// The custom error enum for the cyborg worker. This error enum covers all of the error variants that can occur,
/// enabling all errors to be handled with the `?` operator, but not preventing handling the errors more precisely.
#[derive(Debug, From)]
pub enum Error {
    #[from]
    #[allow(dead_code)]
    Custom(String),

    // -- Externals
    #[from]
    #[allow(dead_code)]
    Io(std::io::Error),

    #[from]
    #[allow(dead_code)]
    Serialization(serde_json::Error),

    #[from]
    #[allow(dead_code)]
    Reqwest(Box<reqwest::Error>),

    #[from]
    #[allow(dead_code)]
    Subxt(Box<subxt::Error>),

    #[from]
    #[allow(dead_code)]
    Conversion(std::string::FromUtf8Error),

    #[from]
    #[allow(dead_code)]
    Cess(Box<cess_rust_sdk::core::Error>),

    #[from]
    ReqwestToStr(reqwest::header::ToStrError),

    #[from]
    #[allow(dead_code)]
    ReqwestParseInt(std::num::ParseIntError),

    #[from]
    #[allow(dead_code)]
    Bollard(Box<bollard::errors::Error>),
}

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }

    pub fn parachain_client_not_intitialized() -> Self {
        Self::Custom("Parachain client not initialized".to_string())
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(Box::new(err))
    }
}

impl From<subxt::Error> for Error {
    fn from(err: subxt::Error) -> Self {
        Self::Subxt(Box::new(err))
    }
}

impl From<cess_rust_sdk::core::Error> for Error {
    fn from(err: cess_rust_sdk::core::Error) -> Self {
        Self::Cess(Box::new(err))
    }
}

impl From<bollard::errors::Error> for Error {
    fn from(err: bollard::errors::Error) -> Self {
        Self::Bollard(Box::new(err))
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
