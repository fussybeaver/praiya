#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
/*
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::de::{DeserializeOwned, Deserializer};
use serde::ser::Serializer;
use serde::Deserialize;
use serde_json::value::Value;

use std::cmp::Eq;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;

use chrono::DateTime;
use chrono::offset::FixedOffset;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct UpdateConnection {
    pub slack_connection: SlackConnection,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct Error {
    pub error: ErrorError,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct ErrorError {
    /// Error message string.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct Pagination {
    /// Echoes offset pagination property.
    pub offset: isize,
    /// Echoes limit pagination property.
    pub limit: u8,
    /// Indicates if there are additional records to return.
    pub more: bool,
    /// The total number of records matching the given query.
    pub total: isize,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct SlackConnection {
    pub id: Option<String>,
    pub source_id: Option<String>,
    pub source_name: Option<String>,
    pub source_type: Option<SlackConnectionSourceType>,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub notification_type: Option<SlackConnectionNotifiationType>,
    pub config: Option<SlackConnectionConfig>,
}

/// Item of the Slack connection that contains configuration options (filters).
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct SlackConnectionConfig {
    pub events: Option<SlackConnectionEvents>,
    pub urgency: Option<SlackConnectionUrgency>,
    pub priorities: Option<SlackConnectionPriorities>,
}

/// Configuration item of the Slack connection allows you to subscribe to different type of events.
pub type SlackConnectionEvents = Vec<String>;

/// Configuration item of the Slack connection that indicates how Slack message will be formatted (can be `responder` or `stakeholder`)
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum SlackConnectionNotifiationType {
    #[serde(rename = "responder")]
    RESPONDER,
    #[serde(rename = "stakeholder")]
    STAKEHOLDER,
}

impl Display for SlackConnectionNotifiationType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SlackConnectionNotifiationType::RESPONDER => write!(f, "responder"),
            SlackConnectionNotifiationType::STAKEHOLDER => write!(f, "stakeholder"),
        }
    }
}

impl std::str::FromStr for SlackConnectionNotifiationType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "responder" => Ok(SlackConnectionNotifiationType::RESPONDER),
            "stakeholder" => Ok(SlackConnectionNotifiationType::STAKEHOLDER),
            _ => Err(()),
        }
    }
}

/// Configuration item of the Slack connection allows you to filter events by priorities.
pub type SlackConnectionPriorities = Vec<String>;

/// Configuration item of the Slack connection that indicates what PagerDuty source would be used for the connection (can be Service or Team).
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum SlackConnectionSourceType {
    #[serde(rename = "service_reference")]
    SERVICE_REFERENCE,
    #[serde(rename = "team_reference")]
    TEAM_REFERENCE,
}

impl Display for SlackConnectionSourceType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SlackConnectionSourceType::SERVICE_REFERENCE => write!(f, "service_reference"),
            SlackConnectionSourceType::TEAM_REFERENCE => write!(f, "team_reference"),
        }
    }
}

impl std::str::FromStr for SlackConnectionSourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "service_reference" => Ok(SlackConnectionSourceType::SERVICE_REFERENCE),
            "team_reference" => Ok(SlackConnectionSourceType::TEAM_REFERENCE),
            _ => Err(()),
        }
    }
}

/// Configuration item of the Slack connection allows you to filter events by urgency.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum SlackConnectionUrgency {
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "null")]
    NULL,
}

impl Display for SlackConnectionUrgency {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SlackConnectionUrgency::HIGH => write!(f, "high"),
            SlackConnectionUrgency::LOW => write!(f, "low"),
            SlackConnectionUrgency::NULL => write!(f, "null"),
        }
    }
}

impl std::str::FromStr for SlackConnectionUrgency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "high" => Ok(SlackConnectionUrgency::HIGH),
            "low" => Ok(SlackConnectionUrgency::LOW),
            "null" => Ok(SlackConnectionUrgency::NULL),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct CreateConnection {
    pub slack_connection: SlackConnection,
}
