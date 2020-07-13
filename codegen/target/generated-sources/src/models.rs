#![allow(unused_imports, unused_qualifications, unused_extern_crates)]

use serde::de::{DeserializeOwned, Deserializer};
use serde::ser::Serializer;
use serde_json::value::Value;

use std::cmp::Eq;
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acknowledgement {
    /// Time at which the acknowledgement was created.
    #[serde(rename = "at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "acknowledger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledger: Option<Acknowledger>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acknowledger {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
/// A message containing information about a single PagerDuty action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    /// Uniquely identifies this outgoing webhook message; can be used for idempotency when processing the messages.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,

    /// The date/time when this message was was sent.
    #[serde(rename = "triggered_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Addon {
    /// The type of Add-on.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The name of the Add-on.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The source URL to display in a frame in the PagerDuty UI. HTTPS is required.
    #[serde(rename = "src")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddonReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    /// The date/time the alert was first triggered.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The current status of the alert.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The alert's de-duplication key.
    #[serde(rename = "alert_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_key: Option<String>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    #[serde(rename = "first_trigger_log_entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_trigger_log_entry: Option<LogEntryReference>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,

    /// Whether or not an alert is suppressed. Suppressed alerts are not created with a parent incident.
    #[serde(rename = "suppressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<bool>,

    /// The magnitude of the problem as reported by the monitoring tool.
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(rename = "integration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<Integration>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertCount {
    /// The count of triggered alerts
    #[serde(rename = "triggered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered: Option<i64>,

    /// The count of resolved alerts
    #[serde(rename = "resolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<i64>,

    /// The total count of alerts
    #[serde(rename = "all")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllOfWebhooksV1AssignedToObject {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The user's name.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The user's email address.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsIncidentMetrics {
    /// ID of the service.  Only included when aggregating by service.
    #[serde(rename = "service_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,

    /// Name of the service.  Only included when aggregating by service.
    #[serde(rename = "service_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,

    /// ID of the team.  Only included when aggregating by team.
    #[serde(rename = "team_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,

    /// Name of the team.  Only included when aggregating by team.
    #[serde(rename = "team_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,

    /// Mean time from when an incident was triggered until it was resolved.
    #[serde(rename = "mean_seconds_to_resolve")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_seconds_to_resolve: Option<i64>,

    /// Mean time between the start of an incident, and the first responder to acknowledge.
    #[serde(rename = "mean_seconds_to_first_ack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_seconds_to_first_ack: Option<i64>,

    /// Mean time between the start of an incident, and the first responder to acknowledge, or to accept responder request.
    #[serde(rename = "mean_seconds_to_engage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_seconds_to_engage: Option<i64>,

    /// Mean time between the start of an incident, and the last additional responder to acknowledge.  For incidents with one or less engaged users, this value is null.
    #[serde(rename = "mean_seconds_to_mobilize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_seconds_to_mobilize: Option<i64>,

    /// Mean number of users who engaged (acknowledged, accepted responder request) with an incident.
    #[serde(rename = "mean_engaged_user_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_engaged_user_count: Option<i64>,

    /// Total count of instances where an incident is escalated between responders assigned to an escalation policy.
    #[serde(rename = "total_escalation_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_escalation_count: Option<i64>,

    /// Total count of instances where an additional responder, who was not on-call for an incident, is added.
    #[serde(rename = "total_assignment_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_assignment_count: Option<i64>,

    /// Total number of unique interruptions during business hours. Business hour: 8am-6pm Mon-Fri, based on the user’s time zone.
    #[serde(rename = "total_business_hour_interruptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_business_hour_interruptions: Option<i64>,

    /// Total number of unique interruptions during sleep hours. Sleep hour: 10pm-8am every day, based on the user’s time zone.
    #[serde(rename = "total_sleep_hour_interruptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_sleep_hour_interruptions: Option<i64>,

    /// Total number of unique interruptions during off hours. Off hour: 6pm-10pm Mon-Fri and all day Sat-Sun, based on the user’s time zone.
    #[serde(rename = "total_off_hour_interruptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_off_hour_interruptions: Option<i64>,

    /// Total number of seconds incidents were snoozed.
    #[serde(rename = "total_snoozed_seconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_snoozed_seconds: Option<i64>,

    /// Total engaged time across all responders for incidents.  Engaged time is measured from the time a user engages with an incident (by acknowledging or accepting a responder request) until the incident is resolved.  This may include periods in which the incidents was snoozed.
    #[serde(rename = "total_engaged_seconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_engaged_seconds: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsRawIncident {
    /// Incident Id
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// ID of the team the incident was assigned to.
    #[serde(rename = "team_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,

    /// ID of the service that the incident triggered on.
    #[serde(rename = "service_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,

    /// Timestamp of when the incident was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Timestamp of when the incident was resolved.
    #[serde(rename = "resolved_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,

    /// Notification level
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    /// A major incident is defined as any incident that requires a coordinated response, often across multiple teams.  https://support.pagerduty.com/docs/operational-reviews#major-incidents
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<bool>,

    /// ID of the incident's priority level.
    #[serde(rename = "priority_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_id: Option<String>,

    /// The user-provided short name of the priority.
    #[serde(rename = "priority_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_name: Option<String>,

    /// The integer representation of the incident's priority level.
    #[serde(rename = "priority_order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_order: Option<i64>,

    /// Time from when incident triggered until it was resolved.
    #[serde(rename = "seconds_to_resolve")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_to_resolve: Option<i64>,

    /// Time between start of an incident, and the first responder to acknowledge.
    #[serde(rename = "seconds_to_first_ack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_to_first_ack: Option<i64>,

    /// Time between start of an incident, and the first responder to acknowledge, or to accept responder request.
    #[serde(rename = "seconds_to_engage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_to_engage: Option<i64>,

    /// Time between start of an incident, and the last additional responder to acknowledge.  If an incident has one or less responders, the value will be null.
    #[serde(rename = "seconds_to_mobilize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_to_mobilize: Option<i64>,

    /// Total engaged time across all responders for this incident.  Engaged time is measured from the time a user engages with an incident (by acknowledging or accepting a responder request) until the incident is resolved.  This may include periods in which the incident was snoozed.
    #[serde(rename = "engaged_seconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engaged_seconds: Option<i64>,

    /// Total number of users who engaged (acknowledged, accepted responder request) in the incident.
    #[serde(rename = "engaged_user_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engaged_user_count: Option<i64>,

    /// Total count of instances where an incident is escalated between responders assigned to an escalation policy.
    #[serde(rename = "escalation_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_count: Option<i64>,

    /// Total count of instances where an additional responder, who was not on-call for the incident, is added.
    #[serde(rename = "assignment_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_count: Option<i64>,

    /// Total number of unique interruptions during business hour. Business hour: 8am-6pm Mon-Fri, based on the user’s time zone.
    #[serde(rename = "business_hour_interruptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_hour_interruptions: Option<i64>,

    /// Total number of unique interruptions during sleep hour. Sleep hour: 10pm-8am every day, based on the user’s time zone.
    #[serde(rename = "sleep_hour_interruptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_hour_interruptions: Option<i64>,

    /// Total number of unique interruptions during off hour. Off hour: 6pm-10pm Mon-Fri and all day Sat-Sun, based on the user’s time zone.
    #[serde(rename = "off_hour_interruptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_hour_interruptions: Option<i64>,

    /// Total seconds the incident has been snoozed for.
    #[serde(rename = "snoozed_seconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snoozed_seconds: Option<i64>,
}
/// Accepts a set of filters to apply to the Incidents before aggregating.  Any incidents that do not match the included filters will be omitted from the results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsmetricsincidentsallFilters {
    /// Accepts an ISO8601 DateTime string.  Any incidents with a created_at less than this value will be omitted from the results.  The maximum supported time range in conjunction with created_at_end is one year.
    #[serde(rename = "created_at_start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_start: Option<String>,

    /// Accepts an ISO8601 DateTime string.  Any incidents with a created_at greater than or equal to this value will be omitted from the results.  The maximum supported time range in conjunction with created_at_start is one year.
    #[serde(rename = "created_at_end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_end: Option<String>,

    /// Any incidents whose urgency does not match the provided string will be omitted from the results.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    /// A boolean flag including whether results should contain only major incidents, or exclude major incidents. If no value is provided all incidents will be included. You can find more information on the major incident classification here: https://support.pagerduty.com/docs/operational-reviews#major-incidents
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<bool>,

    /// An array of team IDs. Only incidents related to these teams will be included in the results. Account must have the teams ability to use this parameter.  Any teams that the requestor does not have access to will be omitted from the results.  If omitted, all teams the requestor has access to will be included in the results.
    #[serde(rename = "team_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,

    /// An array of service IDs. Only incidents related to these services will be included in the results. If omitted, all services the requestor has access to will be included in the results.
    #[serde(rename = "service_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<String>>,

    /// Any incidents whose priority does not match the provided string will be omitted from the results
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}
/// Filters the result, only show incidents that match the conditions passed in the filter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsmetricsincidentsservicesFilters {
    /// Filters the result, showing only the incidents where the creation timestamp is greater than the filter value.
    #[serde(rename = "created_at_start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_start: Option<String>,

    /// Filters the result, showing only the incidents where the creation timestamp is less than the filter value.
    #[serde(rename = "created_at_end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_end: Option<String>,

    /// Filters the result, showing only the incidents where urgency matches the filter value.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    /// A major incident is defined as any incident that requires a coordinated response, often across multiple teams. https://support.pagerduty.com/docs/operational-reviews#major-incidents
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<bool>,

    /// An array of team IDs. Only results related to these teams will be returned. Account must have the teams ability to use this parameter.
    #[serde(rename = "team_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,

    /// An array of service IDs. Only results related to these services will be returned.
    #[serde(rename = "service_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<String>>,

    /// User defined priority name
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}
/// Filters the result, only show incidents that match the conditions passed in the filter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsrawincidentsFilters {
    /// Filters the result, showing only the incidents where the creation timestamp is greater than the filter value.
    #[serde(rename = "created_at_start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_start: Option<String>,

    /// Filters the result, showing only the incidents where the creation timestamp is less than the filter value.
    #[serde(rename = "created_at_end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_end: Option<String>,

    /// Filters the result, showing only the incidents where urgency matches the filter value.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    /// A major incident is defined as any incident that requires a coordinated response, often across multiple teams. https://support.pagerduty.com/docs/operational-reviews#major-incidents
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<bool>,

    /// An array of team IDs. Only results related to these teams will be returned. Account must have the teams ability to use this parameter.
    #[serde(rename = "team_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,

    /// An array of service IDs. Only results related to these services will be returned.
    #[serde(rename = "service_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<String>>,

    /// User defined priority name
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnyOfResponsePlayRespondersItems {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnyOfResponsePlaySubscribersItems {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiBasedIntegration {
    /// The name of this integration.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    /// The date/time when this integration was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorReference>,

    /// This is the unique key used to route events to this integration when received via the PagerDuty Events API.
    #[serde(rename = "integration_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_key: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignLogEntry {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Time at which the log entry was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    #[serde(rename = "agent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,

    /// Optional field containing a note, if one was included with the log entry.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Contexts to be included with the trigger such as links to graphs or images.
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<Context>>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,

    /// Will consist of references unless included
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,

    #[serde(rename = "event_details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_details: Option<LogEntryEventDetails>,

    #[serde(rename = "assigned_user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<UserReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    /// Time at which the assignment was created.
    #[serde(rename = "at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserReference>,
}
/// A JSON object containing data describing the alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    /// The type of the body.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Contexts to be included with the body such as links to graphs or images.
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<Context>>,

    /// An arbitrary JSON object containing any data explaining the nature of the alert.
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<(), ()>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body1 {
    #[serde(rename = "addon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<Addon>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body10 {
    #[serde(rename = "escalation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policy: Option<EscalationPolicy>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body11 {
    #[serde(rename = "extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<Extension>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body12 {
    #[serde(rename = "extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<Extension>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body13 {
    /// An array of incidents, including the parameters to update.
    #[serde(rename = "incidents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incidents: Option<Vec<IncidentsIncidents>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body14 {
    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentsIncident>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body15 {
    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentsidIncident>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body16 {
    /// An array of alerts, including the parameters to update for each alert.
    #[serde(rename = "alerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<Alert>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body17 {
    #[serde(rename = "alert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<Alert>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body18 {
    /// The source incidents that will be merged into the target incident and resolved.
    #[serde(rename = "source_incidents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_incidents: Option<Vec<IncidentReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body19 {
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<IncidentNote>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body2 {
    #[serde(rename = "addon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<Addon>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body20 {
    /// The user id of the requester.
    #[serde(rename = "requester_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_id: Option<String>,

    /// The message sent with the responder request.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The array of targets the responder request is sent to.
    #[serde(rename = "responder_request_targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responder_request_targets: Option<Vec<ResponderRequestTargetReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body21 {
    /// The number of seconds to snooze the incident for. After this number of seconds has elapsed, the incident will return to the \"triggered\" state.
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body22 {
    /// The message to be posted as a status update.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body23 {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<LogEntriesidchannelChannel>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body24 {
    #[serde(rename = "maintenance_window")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body25 {
    #[serde(rename = "maintenance_window")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body26 {
    #[serde(rename = "response_play")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_play: Option<ResponsePlay>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body27 {
    #[serde(rename = "response_play")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_play: Option<ResponsePlay>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body28 {
    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body29 {
    #[serde(rename = "ruleset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<RulesetsRuleset>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body3 {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnalyticsmetricsincidentsallFilters>,

    /// The time zone to use for the results and grouping.
    #[serde(rename = "time_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// The time unit to aggregate metrics by.  If no value is provided, the metrics will be aggregated for the entire period.
    #[serde(rename = "aggregate_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_unit: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body30 {
    #[serde(rename = "ruleset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body31 {
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<RulesetsidrulesRule>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body32 {
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Value>,

    /// The id of the event rule to update.
    #[serde(rename = "rule_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body33 {
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body34 {
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body35 {
    #[serde(rename = "override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _override: Option<ModelOverride>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body36 {
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body37 {
    /// List of all service dependencies to be created.
    #[serde(rename = "relationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<ServiceDependenciesassociateRelationships>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body38 {
    /// List of all service dependencies to be deleted.
    #[serde(rename = "relationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<ServiceDependenciesassociateRelationships>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body39 {
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body4 {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnalyticsmetricsincidentsservicesFilters>,

    /// The time zone to use for the results and grouping.
    #[serde(rename = "time_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// The time unit to aggregate metrics by (day/week/month).  Used in conjunction with the service_id.
    #[serde(rename = "aggregate_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_unit: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body40 {
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body41 {
    #[serde(rename = "integration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<Integration>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body42 {
    #[serde(rename = "integration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<Integration>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body43 {
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body44 {
    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body45 {
    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body46 {
    /// The role of the user on the team.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body47 {
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body48 {
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body49 {
    #[serde(rename = "contact_method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_method: Option<ContactMethod>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body5 {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnalyticsmetricsincidentsallFilters>,

    /// The time zone to use for the results and grouping.
    #[serde(rename = "time_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// The time unit to aggregate metrics by.  If no value is provided, the metrics will be aggregated for the entire period.
    #[serde(rename = "aggregate_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_unit: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body50 {
    #[serde(rename = "contact_method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_method: Option<ContactMethod>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body51 {
    #[serde(rename = "notification_rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_rule: Option<NotificationRule>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body52 {
    #[serde(rename = "notification_rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_rule: Option<NotificationRule>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body6 {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnalyticsrawincidentsFilters>,

    /// Specifies an incident by ID, the paginated results will begin with the incident directly after this one.
    #[serde(rename = "starting_after")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    /// Specifies an incident by ID, the paginated results will end with the incident directly before this one.
    #[serde(rename = "ending_before")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    /// Number of results to include in each batch. Limits between 1 to 1000 are accepted.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,

    /// The time zone to use for the results.
    #[serde(rename = "time_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body7 {
    #[serde(rename = "business_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_service: Option<BusinessServicesBusinessService>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body8 {
    #[serde(rename = "business_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_service: Option<BusinessServicesBusinessService>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body9 {
    #[serde(rename = "escalation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policy: Option<EscalationPolicy>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessService {
    /// The name of the business service.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The user-provided description of the business service.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The point of contact assigned to this service.
    #[serde(rename = "point_of_contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_of_contact: Option<String>,

    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team2>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessServiceReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
/// The business service to be created
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessServicesBusinessService {
    /// The name of the business service.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the business service.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The owner of the business service.
    #[serde(rename = "point_of_contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_of_contact: Option<String>,

    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team1>,
}
/// Polymorphic object representation of the means by which the action was channeled. Has different formats depending on type, indicated by channel[type]. Will be one of `auto`, `email`, `api`, `nagios`, or `timeout` if `agent[type]` is `service`. Will be one of `email`, `sms`, `website`, `web_trigger`, or `note` if `agent[type]` is `user`. See [below](https://developer.pagerduty.com/documentation/rest/log_entries/show#channel_types) for detailed information about channel formats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// user
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<HashMap<(), ()>>,

    /// team
    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<HashMap<(), ()>>,

    /// channel
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<HashMap<(), ()>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConferenceBridge {
    /// The phone number of the conference call for the conference bridge. Phone numbers should be formatted like +1 415-555-1212,,,,1234#, where a comma (,) represents a one-second wait and pound (#) completes access code input.
    #[serde(rename = "conference_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_number: Option<String>,

    /// An URL for the conference bridge. This could be a link to a web conference or Slack channel.
    #[serde(rename = "conference_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_url: Option<String>,
}
/// The method to contact a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactMethod {
    /// The type of contact method being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The label (e.g., \"Work\", \"Mobile\", etc.).
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The \"address\" to deliver to: email, phone number, etc., depending on the type.
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactMethodReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Context {
    /// The type of context being attached to the incident.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The link's target url
    #[serde(rename = "href")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    /// The image's source url
    #[serde(rename = "src")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,

    /// The alternate display for an image
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailBasedIntegration {
    /// The name of this integration.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    /// The date/time when this integration was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorReference>,

    /// This is the unique fully-qualified email address used for routing emails to this integration for processing.
    #[serde(rename = "integration_email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_email: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailContactMethod {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The label (e.g., \"Work\", \"Mobile\", etc.).
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The \"address\" to deliver to: email, phone number, etc., depending on the type.
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Send an abbreviated email message instead of the standard email output. Useful for email-to-SMS gateways and email based pagers.
    #[serde(rename = "send_short_email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_short_email: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalateLogEntry {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Time at which the log entry was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    #[serde(rename = "agent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,

    /// Optional field containing a note, if one was included with the log entry.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Contexts to be included with the trigger such as links to graphs or images.
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<Context>>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,

    /// Will consist of references unless included
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,

    #[serde(rename = "event_details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_details: Option<LogEntryEventDetails>,

    #[serde(rename = "assigned_user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<UserReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalationPolicy {
    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The name of the escalation policy.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Escalation policy description.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The number of times the escalation policy will repeat after reaching the end of its escalation.
    #[serde(rename = "num_loops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_loops: Option<usize>,

    /// Determines how on call handoff notifications will be sent for users on the escalation policy. Defaults to \"if_has_services\".
    #[serde(rename = "on_call_handoff_notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_call_handoff_notifications: Option<String>,

    #[serde(rename = "escalation_rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_rules: Option<Vec<EscalationRule>>,

    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceReference>>,

    /// Teams associated with the policy. Account must have the `teams` ability to use this parameter.
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,
}
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalationPolicyReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalationRule {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The number of minutes before an unacknowledged incident escalates away from this rule.
    #[serde(rename = "escalation_delay_in_minutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_delay_in_minutes: Option<i64>,

    /// The targets an incident should be assigned to upon reaching this rule.
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<EscalationTarget>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalationTarget {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    /// The name of the extension.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The url of the extension.
    #[serde(rename = "endpoint_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,

    /// The objects for which the extension applies
    #[serde(rename = "extension_objects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_objects: Option<Vec<ServiceReference>>,

    #[serde(rename = "extension_schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_schema: Option<ExtensionSchemaReference>,

    /// Whether or not this extension is temporarily disabled; for example, a webhook extension that is repeatedly rejected by the server.
    #[serde(rename = "temporarily_disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporarily_disabled: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionSchema {
    /// A small logo, 18-by-18 pixels.
    #[serde(rename = "icon_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,

    /// A large logo, 75 pixels high and no more than 300 pixels wide.
    #[serde(rename = "logo_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    /// Human friendly display label
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Machine friendly display label
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// The long description for the Extension
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A link to the extension's support guide
    #[serde(rename = "guide_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_url: Option<String>,

    /// The types of PagerDuty incident events that will activate this Extension
    #[serde(rename = "send_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_types: Option<Vec<String>>,

    /// The url that the webhook payload will be sent to for this Extension.
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionSchemaReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Incident {
    /// The number of the incident. This is unique across your account.
    #[serde(rename = "incident_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_number: Option<i64>,

    /// The date/time the incident was first triggered.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The current status of the incident.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// A succinct description of the nature, symptoms, cause, or effect of the incident.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The list of pending_actions on the incident. A pending_action object contains a type of action which can be escalate, unacknowledge, resolve or urgency_change. A pending_action object contains at, the time at which the action will take place. An urgency_change pending_action will contain to, the urgency that the incident will change to.
    #[serde(rename = "pending_actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_actions: Option<Vec<IncidentAction>>,

    /// The incident's de-duplication key.
    #[serde(rename = "incident_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_key: Option<String>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    /// List of all assignments for this incident.
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<Assignment>>,

    /// How the current incident assignments were decided.  Note that `direct_assignment` incidents will not escalate up the attached `escalation_policy`
    #[serde(rename = "assigned_via")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_via: Option<String>,

    /// List of all acknowledgements for this incident.
    #[serde(rename = "acknowledgements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgements: Option<Vec<Acknowledgement>>,

    /// The time at which the status of the incident last changed.
    #[serde(rename = "last_status_change_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "last_status_change_by")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_by: Option<Agent>,

    #[serde(rename = "first_trigger_log_entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_trigger_log_entry: Option<LogEntryReference>,

    #[serde(rename = "escalation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policy: Option<EscalationPolicyReference>,

    /// The teams involved in the incident’s lifecycle.
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,

    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<PriorityReference>,

    /// The current urgency of the incident.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    #[serde(rename = "resolve_reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_reason: Option<ResolveReason>,

    #[serde(rename = "alert_counts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_counts: Option<AlertCount>,

    #[serde(rename = "conference_bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_bridge: Option<ConferenceBridge>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<IncidentBody>,
}
/// An incident action is a pending change to an incident that will automatically happen at some future time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentAction {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentAddon {
    /// The type of Add-on.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The name of the Add-on.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The source URL to display in a frame in the PagerDuty UI. HTTPS is required.
    #[serde(rename = "src")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,

    /// The services this Add-on is associated with. If non-empty, the Add-on will appear only on incidents for those services. If empty, it will appear on incidents for all services.
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentBody {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Additional incident details.
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentNote {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,

    /// The note content
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// The time at which the note was submitted
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentUrgencyRule {
    /// The type of incident urgency: whether it's constant, or it's dependent on the support hours.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The incidents' urgency, if type is constant.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    #[serde(rename = "during_support_hours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub during_support_hours: Option<IncidentUrgencyType>,

    #[serde(rename = "outside_support_hours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outside_support_hours: Option<IncidentUrgencyType>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentUrgencyType {
    /// The type of incident urgency: whether it's constant, or it's dependent on the support hours.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The incidents' urgency, if type is constant.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentsAssignments {
    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserReference>,
}
/// Details of the incident to be created.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentsIncident {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// A succinct description of the nature, symptoms, cause, or effect of the incident.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<PriorityReference>,

    /// The urgency of the incident
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<IncidentBody>,

    /// A string which identifies the incident. Sending subsequent requests referencing the same service and with the same incident_key will result in those requests being rejected if an open incident matches that incident_key.
    #[serde(rename = "incident_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_key: Option<String>,

    /// Assign the incident to these assignees. Cannot be specified if an escalation policy is given.
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<IncidentsAssignments>>,

    #[serde(rename = "escalation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policy: Option<EscalationPolicyReference>,

    #[serde(rename = "conference_bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_bridge: Option<ConferenceBridge>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentsIncidents {
    /// The id of the incident to update.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The incident type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The new status of the incident.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The resolution for this incident if status is set to resolved.
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    /// A succinct description of the nature, symptoms, cause, or effect of the incident.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<PriorityReference>,

    /// Escalate the incident to this level in the escalation policy.
    #[serde(rename = "escalation_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_level: Option<i64>,

    /// Assign the incident to these assignees.
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<IncidentsAssignments>>,

    #[serde(rename = "escalation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policy: Option<EscalationPolicyReference>,

    #[serde(rename = "conference_bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_bridge: Option<ConferenceBridge>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentsRespondersReference {
    /// The status of the responder being added to the incident
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// The message sent with the responder request
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "requester")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<UserReference>,

    #[serde(rename = "requested_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<String>,
}
/// The parameters of the incident to update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentsidIncident {
    /// The incident type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The new status of the incident.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<PriorityReference>,

    /// The resolution for this incident if status is set to resolved.
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    /// The new title of the incident.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Escalate the incident to this level in the escalation policy.
    #[serde(rename = "escalation_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_level: Option<i64>,

    /// Assign the incident to these assignees.
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<IncidentsAssignments>>,

    #[serde(rename = "escalation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policy: Option<EscalationPolicyReference>,

    /// The urgency of the incident.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    #[serde(rename = "conference_bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_bridge: Option<ConferenceBridge>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse200 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2001 {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20010 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "extension_schemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_schemas: Option<Vec<ExtensionSchema>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20011 {
    #[serde(rename = "extension_schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_schema: Option<Vec<ExtensionSchema>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20012 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20013 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "incidents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incidents: Option<Vec<Incident>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20014 {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20015 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "alerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<Alert>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20016 {
    #[serde(rename = "alert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<Alert>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20017 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "log_entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_entries: Option<Vec<LogEntry>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20018 {
    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20019 {
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<IncidentNote>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2002 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<AddonReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20020 {
    #[serde(rename = "responder_request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responder_request: Option<ResponderRequest>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20021 {
    #[serde(rename = "status_update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update: Option<StatusUpdate>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20022 {
    #[serde(rename = "log_entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_entry: Option<LogEntry>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20023 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "maintenance_windows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_windows: Option<Vec<MaintenanceWindow>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20024 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notification>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20025 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "oncalls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncalls: Option<Vec<Oncall>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20026 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "priorities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Option<Vec<Priority>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20027 {
    #[serde(rename = "response_plays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_plays: Option<Vec<ResponsePlay>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20028 {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20029 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "rulesets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulesets: Option<Vec<Value>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2003 {
    /// The time unit that was used to aggregate the metrics.
    #[serde(rename = "aggregate_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_unit: Option<String>,

    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<InlineResponse2003Filters>,

    /// The time zone that was used for grouping the results.
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AnalyticsIncidentMetrics>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20030 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    /// The paginated list of rules of the ruleset.
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Value>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20031 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "schedules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<Schedule>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20032 {
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<ModelOverride>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20033 {
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20034 {
    /// List of all the technical service's dependencies
    #[serde(rename = "relationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<InlineResponse20034Relationships>>,
}
/// The reference to the service that is dependent on the technical service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20034DependentService {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20034Relationships {
    #[serde(rename = "supporting_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_service: Option<InlineResponse20034SupportingService>,

    #[serde(rename = "dependent_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_service: Option<InlineResponse20034DependentService>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
/// The reference to the service that supports the technical service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20034SupportingService {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20035 {
    /// List of all the business service's dependencies.
    #[serde(rename = "relationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<InlineResponse20035Relationships>>,
}
/// The reference to the service that is dependent on the business service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20035DependentService {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20035Relationships {
    #[serde(rename = "supporting_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_service: Option<InlineResponse20035SupportingService>,

    #[serde(rename = "dependent_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_service: Option<InlineResponse20035DependentService>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
/// The reference to the service that supports the business service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20035SupportingService {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20036 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20037 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<EntityReference>>,

    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<EntityReference>>,

    #[serde(rename = "escalation_policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policies: Option<Vec<EntityReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20038 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<Team>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20039 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<InlineResponse20039Members>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20039Members {
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,

    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
/// A collection of filters that were applied to the results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2003Filters {
    /// The lower boundary (inclusive) for the created_at range filter applied to the results.
    #[serde(rename = "created_at_start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_start: Option<String>,

    /// The upper boundary (exclusive) for the created_at range filter applied to the results.
    #[serde(rename = "created_at_end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_end: Option<String>,

    /// The urgency filter applied to the results.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    /// The major incident filter applied to the results.
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<bool>,

    /// The team ids filter applied to the results.
    #[serde(rename = "team_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,

    /// The service ids filter applied to the results.
    #[serde(rename = "service_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<String>>,

    /// The priority filter applied to the results.
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2004 {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnalyticsmetricsincidentsallFilters>,

    /// The time zone that was used for grouping the results.
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// The time unit to aggregate metrics by.  If no value is provided, the metrics will be aggregated for the entire period.
    #[serde(rename = "aggregate_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_unit: Option<String>,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20040 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20041 {
    #[serde(rename = "contact_methods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_methods: Option<Vec<ContactMethod>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20042 {
    #[serde(rename = "notification_rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_rules: Option<Vec<NotificationRule>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20043 {
    #[serde(rename = "notification_rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_rule: Option<NotificationRule>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20044 {
    #[serde(rename = "user_sessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_sessions: Option<Vec<UserSession>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20045 {
    #[serde(rename = "user_session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_session: Option<UserSession>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20046 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "vendors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<Vec<Vendor>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20047 {
    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<Vec<Vendor>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2005 {
    /// The time unit to aggregate metrics by (day/week/month)
    #[serde(rename = "aggregate_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_unit: Option<String>,

    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<InlineResponse2005Filters>,

    /// The time zone that was used for grouping the results.
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}
/// A collection of filters that were applied to the results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2005Filters {
    /// The lower boundary for the created_at range filter applied to the results.
    #[serde(rename = "created_at_start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_start: Option<String>,

    /// The upper boundary for the created_at range filter applied to the results.
    #[serde(rename = "created_at_end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_end: Option<String>,

    /// The urgency filter applied to the results.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,

    /// The major incident filter applied to the results.
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<bool>,

    /// The team ids filter applied to the results;
    #[serde(rename = "team_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,

    /// The service ids filter applied to the results.
    #[serde(rename = "service_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<String>>,

    /// The priority filter applied to the results.
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2006 {
    /// ID of the first incident in the response.
    #[serde(rename = "first")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    /// ID of the last incident in the response.
    #[serde(rename = "last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,

    /// Number of results to include in the batch.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are more resources available than were returned.
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<InlineResponse2005Filters>,

    /// The time zone that the results are in.
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Value>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2007 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "business_services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_services: Option<Vec<BusinessService>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2008 {
    #[serde(rename = "business_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_service: Option<BusinessService>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2009 {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "escalation_policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policies: Option<Vec<EscalationPolicy>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse201 {
    #[serde(rename = "addon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<AddonReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2011 {
    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<Incident>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2012 {
    #[serde(rename = "response_play")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_play: Option<ResponsePlay>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2013 {
    #[serde(rename = "ruleset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2014 {
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2015 {
    #[serde(rename = "contact_method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_method: Option<ContactMethod>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse409 {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<InlineResponse409Error>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse409Error {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,

    /// Error message string
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Integration {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The name of this integration.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    /// The date/time when this integration was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrationReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
/// The parameters to update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntriesidchannelChannel {
    /// New channel details
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// Channel type. Cannot be changed and must match the present value.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Time at which the log entry was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    #[serde(rename = "agent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,

    /// Optional field containing a note, if one was included with the log entry.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Contexts to be included with the trigger such as links to graphs or images.
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<Context>>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,

    /// Will consist of references unless included
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,

    #[serde(rename = "event_details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_details: Option<LogEntryEventDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntryEventDetails {
    /// Additional details about the event.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntryReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The order in which the maintenance window was created.
    #[serde(rename = "sequence_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,

    /// This maintenance window's start time. This is when the services will stop creating incidents. If this date is in the past, it will be updated to be the current time.
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// This maintenance window's end time. This is when the services will start creating incidents again. This date must be in the future and after the `start_time`.
    #[serde(rename = "end_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,

    /// A description for this maintenance window.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "created_by")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserReference>,

    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceReference>>,

    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelOverride {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The start date and time for the override.
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,

    /// The end date and time for the override.
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The type of notification.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The time at which the notification was sent
    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The address where the notification was sent. This will be null for notification type `push_notification`.
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,
}
/// A rule for contacting the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationRule {
    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The delay before firing the rule, in minutes.
    #[serde(rename = "start_delay_in_minutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delay_in_minutes: Option<usize>,

    #[serde(rename = "contact_method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_method: Option<ContactMethodReference>,

    /// Which incident urgency this rule is used for. Account must have the `urgencies` ability to have a low urgency notification rule.
    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationRuleReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotifyLogEntry {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Time at which the log entry was created
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    #[serde(rename = "agent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,

    /// Optional field containing a note, if one was included with the log entry.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Contexts to be included with the trigger such as links to graphs or images.
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<Context>>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,

    /// Will consist of references unless included
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,

    #[serde(rename = "event_details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_details: Option<LogEntryEventDetails>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Oncall {
    #[serde(rename = "escalation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policy: Option<EscalationPolicyReference>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,

    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ScheduleReference>,

    /// The escalation level for the on-call.
    #[serde(rename = "escalation_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_level: Option<i64>,

    /// The start of the on-call. If `null`, the on-call is a permanent user on-call.
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,

    /// The end of the on-call. If `null`, the user does not go off-call.
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutboundIntegrationReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    /// Echoes offset pagination property.
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Echoes limit pagination property.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Indicates if there are additional records to return
    #[serde(rename = "more")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,

    /// The total number of records matching the given query.
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneContactMethod {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The label (e.g., \"Work\", \"Mobile\", etc.).
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The \"address\" to deliver to: email, phone number, etc., depending on the type.
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// The 1-to-3 digit country calling code.
    #[serde(rename = "country_code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<u16>,

    /// If true, this phone is capable of receiving SMS messages.
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// If true, this phone has been blacklisted by PagerDuty and no messages will be sent to it.
    #[serde(rename = "blacklisted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklisted: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Priority {
    /// The user-provided short name of the priority.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The user-provided description of the priority.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorityReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushContactMethod {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The label (e.g., \"Work\", \"Mobile\", etc.).
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The \"address\" to deliver to: email, phone number, etc., depending on the type.
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// The type of device.
    #[serde(rename = "device_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,

    #[serde(rename = "sounds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sounds: Option<Vec<PushContactMethodSound>>,

    /// Time at which the contact method was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// If true, this phone has been blacklisted by PagerDuty and no messages will be sent to it.
    #[serde(rename = "blacklisted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklisted: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushContactMethodSound {
    /// The type of sound.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The sound file name.
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A string that determines the schema of the object. This must be the standard name for the entity, suffixed by `_reference` if the object is a reference.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveReason {
    /// The reason the incident was resolved. The only reason currently supported is merge.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponderRequest {
    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,

    #[serde(rename = "requester")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<UserReference>,

    /// The time the request was made
    #[serde(rename = "requested_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<String>,

    /// The message sent with the responder request
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The array of targets the responder request is being sent to
    #[serde(rename = "responder_request_targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responder_request_targets: Option<Vec<ResponderRequestTargetReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponderRequestTargetReference {
    /// The type of target (either a user or an escalation policy)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The id of the user or escalation policy
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// An array of responders associated with the specified incident
    #[serde(rename = "incident_responders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_responders: Option<Vec<IncidentsRespondersReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsePlay {
    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The name of the response play.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the response play.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<TeamReference>,

    /// An array containing the users and/or teams to be added as subscribers to any incident on which this response play is run.
    #[serde(rename = "subscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<AnyOfResponsePlaySubscribersItems>>,

    /// The content of the notification that will be sent to all incident subscribers upon the running of this response play. Note that this includes any users who may have already been subscribed to the incident prior to the running of this response play. If empty, no notifications will be sent.
    #[serde(rename = "subscribers_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers_message: Option<String>,

    /// An array containing the users and/or escalation policies to be requested as responders to any incident on which this response play is run.
    #[serde(rename = "responders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responders: Option<Vec<AnyOfResponsePlayRespondersItems>>,

    /// The message body of the notification that will be sent to this response play's set of responders. If empty, a default response request notification will be sent.
    #[serde(rename = "responders_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responders_message: Option<String>,

    /// String representing how this response play is allowed to be run. Valid options are:   - `services`: This response play cannot be manually run by any users. It will run automatically for new incidents triggered on any services that are configured with this response play.   - `teams`: This response play can be run manually on an incident only by members of its configured team. This option can only be selected when the `team` property for this response play is not empty.   - `responders`: This response play can be run manually on an incident by any responders in this account.
    #[serde(rename = "runnability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnability: Option<String>,

    /// The telephone number that will be set as the conference number for any incident on which this response play is run.
    #[serde(rename = "conference_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_number: Option<String>,

    /// The URL that will be set as the conference URL for any incident on which this response play is run.
    #[serde(rename = "conference_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Restriction {
    /// Specify the types of `restriction`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The duration of the restriction in seconds.
    #[serde(rename = "duration_seconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,

    /// The start time in HH:mm:ss format.
    #[serde(rename = "start_time_of_day")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_of_day: Option<String>,

    /// Only required for use with a `weekly_restriction` restriction type. The first day of the weekly rotation schedule as [ISO 8601 day](https://en.wikipedia.org/wiki/ISO_week_date) (1 is Monday, etc.)
    #[serde(rename = "start_day_of_week")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_day_of_week: Option<u8>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsRuleset {
    /// ID of the ruleset.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Name of the ruleset.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Routing keys routed to this ruleset.
    #[serde(rename = "routing_keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_keys: Option<Vec<String>>,

    /// The date the ruleset was created at.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<RulesetsRulesetCreator>,

    /// The date the ruleset was last updated.
    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updater")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updater: Option<RulesetsRulesetUpdater>,

    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<RulesetsRulesetTeam>,
}
/// Reference to the user that has created the ruleset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsRulesetCreator {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A string that determines the schema of the object
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
}
/// Reference to the team that owns the ruleset. If none is specified, only admins have access.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsRulesetTeam {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A string that determines the schema of the object
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
}
/// Reference to the user that has updated the ruleset last.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsRulesetUpdater {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A string that determines the schema of the object
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRule {
    /// ID of the event rule.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// Position/index of the rule within the ruleset.
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,

    /// Indicates whether the rule is disabled and would therefore not be evaluated.
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// Indicates whether the rule is the last rule of the ruleset that serves as a catch-all. It has limited functionality compared to other rules.
    #[serde(rename = "catch_all")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catch_all: Option<bool>,

    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<RulesetsidrulesRuleConditions>,

    #[serde(rename = "time_frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_frame: Option<RulesetsidrulesRuleTimeFrame>,

    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<RulesetsidrulesRuleActions>,
}
/// When an event matches this rule, the actions that will be taken to change the resulting alert and incident.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActions {
    #[serde(rename = "annotate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotate: Option<RulesetsidrulesRuleActionsAnnotate>,

    #[serde(rename = "event_action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_action: Option<RulesetsidrulesRuleActionsEventAction>,

    /// Use regular expressions to extract values from event fields to set fields on the resulting alert.
    #[serde(rename = "extractions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extractions: Option<Vec<RulesetsidrulesRuleActionsExtractions>>,

    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<RulesetsidrulesRuleActionsPriority>,

    #[serde(rename = "route")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<RulesetsidrulesRuleActionsRoute>,

    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<RulesetsidrulesRuleActionsSeverity>,

    #[serde(rename = "suppress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress: Option<RulesetsidrulesRuleActionsSuppress>,

    #[serde(rename = "suspend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: Option<RulesetsidrulesRuleActionsSuspend>,
}
/// Set a note on the resulting incident.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActionsAnnotate {
    /// The content of the note.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// Set whether the resulting alert status is trigger or resolve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActionsEventAction {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActionsExtractions {
    /// The alert field that will be set with the value from the regex.
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// The path to the event field where the regex will be applied to extract a value.
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// A RE2 regular expression.  If it contains one or more capture groups, their values will be extracted and appended together.  If it contains no capture groups, the whole match is used.
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}
/// Set the priority ID for the resulting incident. You can find the priority you want by calling the priorities endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActionsPriority {
    /// The priority ID.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// Set the service ID of the target service for the resulting alert. You can find the service you want to route to by calling the services endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActionsRoute {
    /// The target service's ID.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// Set the severity of the resulting alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActionsSeverity {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// Set whether the resulting alert is suppressed.  Can optionally be used with a threshold where resulting alerts will be suppressed until the threshold is met in window of time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActionsSuppress {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,

    /// The number of occurences needed during the window of time to trigger the theshold.
    #[serde(rename = "threshold_value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_value: Option<i64>,

    /// The time unit for the window of time.
    #[serde(rename = "threshold_time_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_time_unit: Option<String>,

    /// The amount of time units for the window of time.
    #[serde(rename = "threshold_time_amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_time_amount: Option<i64>,
}
/// [Early Access] Set the length of time to suspend the resulting alert before triggering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleActionsSuspend {
    /// The amount of time to suspend the alert in seconds.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
/// Conditions evaluated to check if an event matches this event rule. Is always empty for the catch all rule, though.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleConditions {
    /// Operator to combine sub-conditions.
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,

    /// Array of sub-conditions.
    #[serde(rename = "subconditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subconditions: Option<Vec<RulesetsidrulesRuleConditionsSubconditions>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleConditionsParameters {
    /// Path to a field in an event, in dot-notation.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Value to apply to the operator.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Options to configure the operator.
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<(), ()>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleConditionsSubconditions {
    /// The type of operator to apply.
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,

    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RulesetsidrulesRuleConditionsParameters>,
}
/// Time-based conditions for limiting when the rule is active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleTimeFrame {
    #[serde(rename = "active_between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_between: Option<RulesetsidrulesRuleTimeFrameActiveBetween>,

    #[serde(rename = "scheduled_weekly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_weekly: Option<RulesetsidrulesRuleTimeFrameScheduledWeekly>,
}
/// A fixed window of time during which the rule is active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleTimeFrameActiveBetween {
    /// The start time in milliseconds.
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time in milliseconds.
    #[serde(rename = "end_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}
/// A reccuring window of time based on the day of the week, during which the rule is active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetsidrulesRuleTimeFrameScheduledWeekly {
    /// The amount of milliseconds into the day at which the window starts.
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// The duration of the window in milliseconds.
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// The timezone.
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// An array of day values. Ex [1, 3, 5] is Monday, Wednesday, Friday.
    #[serde(rename = "weekdays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekdays: Option<Vec<i32>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// A list of schedule layers.
    #[serde(rename = "schedule_layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_layers: Option<Vec<ScheduleLayer>>,

    /// The time zone of the schedule.
    #[serde(rename = "time_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// The name of the schedule
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the schedule
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "final_schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_schedule: Option<SubSchedule>,

    #[serde(rename = "overrides_subschedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides_subschedule: Option<SubSchedule>,

    /// An array of all of the escalation policies that uses this schedule.
    #[serde(rename = "escalation_policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policies: Option<Vec<EscalationPolicyReference>>,

    /// An array of all of the users on the schedule.
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserReference>>,

    /// An array of all of the teams on the schedule.
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleLayer {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The start time of this layer.
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,

    /// The end time of this layer. If `null`, the layer does not end.
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,

    /// The ordered list of users on this layer. The position of the user on the list determines their order in the layer.
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ScheduleLayerUser>>,

    /// An array of restrictions for the layer. A restriction is a limit on which period of the day or week the schedule layer can accept assignments.
    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<Restriction>>,

    /// The effective start time of the layer. This can be before the start time of the schedule.
    #[serde(rename = "rotation_virtual_start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_virtual_start: Option<chrono::DateTime<chrono::Utc>>,

    /// The duration of each on-call shift in seconds.
    #[serde(rename = "rotation_turn_length_seconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_turn_length_seconds: Option<i64>,

    /// The name of the schedule layer.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// This is a list of entries on the computed layer for the current time range. Since or until must be set in order for this field to be populated.
    #[serde(rename = "rendered_schedule_entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_schedule_entries: Option<Vec<ScheduleLayerEntry>>,

    /// The percentage of the time range covered by this layer. Returns null unless since or until are set.
    #[serde(rename = "rendered_coverage_percentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_coverage_percentage: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleLayerEntry {
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,

    /// The start time of this entry.
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,

    /// The end time of this entry. If null, the entry does not end.
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleLayerUser {
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAction {
    /// The type of schedule action. Must be set to urgency_change.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<ScheduledActionAt>,

    /// Urgency level. Must be set to high.
    #[serde(rename = "to_urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_urgency: Option<String>,
}
/// Represents when scheduled action will occur.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduledActionAt {
    /// Must be set to named_time.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Designates either the start or the end of support hours.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    /// The type of object being created.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The name of the service.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The user-provided description of the service.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Time in seconds that an incident is automatically resolved if left open for that long. Value is `null` if the feature is disabled. Value must not be negative. Setting this field to `0`, `null` (or unset in POST request) will disable the feature.
    #[serde(rename = "auto_resolve_timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_resolve_timeout: Option<i64>,

    /// Time in seconds that an incident changes to the Triggered State after being Acknowledged. Value is `null` if the feature is disabled. Value must not be negative. Setting this field to `0`, `null` (or unset in POST request) will disable the feature.
    #[serde(rename = "acknowledgement_timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgement_timeout: Option<i64>,

    /// The date/time when this service was created
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The current state of the Service. Valid statuses are:   - `active`: The service is enabled and has no open incidents. - `warning`: The service is enabled and has one or more acknowledged incidents. - `critical`: The service is enabled and has one or more triggered incidents. - `maintenance`: The service is under maintenance, no new incidents will be triggered during maintenance mode. - `disabled`: The service is disabled and will not have any new triggered incidents.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The date/time when the most recent incident was created for this service.
    #[serde(rename = "last_incident_timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_incident_timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "escalation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_policy: Option<EscalationPolicyReference>,

    /// The set of teams associated with this service.
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,

    /// An array containing Integration objects that belong to this service. If `integrations` is passed as an argument, these are full objects - otherwise, these are references.
    #[serde(rename = "integrations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<IntegrationReference>>,

    #[serde(rename = "incident_urgency_rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_urgency_rule: Option<IncidentUrgencyRule>,

    #[serde(rename = "support_hours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_hours: Option<SupportHours>,

    /// An array containing scheduled actions for the service.
    #[serde(rename = "scheduled_actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_actions: Option<Vec<ScheduledAction>>,

    /// The array of Add-ons associated with this service.
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<AddonReference>>,

    /// Whether a service creates only incidents, or both alerts and incidents. A service must create alerts in order to enable incident merging. * \"create_incidents\" - The service will create one incident and zero alerts for each incoming event. * \"create_alerts_and_incidents\" - The service will create one incident and one associated alert for each incoming event.
    #[serde(rename = "alert_creation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_creation: Option<String>,

    /// Defines how alerts on this service will be automatically grouped into incidents. Note that the alert grouping features are available only on certain plans. There are three available options: * null - No alert grouping on the service. Each alert will create a separate incident; * \"time\" - All alerts within a specified duration will be grouped into the same incident. This duration is set in the `alert_grouping_timeout` setting (described below). Available on Standard, Enterprise, and Event Intelligence plans; * \"intelligent\" - Alerts will be intelligently grouped based on a machine learning model that looks at the alert summary, timing, and the history of grouped alerts. Available on Enterprise and Event Intelligence plans
    #[serde(rename = "alert_grouping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_grouping: Option<String>,

    /// The duration in minutes within which to automatically group incoming alerts. This setting applies only when `alert_grouping` is set to `\"time\"`. To continue grouping alerts until the incident is resolved, set this value to `0`.
    #[serde(rename = "alert_grouping_timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_grouping_timeout: Option<i64>,
}
/// The reference to the service that is dependent on the supporting service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDependenciesassociateDependentService {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDependenciesassociateRelationships {
    #[serde(rename = "supporting_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_service: Option<ServiceDependenciesassociateSupportingService>,

    #[serde(rename = "dependent_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_service: Option<ServiceDependenciesassociateDependentService>,
}
/// The reference to the service that supports the dependent service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDependenciesassociateSupportingService {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnoozeLogEntry {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// Time at which the log entry was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    #[serde(rename = "agent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,

    /// Optional field containing a note, if one was included with the log entry.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Contexts to be included with the trigger such as links to graphs or images.
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<Context>>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<IncidentReference>,

    /// Will consist of references unless included
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,

    #[serde(rename = "event_details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_details: Option<LogEntryEventDetails>,

    #[serde(rename = "changed_actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_actions: Option<Vec<IncidentAction>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusUpdate {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The message of the status update.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The date/time when this status update was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "sender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<UserReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubSchedule {
    /// The name of the subschedule
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// This is a list of entries on the computed layer for the current time range. Since or until must be set in order for this field to be populated.
    #[serde(rename = "rendered_schedule_entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_schedule_entries: Option<Vec<ScheduleLayerEntry>>,

    /// The percentage of the time range covered by this layer. Returns null unless since or until are set.
    #[serde(rename = "rendered_coverage_percentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_coverage_percentage: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportHours {
    /// The type of support hours
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The time zone for the support hours
    #[serde(rename = "time_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    #[serde(rename = "days_of_week")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<i32>>,

    /// The support hours' starting time of day (date portion is ignored)
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// The support hours' ending time of day (date portion is ignored)
    #[serde(rename = "end_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The label of the tag.
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagsToAdd {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The label of the tag. Should be used when type is \"tag\".
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The id of the tag. Should be used when type is \"tag_reference\".
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagsToRemove_ {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The id of the tag
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The name of the team.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the team.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<TeamReference>,
}
/// Reference to the team that owns the business service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team1 {
    /// The team ID
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
/// Reference to the team that owns the business service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team2 {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A string that determines the schema of the object.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The API show URL at which the object is accessible.
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The name of the user.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of object being created.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The user's email address.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The preferred time zone name. If null, the account's time zone will be used.
    #[serde(rename = "time_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// The schedule color.
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// The user role. Account must have the `read_only_users` ability to set a user as a `read_only_user` or a `read_only_limited_user`, and must have advanced permissions abilities to set a user as `observer` or `restricted_access`.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// The URL of the user's avatar.
    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// The user's bio.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// If true, the user has an outstanding invitation.
    #[serde(rename = "invitation_sent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_sent: Option<bool>,

    /// The user's title.
    #[serde(rename = "job_title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,

    /// The list of teams to which the user belongs. Account must have the `teams` ability to set this.
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamReference>>,

    /// The list of contact methods for the user.
    #[serde(rename = "contact_methods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_methods: Option<Vec<ContactMethodReference>>,

    /// The list of notification rules for the user.
    #[serde(rename = "notification_rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_rules: Option<Vec<NotificationRuleReference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRole {
    /// The role of the user for a set of resources.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Reference>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSession {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "user_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The date/time the user session was first created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The type of the session
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The summary of the session
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vendor {
    /// The short name of the vendor
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// URL of the vendor's main website
    #[serde(rename = "website_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,

    /// URL of a logo identifying the vendor
    #[serde(rename = "logo_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    /// URL of a small thumbnail image identifying the vendor
    #[serde(rename = "thumbnail_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    /// A short description of this vendor, and common use-cases of integrations for this vendor.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// URL of an integration guide for this vendor
    #[serde(rename = "integration_guide_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_guide_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VendorReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
/// Information about the configured webhook.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    /// The url endpoint the webhook payload is sent to.
    #[serde(rename = "endpoint_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,

    /// The name of the webhook.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "webhook_object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_object: Option<WebhookObject>,

    /// The object that contains webhook configuration values depending on the webhook type specification.
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<(), ()>>,

    #[serde(rename = "outbound_integration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_integration: Option<OutboundIntegrationReference>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookIncidentAction {
    /// Uniquely identifies this outgoing webhook message; can be used for idempotency when processing the messages.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,

    /// The date/time when this message was was sent.
    #[serde(rename = "triggered_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,

    /// The type of action being reported by this message. * `incident.trigger` - Sent when an incident is newly created/triggered. * `incident.acknowledge` - Sent when an incident is acknowledged by a user. * `incident.unacknowledge` - Sent when an incident is unacknowledged due to its acknowledgement timing out. * `incident.resolve` - Sent when an incident has been resolved. * `incident.assign` - Sent when an incident has been assigned to another user. Often occurs in concert with an `acknowledge`. * `incident.escalate` - Sent when an incident has been escalated to another user in the same escalation chain. * `incident.delegate` - Sent when an incident has been reassigned to another escalation policy. * `incident.annotate` - Sent when a note is created on an incident.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<Incident>,

    /// Log Entries that correspond to the action this Webhook is reporting. Includes the channels.
    #[serde(rename = "log_entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_entries: Option<Vec<LogEntry>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookObject {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookReference {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short-form, server-generated string that provides succinct, important information about an object suitable for primary labeling of an entity in a client. In many cases, this will be identical to `name`, though it is not intended to be an identifier.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// the API show URL at which the object is accessible
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,

    /// a URL at which the entity is uniquely displayed in the Web app
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksV1AssignedTo {
    /// Time at which the assignment was created.
    #[serde(rename = "at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<AllOfWebhooksV1AssignedToObject>,
}
/// The user assigned to the incident.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksV1AssignedToUser {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The user's name.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The user's email address.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}
/// The incident details at the time of the state change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksV1IncidentData {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The number of the incident. This is unique across the account.
    #[serde(rename = "incident_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_number: Option<i64>,

    /// The date/time the incident was first triggered.
    #[serde(rename = "created_on")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<chrono::DateTime<chrono::Utc>>,

    /// The current status of the incident.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,

    /// The incident's de-duplication key.
    #[serde(rename = "incident_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_key: Option<String>,

    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<WebhooksV1Service>,

    #[serde(rename = "assigned_to_user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_user: Option<WebhooksV1AssignedToUser>,

    #[serde(rename = "assigned_to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<Vec<WebhooksV1AssignedTo>>,

    #[serde(rename = "trigger_summary_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_summary_data: Option<WebhooksV1IncidentDataTriggerSummaryData>,

    #[serde(rename = "trigger_details_html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_details_html_url: Option<String>,

    /// The time at which the status of the incident last changed.
    #[serde(rename = "last_status_change_on")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_on: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "last_status_change_by")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_by: Option<WebhooksV1AssignedToUser>,

    /// Number of times the incident has been escalated.
    #[serde(rename = "number_of_escalations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_escalations: Option<usize>,

    #[serde(rename = "urgency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksV1IncidentDataTriggerSummaryData {
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}
/// A message containing information about a single PagerDuty action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksV1Message {
    /// Uniquely identifies this outgoing webhook message; can be used for idempotency when processing the messages.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,

    /// The type of action being reported by this message.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The date/time when the incident changed state.
    #[serde(rename = "created_on")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WebhooksV1MessageData>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksV1MessageData {
    #[serde(rename = "incident")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident: Option<WebhooksV1IncidentData>,
}
/// The service on which the incident occurred.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksV1Service {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the service.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,

    /// The date/time the service was deleted, if it has been removed.
    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The description of the service.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeeklyRestriction {
    /// Specify the types of `restriction`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    /// The duration of the restriction in seconds.
    #[serde(rename = "duration_seconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,

    /// The start time in HH:mm:ss format.
    #[serde(rename = "start_time_of_day")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_of_day: Option<String>,

    /// The first day of the weekly rotation schedule as [ISO 8601 day](https://en.wikipedia.org/wiki/ISO_week_date) (1 is Monday, etc.)
    #[serde(rename = "start_day_of_week")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_day_of_week: Option<u8>,
}
