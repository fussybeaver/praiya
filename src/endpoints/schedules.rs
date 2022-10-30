//! Method, error and parameter types for the Schedules endpoint.

use futures_core::Stream;
use futures_util::StreamExt;
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

/// A client for the PagerDuty schedules API
pub struct SchedulesClient {
    pub(crate) api_endpoint: String,
    pub(crate) client: Praiya,
}

impl Praiya {
    pub fn schedules(&self) -> SchedulesClient {
        SchedulesClient {
            api_endpoint: std::env::var("PAGERDUTY_API_ENDPOINT")
                .unwrap_or_else(|_| String::from(API_ENDPOINT)),
            client: Praiya::clone(self),
        }
    }
}

single_response_type!(Schedule, schedule, CreateSchedule);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[SchedulesClient::create_schedule"]
#[allow(dead_code)]
struct CreateSchedule {
    overflow: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateScheduleOverrideResponse {
    pub status: u16,
    #[serde(rename = "override")]
    pub _override: ModelOverride,
}

single_response_type!(Schedule, schedule, CreateSchedulePreview);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[SchedulesClient::create_schedule_preview"]
#[allow(dead_code)]
struct CreateSchedulePreview {
    overflow: bool,
    since: chrono::DateTime<chrono::Utc>,
    until: chrono::DateTime<chrono::Utc>,
}

single_response_type!(Schedule, schedule, GetSchedule);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[SchedulesClient::get_schedule]"]
#[allow(dead_code)]
struct GetSchedule {
    since: chrono::DateTime<chrono::Utc>,
    time_zone: chrono_tz::Tz,
    until: chrono::DateTime<chrono::Utc>,
}

list_response_type!(ListScheduleOverrides, overrides, ModelOverride);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[SchedulesClient::list_schedules_overrides"]
#[allow(dead_code)]
struct ListScheduleOverrides {
    since: chrono::DateTime<chrono::Utc>,
    until: chrono::DateTime<chrono::Utc>,
    editable: bool,
    overflow: bool,
}

list_response_type!(ListScheduleUsers, users, User);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[SchedulesClient::list_schedules_users"]
#[allow(dead_code)]
struct ListScheduleUsers {
    since: chrono::DateTime<chrono::Utc>,
    until: chrono::DateTime<chrono::Utc>,
}

list_response_type!(ListSchedules, schedules, Schedule);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[SchedulesClient::list_schedules"]
#[allow(dead_code)]
struct ListSchedules {
    query: String,
    include: Vec<String>,
}

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[SchedulesClient::list_schedules_audit_records]"]
#[allow(dead_code)]
struct ListSchedulesAuditRecords {
    since: chrono::DateTime<chrono::Utc>,
    until: chrono::DateTime<chrono::Utc>,
}

single_response_type!(Schedule, schedule, UpdateSchedule);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[SchedulesClient::update_schedule]"]
#[allow(dead_code)]
struct UpdateSchedule {
    overflow: bool,
}

impl SchedulesClient {
    /// ---
    ///
    /// # Create a schedule
    ///
    /// Create a new on-call schedule.
    ///
    /// ---
    pub async fn create_schedule(
        &self,
        query_params: CreateScheduleParams,
        body: crate::models::CreateSchedule,
    ) -> Result<Schedule, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, "/schedules", Some(&query_params.qs))?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateScheduleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create one or more overrides
    ///
    /// Create one or more overrides, each for a specific user covering a specified time range. If you create an override on top of an existing override, the last created override will have priority.
    ///
    /// ---
    pub async fn create_schedule_override(
        &self,
        id: &str,
        body: CreateScheduleOverride,
    ) -> Result<Vec<CreateScheduleOverrideResponse>, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/schedules/{}/overrides", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        let response = self.client.process_request(req).await?;

        Praiya::decode_response(response).await
    }

    /// ---
    ///
    /// # Preview a schedule
    ///
    /// Preview what an on-call schedule would look like without saving it.
    ///
    /// ---
    pub async fn create_schedule_preview(
        &self,
        query_params: CreateSchedulePreviewParams,
        body: crate::models::CreateSchedulePreview,
    ) -> Result<Schedule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            "/schedules/preview",
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateSchedulePreviewResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete a schedule
    ///
    /// Delete an on-call schedule.
    ///
    /// ---
    pub async fn delete_schedule(&self, id: &str) -> Result<(), Error> {
        let url = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}", &id), None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Delete an override
    ///
    /// Remove an override.
    ///
    /// ---
    pub async fn delete_schedule_override(&self, id: &str, override_id: &str) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/schedules/{}/overrides/{}", &id, &override_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Get a schedule
    ///
    /// Show detailed information about a schedule, including entries for each layer and sub-schedule.
    ///
    /// ---
    pub async fn get_schedule(
        &self,
        id: &str,
        query_params: GetScheduleParams,
    ) -> Result<Schedule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/schedules/{}", &id),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetScheduleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List overrides
    ///
    /// List overrides for a given time range.
    ///
    /// ---
    pub fn list_schedule_overrides(
        &self,
        id: &str,
        query_params: ListScheduleOverridesParams,
    ) -> impl Stream<Item = Result<ModelOverride, Error>> + '_ {
        self.client
            .list_request::<_, _, ListScheduleOverridesResponse>(
                &self.api_endpoint,
                &format!("/schedules/{}/overrides", &id),
                query_params,
                PraiyaCustomHeaders::None,
            )
    }

    /// ---
    ///
    /// # List users on call.
    ///
    /// List all of the users on call in a given schedule for a given time range.
    ///
    /// ---
    pub fn list_schedule_users(
        &self,
        id: &str,
        query_params: ListScheduleUsersParams,
    ) -> impl Stream<Item = Result<User, Error>> + '_ {
        self.client.list_request::<_, _, ListScheduleUsersResponse>(
            &self.api_endpoint,
            &format!("/schedules/{}/users", &id),
            query_params,
            PraiyaCustomHeaders::None,
        )
    }

    /// ---
    ///
    /// # List schedules
    ///
    /// List the on-call schedules.
    ///
    /// ---
    pub fn list_schedules(
        &self,
        query_params: ListSchedulesParams,
    ) -> impl Stream<Item = Result<Schedule, Error>> + '_ {
        self.client.list_request::<_, _, ListSchedulesResponse>(
            &self.api_endpoint,
            "/schedules",
            query_params,
            PraiyaCustomHeaders::None,
        )
    }

    /// ---
    ///
    /// # List audit records for a schedule
    ///
    /// The returned records are sorted by the `execution_time` from newest to oldest.
    ///
    /// ---
    pub fn list_schedules_audit_records(
        &self,
        id: &str,
        query_params: ListSchedulesAuditRecordsParams,
    ) -> impl Stream<Item = Result<AuditRecord, Error>> + '_ {
        let base_request = BaseRequest {
            host: String::from(&self.api_endpoint),
            method: http::Method::GET,
            options: std::sync::Arc::new(query_params),
            path: format!("/schedules/{}/audit/records", &id),
            headers: std::collections::HashMap::new(),
        };

        self.client.process_into_paginated_stream::<AuditRecord, crate::praiya::PaginatedCursorResponse, crate::praiya::PaginatedCursorPosition, crate::praiya::PaginationCursorQueryComponent>(
            base_request,
            std::sync::Arc::new(crate::praiya::PaginationCursorQueryComponent {
                cursor: None,
                limit: DEFAULT_PAGERDUTY_API_LIMIT,
            }),
        )
        .boxed()
    }

    /// ---
    ///
    /// # Update a schedule
    ///
    /// Update an existing on-call schedule.
    ///
    /// ---
    pub async fn update_schedule(
        &self,
        id: &str,
        query_params: UpdateScheduleParams,
        body: crate::models::UpdateSchedule,
    ) -> Result<Schedule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/schedules/{}", &id),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateScheduleResponse>(req)
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
    async fn test_create_schedule() {
        let pagerduty = crate::Praiya::new("test");
        let create_schedule = CreateSchedule {
            schedule: Schedule {
                name: Some(String::from("Night Shift")),
                time_zone: Some(String::from("America/New_York")),
                description: Some(String::from("Rotation schedule for engineering")),
                schedule_layers: Some(vec![ScheduleLayer {
                    name: Some(String::from("Night Shift")),
                    start: chrono::DateTime::parse_from_rfc3339("2015-11-06T20:00:00-05:00")
                        .unwrap(),
                    rotation_virtual_start: chrono::DateTime::parse_from_rfc3339(
                        "2015-11-06T20:00:00-05:00",
                    )
                    .unwrap(),
                    users: vec![ScheduleLayerUser {
                        user: User {
                            id: Some(String::from("PXPGF42")),
                            _type: UserTypeEnum::USER_REFERENCE,
                            ..Default::default()
                        },
                    }],
                    restrictions: Some(vec![Restriction {
                        _type: RestrictionTypeEnum::DAILY_RESTRICTION,
                        start_time_of_day: String::from("08:00:00"),
                        duration_seconds: 32400,
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        };

        let mut opts_builder = super::CreateScheduleParamsBuilder::new();
        opts_builder.overflow(true);
        let opts = opts_builder.build();

        let schedule = pagerduty
            .schedules()
            .create_schedule(opts, create_schedule)
            .await
            .unwrap();

        assert_eq!(schedule.id, Some(String::from("PI7DH85")));
    }

    #[tokio::test]
    async fn test_create_schedule_override() {
        let pagerduty = crate::Praiya::new("test");

        let create_schedule_override = CreateScheduleOverride {
            overrides: Some(vec![ModelOverride {
                start: chrono::DateTime::parse_from_rfc3339("2012-07-01T00:00:00-04:00").unwrap(),
                end: chrono::DateTime::parse_from_rfc3339("2012-07-02T00:00:00-04:00").unwrap(),
                user: User {
                    id: Some(String::from("PEYSGVA")),
                    _type: UserTypeEnum::USER_REFERENCE,
                    ..Default::default()
                },
                // time_zone: Does not exist in swagger?
                ..Default::default()
            }]),
        };

        let overrides = pagerduty
            .schedules()
            .create_schedule_override("PI7DH85", create_schedule_override)
            .await
            .unwrap();

        assert_eq!(
            overrides[0]._override.id,
            Some(String::from("Q3X6MJ1LUKD6QW"))
        );
    }

    #[tokio::test]
    async fn test_create_schedule_preview() {
        let pagerduty = crate::Praiya::new("test");
        let create_schedule_preview = CreateSchedulePreview {
            schedule: Schedule {
                name: Some(String::from("Daily Engineering Rotation")),
                time_zone: Some(String::from("America/New_York")),
                description: Some(String::from("Rotation schedule for engineering")),
                schedule_layers: Some(vec![ScheduleLayer {
                    name: Some(String::from("Night Shift")),
                    start: chrono::DateTime::parse_from_rfc3339("2015-11-06T20:00:00-05:00")
                        .unwrap(),
                    end: Some(
                        chrono::DateTime::parse_from_rfc3339("2016-11-06T20:00:00-05:00").unwrap(),
                    ),
                    rotation_virtual_start: chrono::DateTime::parse_from_rfc3339(
                        "2015-11-06T20:00:00-05:00",
                    )
                    .unwrap(),
                    users: vec![ScheduleLayerUser {
                        user: User {
                            id: Some(String::from("PXPGF42")),
                            _type: UserTypeEnum::USER_REFERENCE,
                            ..Default::default()
                        },
                    }],
                    restrictions: Some(vec![Restriction {
                        _type: RestrictionTypeEnum::DAILY_RESTRICTION,
                        start_time_of_day: String::from("08:00:00"),
                        duration_seconds: 32400,
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        };

        let mut opts_builder = super::CreateSchedulePreviewParamsBuilder::new();
        opts_builder.overflow(true);
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.since(&since);
        opts_builder.until(&now);
        let opts = opts_builder.build();

        let schedule = pagerduty
            .schedules()
            .create_schedule_preview(opts, create_schedule_preview)
            .await
            .unwrap();

        assert_eq!(schedule.id, Some(String::from("PI7DH85")));
    }

    #[tokio::test]
    async fn test_delete_schedule() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .schedules()
            .delete_schedule("PIJ90N7")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    // Prism doesn't support Content-Type application/json with empty responses
    // #[tokio::test]
    // async fn test_delete_schedule_override() {
    //     env_logger::init();
    //     let pagerduty = crate::Praiya::new("test");
    //     let unit = pagerduty
    //         .schedules()
    //         .delete_schedule_override("PIJ90N7", "Q3X6MJ1LUKD6QW")
    //         .await
    //         .unwrap();
    //     assert_eq!(unit, ());
    // }

    #[tokio::test]
    async fn test_get_schedule() {
        let pagerduty = crate::Praiya::new("test");

        use chrono_tz::America::New_York;

        let mut opts_builder = super::GetScheduleParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.since(&since);
        opts_builder.time_zone(&New_York);
        opts_builder.until(&now);
        let opts = opts_builder.build();

        let schedule = pagerduty
            .schedules()
            .get_schedule("PIJ90N7", opts)
            .await
            .unwrap();

        assert_eq!(schedule.id, Some(String::from("PI7DH85")));
    }

    #[tokio::test]
    async fn test_list_schedule_overrides() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListScheduleOverridesParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.since(&since);
        opts_builder.until(&now);
        opts_builder.editable(true);
        opts_builder.overflow(true);
        let opts = opts_builder.build();

        let schedule_override: Option<ModelOverride> = pagerduty
            .schedules()
            .list_schedule_overrides("PIJ90N7", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            schedule_override.unwrap().id.unwrap(),
            String::from("PQ47DCP")
        );
    }

    #[tokio::test]
    async fn test_list_schedule_users() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListScheduleUsersParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.since(&since);
        opts_builder.until(&now);
        let opts = opts_builder.build();

        let user: Option<User> = pagerduty
            .schedules()
            .list_schedule_users("PIJ90N7", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(user.unwrap().id.unwrap(), String::from("PAM4FGS"));
    }

    #[tokio::test]
    async fn test_list_schedules() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListSchedulesParamsBuilder::new();
        opts_builder.query("");
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let schedule: Option<Schedule> = pagerduty
            .schedules()
            .list_schedules(opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(schedule.unwrap().id.unwrap(), String::from("PI7DH85"));
    }

    #[tokio::test]
    async fn test_list_schedule_audit_records() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListSchedulesAuditRecordsParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.since(&since);
        opts_builder.until(&now);
        let opts = opts_builder.build();

        let record: Option<AuditRecord> = pagerduty
            .schedules()
            .list_schedules_audit_records("PIJ90N7", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            record.unwrap().id,
            String::from("PD_ASSIGN_TEAM_TO_SCHEDULE")
        );
    }

    #[tokio::test]
    async fn test_update_schedule() {
        let pagerduty = crate::Praiya::new("test");
        let update_schedule = UpdateSchedule {
            schedule: Schedule {
                name: Some(String::from("Night Shift")),
                time_zone: Some(String::from("America/New_York")),
                description: Some(String::from("Rotation schedule for engineering")),
                schedule_layers: Some(vec![ScheduleLayer {
                    name: Some(String::from("Night Shift")),
                    start: chrono::DateTime::parse_from_rfc3339("2015-11-06T20:00:00-05:00")
                        .unwrap(),
                    rotation_virtual_start: chrono::DateTime::parse_from_rfc3339(
                        "2015-11-06T20:00:00-05:00",
                    )
                    .unwrap(),
                    users: vec![ScheduleLayerUser {
                        user: User {
                            id: Some(String::from("PXPGF42")),
                            _type: UserTypeEnum::USER_REFERENCE,
                            ..Default::default()
                        },
                    }],
                    restrictions: Some(vec![Restriction {
                        _type: RestrictionTypeEnum::DAILY_RESTRICTION,
                        start_time_of_day: String::from("08:00:00"),
                        duration_seconds: 32400,
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        };

        let mut opts_builder = super::UpdateScheduleParamsBuilder::new();
        opts_builder.overflow(true);
        let opts = opts_builder.build();

        let schedule = pagerduty
            .schedules()
            .update_schedule("PI7DH85", opts, update_schedule)
            .await
            .unwrap();

        assert_eq!(schedule.id, Some(String::from("PI7DH85")));
    }
}
