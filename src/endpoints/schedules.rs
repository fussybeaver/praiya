//! Method, error and parameter types for the Schedules endpoint.
#![allow(
    unused_imports,
)]
/* 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


use crate::{
    BaseOption, BaseRequest, Praiya, PaginatedResponse, PaginationQueryComponent,
    SingleResponse, SubSystem, DEFAULT_PAGERDUTY_API_LIMIT
};
use crate::errors::Error;
use crate::models::*;

use std::collections::HashMap;
use std::sync::Arc;

use futures_core::Stream;
use futures_util::stream::StreamExt;
use http::request::Builder;
use hyper::{Body, Method};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use url::form_urlencoded;

pub struct SchedulesClient {
    pub(crate) api_endpoint: String,
    pub(crate) workspace: String,
    pub(crate) client: Praiya,
}


/// Query parameters for the [Create a schedule](Schedules::create_schedule()) endpoint.
#[derive(Default, Serialize)]
pub struct SchedulesCreateScheduleParams {
    pub(crate) qs: String,
}

pub struct SchedulesCreateScheduleParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> SchedulesCreateScheduleParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// Any on-call schedule entries that pass the date range bounds will be truncated at the bounds, unless the parameter `overflow=true` is passed. This parameter defaults to false. For instance, if your schedule is a rotation that changes daily at midnight UTC, and your date range is from `2011-06-01T10:00:00Z` to `2011-06-01T14:00:00Z`:   - If you don't pass the `overflow=true` parameter, you will get one schedule entry returned with a start of `2011-06-01T10:00:00Z` and end of `2011-06-01T14:00:00Z`. - If you do pass the `overflow=true` parameter, you will get one schedule entry returned with a start of `2011-06-01T00:00:00Z` and end of `2011-06-02T00:00:00Z`. 
    pub fn overflow(&mut self, overflow: bool) -> &mut Self {
        self.qs.append_pair("overflow", &serde_urlencoded::to_string(&overflow).unwrap_or_default());

        self
    }

    pub fn build(&mut self) -> SchedulesCreateScheduleParams {
        SchedulesCreateScheduleParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for SchedulesCreateScheduleParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}
/// Query parameters for the [Preview a schedule](Schedules::create_schedule_preview()) endpoint.
#[derive(Default, Serialize)]
pub struct SchedulesCreateSchedulePreviewParams {
    pub(crate) qs: String,
}

pub struct SchedulesCreateSchedulePreviewParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> SchedulesCreateSchedulePreviewParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// The start of the date range over which you want to search.
    pub fn since(&mut self, since: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("since", &serde_urlencoded::to_string(&since).unwrap_or_default());

        self
    }

    /// The end of the date range over which you want to search.
    pub fn until(&mut self, until: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("until", &serde_urlencoded::to_string(&until).unwrap_or_default());

        self
    }

    /// Any on-call schedule entries that pass the date range bounds will be truncated at the bounds, unless the parameter `overflow=true` is passed. This parameter defaults to false. For instance, if your schedule is a rotation that changes daily at midnight UTC, and your date range is from `2011-06-01T10:00:00Z` to `2011-06-01T14:00:00Z`:   - If you don't pass the `overflow=true` parameter, you will get one schedule entry returned with a start of `2011-06-01T10:00:00Z` and end of `2011-06-01T14:00:00Z`. - If you do pass the `overflow=true` parameter, you will get one schedule entry returned with a start of `2011-06-01T00:00:00Z` and end of `2011-06-02T00:00:00Z`. 
    pub fn overflow(&mut self, overflow: bool) -> &mut Self {
        self.qs.append_pair("overflow", &serde_urlencoded::to_string(&overflow).unwrap_or_default());

        self
    }

    pub fn build(&mut self) -> SchedulesCreateSchedulePreviewParams {
        SchedulesCreateSchedulePreviewParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for SchedulesCreateSchedulePreviewParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}
/// Query parameters for the [Get a schedule](Schedules::get_schedule()) endpoint.
#[derive(Default, Serialize)]
pub struct SchedulesGetScheduleParams {
    pub(crate) qs: String,
}

pub struct SchedulesGetScheduleParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> SchedulesGetScheduleParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// Time zone in which dates in the result will be rendered.
    pub fn time_zone(&mut self, time_zone: &'req str) -> &mut Self {
        self.qs.append_pair("time_zone", &time_zone);

        self
    }

    /// The start of the date range over which you want to search.
    pub fn since(&mut self, since: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("since", &serde_urlencoded::to_string(&since).unwrap_or_default());

        self
    }

    /// The end of the date range over which you want to search.
    pub fn until(&mut self, until: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("until", &serde_urlencoded::to_string(&until).unwrap_or_default());

        self
    }

    pub fn build(&mut self) -> SchedulesGetScheduleParams {
        SchedulesGetScheduleParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for SchedulesGetScheduleParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}
/// Query parameters for the [List overrides](Schedules::list_schedule_overrides()) endpoint.
#[derive(Default, Serialize)]
pub struct SchedulesListScheduleOverridesParams {
    pub(crate) qs: String,
}

pub struct SchedulesListScheduleOverridesParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> SchedulesListScheduleOverridesParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// The start of the date range over which you want to search.
    pub fn since(&mut self, since: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("since", &serde_urlencoded::to_string(&since).unwrap_or_default());

        self
    }

    /// The end of the date range over which you want to search.
    pub fn until(&mut self, until: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("until", &serde_urlencoded::to_string(&until).unwrap_or_default());

        self
    }

    /// When this parameter is present, only editable overrides will be returned. The result will only include the id of the override if this parameter is present. Only future overrides are editable.
    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.qs.append_pair("editable", &serde_urlencoded::to_string(&editable).unwrap_or_default());

        self
    }

    /// Any on-call schedule entries that pass the date range bounds will be truncated at the bounds, unless the parameter overflow=true is passed. This parameter defaults to false.
    pub fn overflow(&mut self, overflow: bool) -> &mut Self {
        self.qs.append_pair("overflow", &serde_urlencoded::to_string(&overflow).unwrap_or_default());

        self
    }

    pub fn build(&mut self) -> SchedulesListScheduleOverridesParams {
        SchedulesListScheduleOverridesParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for SchedulesListScheduleOverridesParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}
/// Query parameters for the [List users on call.](Schedules::list_schedule_users()) endpoint.
#[derive(Default, Serialize)]
pub struct SchedulesListScheduleUsersParams {
    pub(crate) qs: String,
}

pub struct SchedulesListScheduleUsersParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> SchedulesListScheduleUsersParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// The start of the date range over which you want to search.
    pub fn since(&mut self, since: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("since", &serde_urlencoded::to_string(&since).unwrap_or_default());

        self
    }

    /// The end of the date range over which you want to search.
    pub fn until(&mut self, until: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("until", &serde_urlencoded::to_string(&until).unwrap_or_default());

        self
    }

    pub fn build(&mut self) -> SchedulesListScheduleUsersParams {
        SchedulesListScheduleUsersParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for SchedulesListScheduleUsersParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineListResponse20035 {
    pub offset: usize,
    pub more: bool,
    pub limit: usize,
    pub total: Option<u64>,
    pub inline20035: Vec<Schedule>,
}

/// Query parameters for the [List schedules](Schedules::list_schedules()) endpoint.
#[derive(Default, Serialize)]
pub struct SchedulesListSchedulesParams {
    pub(crate) qs: String,
}

pub struct SchedulesListSchedulesParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> SchedulesListSchedulesParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// Filters the result, showing only the schedules whose name matches the query.
    pub fn query(&mut self, query: &'req str) -> &mut Self {
        self.qs.append_pair("query", &query);

        self
    }

    pub fn build(&mut self) -> SchedulesListSchedulesParams {
        SchedulesListSchedulesParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for SchedulesListSchedulesParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}
/// Query parameters for the [Update a schedule](Schedules::update_schedule()) endpoint.
#[derive(Default, Serialize)]
pub struct SchedulesUpdateScheduleParams {
    pub(crate) qs: String,
}

pub struct SchedulesUpdateScheduleParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> SchedulesUpdateScheduleParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// Any on-call schedule entries that pass the date range bounds will be truncated at the bounds, unless the parameter `overflow=true` is passed. This parameter defaults to false. For instance, if your schedule is a rotation that changes daily at midnight UTC, and your date range is from `2011-06-01T10:00:00Z` to `2011-06-01T14:00:00Z`:   - If you don't pass the `overflow=true` parameter, you will get one schedule entry returned with a start of `2011-06-01T10:00:00Z` and end of `2011-06-01T14:00:00Z`. - If you do pass the `overflow=true` parameter, you will get one schedule entry returned with a start of `2011-06-01T00:00:00Z` and end of `2011-06-02T00:00:00Z`. 
    pub fn overflow(&mut self, overflow: bool) -> &mut Self {
        self.qs.append_pair("overflow", &serde_urlencoded::to_string(&overflow).unwrap_or_default());

        self
    }

    pub fn build(&mut self) -> SchedulesUpdateScheduleParams {
        SchedulesUpdateScheduleParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for SchedulesUpdateScheduleParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

impl SchedulesClient {
    /// ---
    ///
    /// # Create a schedule
    ///
    /// Create a new on-call schedule.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub async fn create_schedule(&self, query_params: SchedulesCreateScheduleParams, body: CreateSchedule) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, "/schedules", &SchedulesCreateScheduleParamsBuilder::new().build().qs)?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, SchedulesCreateScheduleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create an override
    ///
    /// Create an override for a specific user covering the specified time range. If you create an override on top of an existing one, the last created override will have priority.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub async fn create_schedule_override(&self, id: &str, body: CreateScheduleOverride) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}/overrides", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, SchedulesCreateScheduleOverrideResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Preview a schedule
    ///
    /// Preview what an on-call schedule would look like without saving it.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub async fn create_schedule_preview(&self, query_params: SchedulesCreateSchedulePreviewParams, body: CreateSchedulePreview) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, "/schedules/preview", &SchedulesCreateSchedulePreviewParamsBuilder::new().build().qs)?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, SchedulesCreateSchedulePreviewResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete a schedule
    ///
    /// Delete an on-call schedule.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub async fn delete_schedule(&self, id: &str) -> Result<(), Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::DELETE),
            Body::empty());


        self.client
            .process_into_value::<, SchedulesDeleteScheduleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete an override
    ///
    /// Remove an override. 
    /// 
    /// You cannot remove a past override. 
    /// 
    /// If the override start time is before the current time, but the end time is after the current time, the override will be truncated to the current time. 
    /// 
    /// If the override is truncated, the status code will be 200 OK, as opposed to a 204 No Content for a successful delete.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub async fn delete_schedule_override(&self, id: &str, override_id: &str) -> Result<(), Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}/overrides/{}", &id, &override_id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::DELETE),
            Body::empty());


        self.client
            .process_into_value::<, SchedulesDeleteScheduleOverrideResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get a schedule
    ///
    /// Show detailed information about a schedule, including entries for each layer and sub-schedule.
    /// 
    /// ---
    pub async fn get_schedule(&self, id: &str, query_params: SchedulesGetScheduleParams) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}", &id), &SchedulesGetScheduleParamsBuilder::new().build().qs)?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, SchedulesGetScheduleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List overrides
    ///
    /// List overrides for a given time range.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub async fn list_schedule_overrides(&self, id: &str, query_params: SchedulesListScheduleOverridesParams) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}/overrides", &id), &SchedulesListScheduleOverridesParamsBuilder::new().build().qs)?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, SchedulesListScheduleOverridesResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List users on call.
    ///
    /// List all of the users on call in a given schedule for a given time range.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub async fn list_schedule_users(&self, id: &str, query_params: SchedulesListScheduleUsersParams) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}/users", &id), &SchedulesListScheduleUsersParamsBuilder::new().build().qs)?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, SchedulesListScheduleUsersResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List schedules
    ///
    /// List the on-call schedules.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub fn list_schedules(&self, query_params: SchedulesListSchedulesParams) -> impl Stream<Item = Result<Schedule, Error>> + '_ {
        let base_request = BaseRequest {
            host: String::clone(&self.api_endpoint),
            method: Method::GET,
            options: Arc::new(SchedulesListSchedulesParamsBuilder::new().build()),
            path: String::from("/schedules"),
        };

        self.client
            .process_into_paginated_stream::<Schedule, InlineListResponse20035>(
                base_request,
                PaginationQueryComponent {
                    offset: 0,
                    limit: DEFAULT_PAGERDUTY_API_LIMIT,
                },
            )
            .boxed()

    }

    /// ---
    ///
    /// # List audit records for a schedule
    ///
    /// The returned records are sorted by the `execution_time` from newest to oldest.
    /// 
    /// See [`Cursor-based pagination`](https://developer.pagerduty.com/docs/rest-api-v2/pagination/) for instructions on how to paginate through the result set.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#audit-record).
    /// 
    /// <!-- theme: warning -->
    /// > ### Early Access
    /// > This endpoint's interface is under development and subject to change. Do not use it in production systems.
    /// > Your request must set an X-EARLY-ACCESS header with value `audit-early-access` to acknowledge this.
    /// >
    /// > Audit records for schedules resource started in August 2020 and records may be purged while the API is in early access.
    /// 
    /// 
    /// ---
    pub async fn list_schedules_audit_records(&self, id: &str) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}/audit/records", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, SchedulesListSchedulesAuditRecordsResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update a schedule
    ///
    /// Update an existing on-call schedule.
    /// 
    /// A Schedule determines the time periods that users are On-Call.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#schedules)
    /// 
    /// 
    /// ---
    pub async fn update_schedule(&self, id: &str, query_params: SchedulesUpdateScheduleParams, body: UpdateSchedule) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/schedules/{}", &id), &SchedulesUpdateScheduleParamsBuilder::new().build().qs)?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, SchedulesUpdateScheduleResponse>(req)
            .await
    }

}