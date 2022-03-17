//! Method, error and parameter types for the LogEntries endpoint.
#![allow(
    unused_imports,
)]
/* 
 * PagerDuty API
 *
 * This document describes the PagerDuty REST APIs.  For guides and examples please visit our [Documentation.](https://developer.pagerduty.com/docs/get-started/getting-started/)  Our REST APIs are defined in OpenAPI v3.x. You can view the schema at [github.com/PagerDuty/api-schema](https://github.com/PagerDuty/api-schema). 
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@pagerduty.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::Deserialize;

use crate::models::*;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct LogEntriesClient {
    pub(crate) api_endpoint: String,
    pub(crate) client: PagerDuty,
}


/// Query parameters for the [Get a log entry](LogEntries::get_log_entry()) endpoint.
#[derive(Default, Serialize)]
pub struct LogEntriesGetLogEntryParams<'req> {
    pub(crate) qs: String,
}

pub struct LogEntriesGetLogEntryParamsBuilder {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> LogEntriesGetLogEntryParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// Time zone in which dates in the result will be rendered.
    pub fn time_zone(&mut self, time_zone: &'req str) -> &mut Self {
        self.qs.append_pair("time_zone", serde_urlencoded::to_string(time_zone));

        self
    }

    /// Array of additional Models to include in response.
    pub fn include<I: IntoIterator<Item = &'req str>>(&mut self, include: I) -> &mut Self {
        for item in include {
            self.qs.append_pair("include[]", serde_urlencoded::to_string(item));
        }
        self
    }
}


impl BaseOption for LogEntriesGetLogEntryParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
        let mut query = form_urlencoded::Serializer::new(form_urlencoded::Serializer::Clone(&self.qs))
        query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}
/// Query parameters for the [List log entries](LogEntries::list_log_entries()) endpoint.
#[derive(Default, Serialize)]
pub struct LogEntriesListLogEntriesParams<'req> {
    pub(crate) qs: String,
}

pub struct LogEntriesListLogEntriesParamsBuilder {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> LogEntriesListLogEntriesParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// Time zone in which dates in the result will be rendered.
    pub fn time_zone(&mut self, time_zone: &'req str) -> &mut Self {
        self.qs.append_pair("time_zone", serde_urlencoded::to_string(time_zone));

        self
    }

    /// The start of the date range over which you want to search.
    pub fn since(&mut self, since: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("since", serde_urlencoded::to_string(since));

        self
    }

    /// The end of the date range over which you want to search.
    pub fn until(&mut self, until: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("until", serde_urlencoded::to_string(until));

        self
    }

    /// If `true`, will return a subset of log entries that show only the most important changes to the incident.
    pub fn is_overview(&mut self, is_overview: bool) -> &mut Self {
        self.qs.append_pair("is_overview", serde_urlencoded::to_string(is_overview));

        self
    }

    /// Array of additional Models to include in response.
    pub fn include<I: IntoIterator<Item = &'req str>>(&mut self, include: I) -> &mut Self {
        for item in include {
            self.qs.append_pair("include[]", serde_urlencoded::to_string(item));
        }
        self
    }
}


impl BaseOption for LogEntriesListLogEntriesParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
        let mut query = form_urlencoded::Serializer::new(form_urlencoded::Serializer::Clone(&self.qs))
        query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

impl BaseOption for LogEntriesUpdateLogEntryChannelParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
        let mut query = form_urlencoded::Serializer::new(form_urlencoded::Serializer::Clone(&self.qs))
        query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

impl LogEntriesClient {
    /// ---
    ///
    /// # Get a log entry
    ///
    /// Get details for a specific incident log entry. This method provides additional information you can use to get at raw event data.
    /// 
    /// A log of all the events that happen to an Incident, and these are exposed as Log Entries.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#log-entries)
    /// 
    /// 
    /// ---
    pub async fn get_log_entry(&self, id: &str, query_params: LogEntriesGetLogEntryParams) -> Result<GetLogEntryResponse, Error> {
        let uri = PagerDuty::parse_url(&self.api_endpoint, format!("{}/{}", &self.path(), &id), LogEntriesGetLogEntryParamsBuilder::new().build().qs)?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value(req)
            .await
    }

    /// ---
    ///
    /// # List log entries
    ///
    /// List all of the incident log entries across the entire account.
    /// 
    /// A log of all the events that happen to an Incident, and these are exposed as Log Entries.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#log-entries)
    /// 
    /// 
    /// ---
    pub fn list_log_entries(&self, query_params: LogEntriesListLogEntriesParams) -> impl Stream<Item = Result<LogEntry, Error>> + '_ {
        let base_request = BaseRequest {
            host: String::clone(&self.api_endpoint),
            method: Method::GET,
            options: Arc::new(LogEntriesListLogEntriesParamsBuilder::new().build()),
            path: self.path(),
        };

        self.client
            .process_into_paginated_stream::<LogEntriesResponse, >
    }

    /// ---
    ///
    /// # Update log entry channel information.
    ///
    /// Update an existing incident log entry channel.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#log-entries)
    /// 
    /// 
    /// ---
    pub async fn update_log_entry_channel(&self, id: &str, body: LogEntryChannel) -> Result<(), Error> {
        let uri = PagerDuty::parse_url(&self.api_endpoint, format!("{}/{}", &self.path(), &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::PUT),
            Some(PagerDuty::serialize_payload(LogEntryChannel)?));


        self.client
            .process_into_value(req)
            .await
    }

}
