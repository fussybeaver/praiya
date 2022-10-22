//! Method, error and parameter types for the Incidents endpoint.

use futures_core::Stream;
use http::header::FROM;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::Error;
use crate::models::*;
use crate::praiya::{NoopParams, PraiyaCustomHeaders};
use crate::{
    BaseOption, BaseRequest, PaginatedResponse, PaginationQueryComponent, Praiya, SingleResponse,
    SubSystem, DEFAULT_PAGERDUTY_API_LIMIT,
};

pub const API_ENDPOINT: &str = "https://api.pagerduty.com";

/// A client for the PagerDuty incidents API
pub struct IncidentsClient {
    pub(crate) api_endpoint: String,
    pub(crate) client: Praiya,
    pub(crate) from_email: String,
}

impl Praiya {
    pub fn incidents(&self, from_email: &str) -> IncidentsClient {
        IncidentsClient {
            api_endpoint: std::env::var("PAGERDUTY_API_ENDPOINT")
                .unwrap_or_else(|_| String::from(API_ENDPOINT)),
            client: Praiya::clone(self),
            from_email: String::from(from_email),
        }
    }
}

single_response_type!(Incident, incident, CreateIncident);

single_response_type!(IncidentNote, note, CreateIncidentNote);

plural_response_type!(
    NotificationSubscriptionWithContext,
    subscriptions,
    CreateIncidentNotificationSubscribers
);

single_response_type!(
    ResponderRequest,
    responder_request,
    CreateIncidentResponderRequest
);

single_response_type!(Incident, incident, CreateIncidentSnooze);

single_response_type!(StatusUpdate, status_update, CreateIncidentStatusUpdate);

single_response_type!(Incident, incident, GetIncident);

single_response_type!(Alert, alert, GetIncidentAlert);

list_response_type!(ListIncidentBusinessService, services, BusinessService);

list_response_type!(
    ListIncidentNotificationSubscription,
    subscribers,
    NotificationSubscription
);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[IncidentsClient::get_outlier_incident]"]
#[allow(dead_code)]
struct GetOutlierIncident {
    additional_details: Vec<String>,
    since: chrono::DateTime<chrono::Utc>,
}

single_response_type!(OutlierIncident, outlier_incident, GetOutlierIncident);

list_response_type!(ListPastIncidents, past_incidents, PastIncidents);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[IncidentsClient::get_related_incidents]"]
#[allow(dead_code)]
struct GetRelatedIncidents {
    additional_details: Vec<String>,
}

plural_response_type!(RelatedIncidents, related_incidents, GetRelatedIncidents);

list_response_type!(ListIncidentAlerts, alerts, Alert);

list_response_type!(ListIncidentLogEntries, log_entries, LogEntry);

list_response_type!(ListIncidentNote, notes, IncidentNote);

list_response_type!(ListIncident, incidents, Incident);

single_response_type!(IncidentReference, incident, MergeIncident);

single_response_type!(
    PutIncidentManualBusinessServiceAssociationRelationEnum,
    relation,
    PutIncidentManualBusinessServiceAssociation
);

single_response_type!(Incident, incident, UpdateIncident);

single_response_type!(Alert, alert, UpdateIncidentAlert);

plural_response_type!(Alert, alerts, UpdateIncidentAlerts);

plural_response_type!(Incident, incidents, UpdateIncidents);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[IncidentsClient::list_incident_alerts]"]
#[allow(dead_code)]
struct ListIncidentAlerts {
    statuses: Vec<String>,
    alert_key: String,
    sort_by: Vec<String>,
    include: Vec<String>,
}

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[IncidentsClient::list_incident_log_entries]"]
#[allow(dead_code)]
struct ListIncidentLogEntries {
    until: chrono::DateTime<chrono::Utc>,
    since: chrono::DateTime<chrono::Utc>,
    time_zone: chrono_tz::Tz,
    include: Vec<String>,
}

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[IncidentsClient::list_incidents]"]
#[allow(dead_code)]
struct ListIncidents {
    date_range: String,
    incident_key: String,
    include: Vec<String>,
    service_ids: Vec<String>,
    since: chrono::DateTime<chrono::Utc>,
    sort_by: Vec<String>,
    statuses: Vec<String>,
    team_ids: Vec<String>,
    time_zone: chrono_tz::Tz,
    until: chrono::DateTime<chrono::Utc>,
    urgencies: Vec<String>,
    user_ids: Vec<String>,
}

impl IncidentsClient {
    /// ---
    ///
    /// # Create an Incident
    ///
    /// Create an incident synchronously without a corresponding event from a monitoring service.
    ///
    ///
    /// ---
    pub async fn create_incident(&self, body: CreateIncident) -> Result<Incident, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, "/incidents", None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateIncidentResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create a note on an incident
    ///
    /// Create a new note for the specified incident.
    ///
    /// An incident represents a problem or an issue that needs to be addressed and resolved.
    ///
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#incidents)
    ///
    ///
    /// ---
    pub async fn create_incident_note(
        &self,
        id: &str,
        body: CreateIncidentNote,
    ) -> Result<IncidentNote, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/notes", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateIncidentNoteResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Add notification subscriber
    ///
    /// Subscribe the given entity to Incident Status Update Notifications.
    ///
    ///
    /// ---
    pub async fn create_incident_notification_subscribers(
        &self,
        id: &str,
        body: CreateIncidentNotificationSubscribers,
    ) -> Result<Vec<NotificationSubscriptionWithContext>, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/status_updates/subscribers", &id),
            None,
        )?;

        let mut builder = http::request::Builder::new();
        builder = builder.header(FROM, &self.from_email);
        let early_access: &str = PraiyaCustomHeaders::EarlyAccess(None).into();
        builder = builder.header(early_access, "true");

        let req = self.client.build_request(
            url,
            builder.method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateIncidentNotificationSubscribersResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create a responder request for an incident
    ///
    /// Send a new responder request for the specified incident.
    ///
    ///
    /// ---
    pub async fn create_incident_responder_request(
        &self,
        id: &str,
        body: CreateIncidentResponderRequest,
    ) -> Result<ResponderRequest, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/responder_requests", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateIncidentResponderRequestResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Snooze an incident
    ///
    /// Snooze an incident.
    ///
    ///
    /// ---
    pub async fn create_incident_snooze(
        &self,
        id: &str,
        body: CreateIncidentSnooze,
    ) -> Result<Incident, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/snooze", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateIncidentSnoozeResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create a status update on an incident
    ///
    /// Create a new status update for the specified incident.
    ///
    ///
    /// ---
    pub async fn create_incident_status_update(
        &self,
        id: &str,
        body: CreateIncidentStatusUpdate,
    ) -> Result<StatusUpdate, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/status_updates", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateIncidentStatusUpdateResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get an incident
    ///
    /// Show detailed information about an incident. Accepts either an incident id, or an incident number.
    ///
    ///
    /// ---
    pub async fn get_incident(&self, id: &str) -> Result<Incident, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, &format!("/incidents/{}", &id), None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetIncidentResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get an alert
    ///
    /// Show detailed information about an alert. Accepts an alert id.
    ///
    /// When a service sends an event to PagerDuty, an alert and corresponding incident is triggered in PagerDuty.
    ///
    ///
    /// ---
    pub async fn get_incident_alert(&self, id: &str, alert_id: &str) -> Result<Alert, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/alerts/{}", &id, &alert_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetIncidentAlertResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List Business Services impacted by the given Incident
    ///
    /// Retrieve a list of Business Services that are being impacted by the given Incident.
    ///
    ///
    /// ---
    pub fn list_incident_impacted_business_services(
        &self,
        id: &str,
    ) -> impl Stream<Item = Result<BusinessService, Error>> + '_ {
        self.client
            .list_request::<_, _, ListIncidentBusinessServiceResponse>(
                &self.api_endpoint,
                &format!("/incidents/{}/business_services/impacts", &id),
                NoopParams {},
                PraiyaCustomHeaders::EarlyAccess(None),
            )
    }

    /// ---
    ///
    /// # List notification subscribers
    ///
    /// Retrieve a list of Notification Subscribers on the Incident.
    ///
    ///
    /// ---
    pub fn list_incident_notification_subscribers(
        &self,
        id: &str,
    ) -> impl Stream<Item = Result<NotificationSubscription, Error>> + '_ {
        self.client
            .list_request::<_, _, ListIncidentNotificationSubscriptionResponse>(
                &self.api_endpoint,
                &format!("/incidents/{}/status_updates/subscribers", &id),
                NoopParams {},
                PraiyaCustomHeaders::EarlyAccess(None),
            )
    }

    /// ---
    ///
    /// # Get Outlier Incident
    ///
    /// Gets Outlier Incident information for a given Incident on its Service.
    ///
    ///
    /// ---
    pub async fn get_outlier_incident(
        &self,
        id: &str,
        query_params: GetOutlierIncidentParams,
    ) -> Result<OutlierIncident, Error> {
        let uri = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/outlier_incident", &id),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            uri,
            http::request::Builder::new().method(http::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetOutlierIncidentResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get Past Incidents
    ///
    /// Past Incidents returns Incidents within the past 6 months that have similar metadata and were generated on the same Service as the parent Incident. By default, 5 Past Incidents are returned. Note: This feature is currently available as part of the Event Intelligence package or Digital Operations plan only.
    ///
    ///
    /// ---
    pub fn list_past_incidents(
        &self,
        id: &str,
    ) -> impl Stream<Item = Result<PastIncidents, Error>> + '_ {
        self.client.list_request::<_, _, ListPastIncidentsResponse>(
            &self.api_endpoint,
            &format!("/incidents/{}/past_incidents", &id),
            NoopParams {},
            PraiyaCustomHeaders::None,
        )
    }

    /// ---
    ///
    /// # Get Related Incidents
    ///
    /// Returns the 20 most recent Related Incidents that are impacting other Responders and Services. Note: This feature is currently available as part of the Event Intelligence package or Digital Operations plan only.
    ///
    ///
    /// ---
    pub async fn get_related_incidents(
        &self,
        id: &str,
        query_params: GetRelatedIncidentsParams,
    ) -> Result<Vec<RelatedIncidents>, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/related_incidents", &id),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetRelatedIncidentsResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List alerts for an incident
    ///
    /// List alerts for the specified incident.
    ///
    ///
    /// ---
    pub fn list_incident_alerts(
        &self,
        id: &str,
        query_params: ListIncidentAlertsParams,
    ) -> impl Stream<Item = Result<Alert, Error>> + '_ {
        self.client
            .list_request::<_, _, ListIncidentAlertsResponse>(
                &self.api_endpoint,
                &format!("/incidents/{}/alerts", &id),
                query_params,
                PraiyaCustomHeaders::None,
            )
    }

    /// ---
    ///
    /// # List log entries for an incident
    ///
    /// List log entries for the specified incident.
    ///
    /// A Log Entry are a record of all events on your account.
    ///
    ///
    /// ---
    pub fn list_incident_log_entries(
        &self,
        id: &str,
        query_params: ListIncidentLogEntriesParams,
    ) -> impl Stream<Item = Result<LogEntry, Error>> + '_ {
        self.client
            .list_request::<_, _, ListIncidentLogEntriesResponse>(
                &self.api_endpoint,
                &format!("/incidents/{}/log_entries", &id),
                query_params,
                PraiyaCustomHeaders::None,
            )
    }

    /// ---
    ///
    /// # List notes for an incident
    ///
    /// List existing notes for the specified incident.
    ///
    ///
    /// ---
    pub fn list_incident_notes(
        &self,
        id: &str,
    ) -> impl Stream<Item = Result<IncidentNote, Error>> + '_ {
        self.client.list_request::<_, _, ListIncidentNoteResponse>(
            &self.api_endpoint,
            &format!("/incidents/{}/notes", &id),
            NoopParams {},
            PraiyaCustomHeaders::None,
        )
    }

    /// ---
    ///
    /// # List incidents
    ///
    /// List existing incidents.
    ///
    ///
    /// ---
    pub fn list_incidents(
        &self,
        query_params: ListIncidentsParams,
    ) -> impl Stream<Item = Result<Incident, Error>> + '_ {
        self.client.list_request::<_, _, ListIncidentResponse>(
            &self.api_endpoint,
            "/incidents",
            query_params,
            PraiyaCustomHeaders::None,
        )
    }

    /// ---
    ///
    /// # Merge incidents
    ///
    /// Merge a list of source incidents into this incident.
    ///
    ///
    /// ---
    pub async fn merge_incidents(
        &self,
        id: &str,
        body: MergeIncidents,
    ) -> Result<IncidentReference, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/merge", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, MergeIncidentResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Manually change an Incident&#x27;s Impact on a Business Service.
    ///
    /// Change Impact of an Incident on a Business Service.
    ///
    ///
    /// ---
    pub async fn put_incident_manual_business_service_association(
        &self,
        id: &str,
        business_service_id: &str,
        body: PutIncidentManualBusinessServiceAssociation,
    ) -> Result<PutIncidentManualBusinessServiceAssociationRelationEnum, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/incidents/{}/business_services/{}/impacts",
                &id, &business_service_id
            ),
            None,
        )?;

        let mut builder = http::request::Builder::new();
        let early_access: &str = PraiyaCustomHeaders::EarlyAccess(None).into();
        builder = builder.header(early_access, "true");

        let req = self.client.build_request(
            url,
            builder.method(http::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, PutIncidentManualBusinessServiceAssociationResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Remove notification subscriber
    ///
    /// Unsubscribes the matching Subscriber from Incident Status Update Notifications.
    ///
    ///
    /// ---
    pub async fn remove_incident_notification_subscriber(
        &self,
        id: &str,
        body: RemoveIncidentNotificationSubscribers,
    ) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/status_updates/unsubscribe", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Update an incident
    ///
    /// Acknowledge, resolve, escalate or reassign an incident.
    ///
    ///
    /// ---
    pub async fn update_incident(&self, id: &str, body: UpdateIncident) -> Result<Incident, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, &format!("/incidents/{}", &id), None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateIncidentResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update an alert
    ///
    /// Resolve an alert or associate an alert with a new parent incident.
    ///
    ///
    /// ---
    pub async fn update_incident_alert(
        &self,
        id: &str,
        alert_id: &str,
        body: UpdateIncidentAlert,
    ) -> Result<Alert, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/alerts/{}", &id, &alert_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateIncidentAlertResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Manage alerts
    ///
    /// Resolve multiple alerts or associate them with different incidents.
    ///
    /// A maximum of 500 alerts may be updated at a time. If more than this number of alerts are given, the API will respond with status 413 (Request Entity Too Large).
    ///
    /// Praiya note: this endpoint could return a partial list, because we do not implement streams
    /// in PUT requests.
    ///
    ///
    /// ---
    pub async fn update_incident_alerts(
        &self,
        id: &str,
        body: UpdateIncidentAlerts,
    ) -> Result<Vec<Alert>, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/incidents/{}/alerts", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateIncidentAlertsResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Manage incidents
    ///
    /// Acknowledge, resolve, escalate or reassign one or more incidents.
    ///
    /// A maximum of 500 incidents may be updated at a time. If more than this number of incidents are given, the API will respond with status 413 (Request Entity Too Large).
    ///
    /// Praiya note: this endpoint could return a partial list, because we do not implement streams
    /// in PUT requests.
    ///
    /// ---
    pub async fn update_incidents(&self, body: UpdateIncidents) -> Result<Vec<Incident>, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, "/incidents", None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, &self.from_email)
                .method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateIncidentsResponse>(req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::models::*;
    use crate::praiya::ParamsBuilder;
    use futures_util::TryStreamExt;

    #[tokio::test]
    async fn test_create_incident() {
        let pagerduty = crate::Praiya::new("test");
        let create_incident = CreateIncident {
            incident: IncidentsIncident {
                service: Service {
                    ..Default::default()
                },
                title: String::from("The building is on fire!"),
                ..Default::default()
            },
        };
        let incident = pagerduty
            .incidents("from@example.com")
            .create_incident(create_incident)
            .await
            .unwrap();

        assert_eq!(incident.id, Some(String::from("PT4KHLK")));
    }

    #[tokio::test]
    async fn test_create_incident_note() {
        let pagerduty = crate::Praiya::new("test");
        let create_incident_note = CreateIncidentNote {
            note: IncidentsidnotesNote {
                content: String::from("Solved by pouring water on the fire"),
            },
        };
        let note = pagerduty
            .incidents("from@example.com")
            .create_incident_note("PT4KHLK", create_incident_note)
            .await
            .unwrap();

        assert_eq!(note.id, Some(String::from("PWL7QXS")));
    }

    #[tokio::test]
    async fn test_create_incident_notification_subscribers() {
        let pagerduty = crate::Praiya::new("test");
        let create_incident_notification_subscriber = NotificationSubscriber {
            subscriber_id: Some(String::from("PD1234")),
            subscriber_type: Some(NotificationSubscriberSubscriberTypeEnum::TEAM),
        };
        let create_incident_notification_subscribers =
            super::CreateIncidentNotificationSubscribers {
                subscribers: vec![create_incident_notification_subscriber],
            };
        let subscription = pagerduty
            .incidents("from@example.com")
            .create_incident_notification_subscribers(
                "PT4KHLK",
                create_incident_notification_subscribers,
            )
            .await
            .unwrap();

        assert_eq!(subscription[0].account_id, Some(String::from("PD1234")));
    }

    #[tokio::test]
    async fn test_create_incident_responder_request() {
        let pagerduty = crate::Praiya::new("test");
        let create_incident_responder_request = CreateIncidentResponderRequest {
            requester_id: String::from("PL1JMK5"),
            message: String::from("Please help with issue - join bridge at +1(234)-567-8910"),
            responder_request_targets: vec![ResponderRequestTargetReference {
                id: Some(String::from("PJ25ZYX")),
                _type: String::from("user_reference"),
                ..Default::default()
            }],
            ..Default::default()
        };
        let responder_request = pagerduty
            .incidents("from@example.com")
            .create_incident_responder_request("PT4KHLK", create_incident_responder_request)
            .await
            .unwrap();

        assert_eq!(
            responder_request.incident.unwrap().id,
            Some(String::from("PXP12GZ"))
        );
    }

    #[tokio::test]
    async fn test_create_incident_snooze() {
        let pagerduty = crate::Praiya::new("test");
        let create_incident_snooze = CreateIncidentSnooze { duration: 1 };
        let incident = pagerduty
            .incidents("from@example.com")
            .create_incident_snooze("PT4KHLK", create_incident_snooze)
            .await
            .unwrap();

        assert_eq!(incident.id, Some(String::from("PT4KHLK")));
    }

    #[tokio::test]
    async fn test_create_incident_status_update() {
        let pagerduty = crate::Praiya::new("test");
        let create_incident_status_update = CreateIncidentStatusUpdate {
            message: String::from("The server fire is spreading."),
            ..Default::default()
        };
        let status_update = pagerduty
            .incidents("from@example.com")
            .create_incident_status_update("PT4KHLK", create_incident_status_update)
            .await
            .unwrap();

        assert_eq!(status_update.id, Some(String::from("PWL7QXS")));
    }

    #[tokio::test]
    async fn test_get_incident() {
        let pagerduty = crate::Praiya::new("test");
        let incident = pagerduty
            .incidents("from@example.com")
            .get_incident("PT4KHLK")
            .await
            .unwrap();

        assert_eq!(incident.id, Some(String::from("PT4KHLK")));
    }

    #[tokio::test]
    async fn test_get_incident_alert() {
        let pagerduty = crate::Praiya::new("test");
        let alert = pagerduty
            .incidents("from@example.com")
            .get_incident_alert("PWL7QXS", "PT4KHLK")
            .await
            .unwrap();

        assert_eq!(alert.id, Some(String::from("PT4KHLK")));
    }

    #[tokio::test]
    async fn test_list_incident_impacted_business_services() {
        let pagerduty = crate::Praiya::new("test");

        let business_service: Option<BusinessService> = pagerduty
            .incidents("from@example.com")
            .list_incident_impacted_business_services("PT4KHLK")
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            business_service.unwrap().id.as_ref().unwrap(),
            &String::from("PD1234")
        );
    }

    #[tokio::test]
    async fn test_list_incident_notification_subscribers() {
        let pagerduty = crate::Praiya::new("test");

        let notification_subscription: Option<NotificationSubscription> = pagerduty
            .incidents("from@example.com")
            .list_incident_notification_subscribers("PT4KHLK")
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            notification_subscription
                .unwrap()
                .subscriber_id
                .as_ref()
                .unwrap(),
            &String::from("PD1234")
        );
    }

    #[tokio::test]
    async fn test_get_outlier_incident() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetOutlierIncidentParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.additional_details(vec![""]);
        opts_builder.since(&since);
        let opts = opts_builder.build();

        let outlier_incident = pagerduty
            .incidents("from@example.com")
            .get_outlier_incident("PT4KHLK", opts)
            .await
            .unwrap();

        assert_eq!(
            outlier_incident.incident.unwrap().id,
            Some(String::from("PR2P3RW"))
        );
    }

    #[tokio::test]
    async fn test_list_past_incidents() {
        let pagerduty = crate::Praiya::new("test");

        let past_incident: Option<PastIncidents> = pagerduty
            .incidents("from@example.com")
            .list_past_incidents("PT4KHLK")
            .try_next()
            .await
            .unwrap();

        assert_eq!(past_incident.unwrap().score.unwrap(), 46.8249);
    }

    #[tokio::test]
    async fn test_get_related_incidents() {
        let pagerduty = crate::Praiya::new("test");
        let mut opts_builder = super::GetRelatedIncidentsParamsBuilder::new();
        opts_builder.additional_details(vec![""]);
        let opts = opts_builder.build();

        let incident = pagerduty
            .incidents("from@example.com")
            .get_related_incidents("PT4KHLK", opts)
            .await
            .unwrap();

        assert_eq!(
            incident[0].incident.as_ref().unwrap().id,
            Some(String::from("PR2P3RW"))
        );
    }

    #[tokio::test]
    async fn test_list_incident_alerts() {
        let pagerduty = crate::Praiya::new("test");
        let mut opts_builder = super::ListIncidentAlertsParamsBuilder::new();
        opts_builder.statuses(vec!["triggered"]);
        opts_builder.alert_key("abc");
        opts_builder.sort_by(vec!["id"]);
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let alert: Option<Alert> = pagerduty
            .incidents("from@example.com")
            .list_incident_alerts("PT4KHLK", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            alert.unwrap().id.as_ref().unwrap(),
            &String::from("PT4KHLK")
        );
    }

    #[tokio::test]
    async fn test_list_incident_log_entries() {
        let pagerduty = crate::Praiya::new("test");
        let mut opts_builder = super::ListIncidentLogEntriesParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.until(&now);
        opts_builder.since(&since);
        opts_builder.time_zone(&chrono_tz::EST);
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let log_entry: Option<LogEntry> = pagerduty
            .incidents("from@example.com")
            .list_incident_log_entries("PT4KHLK", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            log_entry.unwrap().id.as_ref().unwrap(),
            &String::from("Q02JTSNZWHSEKV")
        );
    }

    #[tokio::test]
    async fn test_list_incident_notes() {
        let pagerduty = crate::Praiya::new("test");
        let note: Option<IncidentNote> = pagerduty
            .incidents("from@example.com")
            .list_incident_notes("PT4KHLK")
            .try_next()
            .await
            .unwrap();

        assert_eq!(note.unwrap().id.as_ref().unwrap(), &String::from("PWL7QXS"));
    }

    #[tokio::test]
    async fn test_list_incidents() {
        let pagerduty = crate::Praiya::new("test");
        let mut opts_builder = super::ListIncidentsParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.until(&now);
        opts_builder.since(&since);
        opts_builder.time_zone(&chrono_tz::EST);
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let incident: Option<Incident> = pagerduty
            .incidents("from@example.com")
            .list_incidents(opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            incident.unwrap().id.as_ref().unwrap(),
            &String::from("PT4KHLK")
        );
    }

    #[tokio::test]
    async fn test_merge_incidents() {
        let pagerduty = crate::Praiya::new("test");
        let mut source_incidents = vec![];
        source_incidents.push(IncidentReference {
            id: Some(String::from("P8JOGX7")),
            ..Default::default()
        });
        source_incidents.push(IncidentReference {
            id: Some(String::from("PPVZH9X")),
            ..Default::default()
        });
        let merge_incidents = MergeIncidents { source_incidents };

        let incident: IncidentReference = pagerduty
            .incidents("from@example.com")
            .merge_incidents("PT4KHLK", merge_incidents)
            .await
            .unwrap();

        assert_eq!(incident.id.as_ref().unwrap(), &String::from("PT4KHLK"));
    }

    #[tokio::test]
    async fn test_put_incident_manual_business_service_association() {
        let pagerduty = crate::Praiya::new("test");
        let assoc = PutIncidentManualBusinessServiceAssociation {
            relation: PutIncidentManualBusinessServiceAssociationRelationEnum::IMPACTED,
        };
        let res = pagerduty
            .incidents("from@example.com")
            .put_incident_manual_business_service_association("PD1234", "PT4KHLK", assoc)
            .await
            .unwrap();

        assert_eq!(
            res,
            PutIncidentManualBusinessServiceAssociationRelationEnum::IMPACTED
        );
    }

    #[tokio::test]
    async fn test_remove_incident_notification_subscriber() {
        let pagerduty = crate::Praiya::new("test");
        let notification_subscribers = super::RemoveIncidentNotificationSubscribers {
            subscribers: vec![NotificationSubscriber {
                subscriber_id: Some(String::from("PD1234")),
                subscriber_type: Some(NotificationSubscriberSubscriberTypeEnum::TEAM),
            }],
        };
        let unit = pagerduty
            .incidents("from@example.com")
            .remove_incident_notification_subscriber("PT4KHLK", notification_subscribers)
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_update_incident() {
        let pagerduty = crate::Praiya::new("test");
        let update_incident = UpdateIncident {
            incident: IncidentsidIncident {
                status: Some(IncidentsidIncidentStatusEnum::ACKNOWLEDGED),
                ..Default::default()
            },
        };
        let incident = pagerduty
            .incidents("from@example.com")
            .update_incident("PT4KHLK", update_incident)
            .await
            .unwrap();

        assert_eq!(incident.id, Some(String::from("PT4KHLK")));
    }

    #[tokio::test]
    async fn test_update_incident_alert() {
        let pagerduty = crate::Praiya::new("test");
        let update_incident_alert = UpdateIncidentAlert {
            alert: Alert {
                status: Some(AlertStatusEnum::RESOLVED),
                incident: Some(Incident {
                    id: Some(String::from("PEYSGVF")),
                    ..Default::default()
                }),
                body: Some(Body {
                    contexts: Some(vec![Context {
                        _type: ContextTypeEnum::LINK,
                        ..Default::default()
                    }]),
                    details: Some(
                        vec![
                            (
                                String::from("customKey"),
                                serde_json::Value::String(String::from("Server is on fire!")),
                            ),
                            (
                                String::from("customKey2"),
                                serde_json::Value::String(String::from("Other struff!")),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        let alert = pagerduty
            .incidents("from@example.com")
            .update_incident_alert("PT4KHLK", "PEYSGVF", update_incident_alert)
            .await
            .unwrap();

        assert_eq!(
            alert.incident.as_ref().unwrap().id,
            Some(String::from("PEYSGVF"))
        );
    }

    #[tokio::test]
    async fn test_update_incident_alerts() {
        let pagerduty = crate::Praiya::new("test");
        let update_incident_alerts = UpdateIncidentAlerts {
            alerts: vec![
                Alert {
                    id: Some(String::from("PPVZH9X")),
                    status: Some(AlertStatusEnum::RESOLVED),
                    ..Default::default()
                },
                Alert {
                    id: Some(String::from("P8JOGX7")),
                    incident: Some(Incident {
                        id: Some(String::from("PPVZH9X")),
                        _type: String::from("incident_reference"),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ],
        };
        let alerts = pagerduty
            .incidents("from@example.com")
            .update_incident_alerts("PT4KHLK", update_incident_alerts)
            .await
            .unwrap();

        assert_eq!(alerts[0].id, Some(String::from("PT4KHLK")));
    }

    #[tokio::test]
    async fn test_update_incidents() {
        let pagerduty = crate::Praiya::new("test");
        let update_incidents = UpdateIncidents {
            incidents: vec![
                IncidentsIncidents {
                    id: String::from("PT4KHLK"),
                    status: Some(IncidentsIncidentsStatusEnum::ACKNOWLEDGED),
                    ..Default::default()
                },
                IncidentsIncidents {
                    id: String::from("PQMF62U"),
                    priority: Some(Priority {
                        id: Some(String::from("P53ZZH5")),
                        _type: String::from("priority_reference"),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ],
        };
        let incidents = pagerduty
            .incidents("from@example.com")
            .update_incidents(update_incidents)
            .await
            .unwrap();

        assert_eq!(incidents[0].id, Some(String::from("PT4KHLK")));
    }
}
