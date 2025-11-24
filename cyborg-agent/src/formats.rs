//! Types to aid serialization with serde

use std::{fmt, str::FromStr};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Serialize, Deserialize)]
#[serde(remote = "uuid::Uuid")]
pub struct SerdeUuid(#[serde(getter = "uuid::Uuid::to_string")] String);

impl From<SerdeUuid> for uuid::Uuid {
    fn from(def: SerdeUuid) -> uuid::Uuid {
        uuid::Uuid::parse_str(&def.0).unwrap()
    }
}

pub struct OptionalUuid(pub Option<uuid::Uuid>);

struct UuidVisitor;

impl<'de> Visitor<'de> for UuidVisitor {
    type Value = OptionalUuid;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an empty string or uuid")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "" {
            Ok(OptionalUuid(None))
        } else {
            uuid::Uuid::parse_str(&v)
                .map(|uuid| OptionalUuid(Some(uuid)))
                .map_err(E::custom)
        }
    }
}

impl<'de> Deserialize<'de> for OptionalUuid {
    fn deserialize<D>(deserializer: D) -> Result<OptionalUuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(UuidVisitor)
    }
}

impl Serialize for OptionalUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let OptionalUuid(Some(uuid)) = self {
            serializer.serialize_str(&uuid.to_string())
        } else {
            serializer.serialize_str("")
        }
    }
}

pub struct OptionalStatusCode(pub Option<http::StatusCode>);

struct StatusCodeVisitor;

impl<'de> Visitor<'de> for StatusCodeVisitor {
    type Value = OptionalStatusCode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an empty string or status code")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "" {
            Ok(OptionalStatusCode(None))
        } else {
            http::StatusCode::from_str(&v)
                .map(|status_code| OptionalStatusCode(Some(status_code)))
                .map_err(E::custom)
        }
    }
}

impl<'de> Deserialize<'de> for OptionalStatusCode {
    fn deserialize<D>(deserializer: D) -> Result<OptionalStatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StatusCodeVisitor)
    }
}

impl Serialize for OptionalStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let OptionalStatusCode(Some(status_code)) = self {
            serializer.serialize_str(&status_code.as_str())
        } else {
            serializer.serialize_str("")
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "http::StatusCode")]
pub struct SerdeStatusCode(#[serde(getter = "http::StatusCode::to_string")] String);

impl From<SerdeStatusCode> for http::StatusCode {
    fn from(def: SerdeStatusCode) -> http::StatusCode {
        http::StatusCode::from_str(&def.0).unwrap()
    }
}
