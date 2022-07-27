//! Method, error and parameter types for the Audit endpoint.
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

pub struct AuditClient {
    pub(crate) api_endpoint: String,
    pub(crate) workspace: String,
    pub(crate) client: Praiya,
}


/// Query parameters for the [List audit records](Audit::list_audit_records()) endpoint.
#[derive(Default, Serialize)]
pub struct AuditListAuditRecordsParams {
    pub(crate) qs: String,
}

pub struct AuditListAuditRecordsParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> AuditListAuditRecordsParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// The start of the date range over which you want to search. If not specified, defaults to `now() - 24 hours` (past 24 hours)
    pub fn since(&mut self, since: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("since", &serde_urlencoded::to_string(&since).unwrap_or_default());

        self
    }

    /// The end of the date range over which you want to search. If not specified, defaults to `now()`
    pub fn until(&mut self, until: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("until", &serde_urlencoded::to_string(&until).unwrap_or_default());

        self
    }

    /// Resource type filter
    pub fn resource_types<I: IntoIterator<Item = &'req str>>(&mut self, resource_types: I) -> &mut Self {
        for item in resource_types {
            self.qs.append_pair("resource_types[]", &item);
        }
        self
    }

    /// Actor type filter.
    pub fn actor_type(&mut self, actor_type: &'req str) -> &mut Self {
        self.qs.append_pair("actor_type", &actor_type);

        self
    }

    /// Actor Id filter. Must be qualified by providing the `actor_type` param.
    pub fn actor_id(&mut self, actor_id: &'req str) -> &mut Self {
        self.qs.append_pair("actor_id", &actor_id);

        self
    }

    /// Method type filter.
    pub fn method_type(&mut self, method_type: &'req str) -> &mut Self {
        self.qs.append_pair("method_type", &method_type);

        self
    }

    /// Method truncated_token filter. Must be qualified by providing the `method_type` param.
    pub fn method_truncated_token(&mut self, method_truncated_token: &'req str) -> &mut Self {
        self.qs.append_pair("method_truncated_token", &method_truncated_token);

        self
    }

    /// Action filter
    pub fn actions<I: IntoIterator<Item = &'req str>>(&mut self, actions: I) -> &mut Self {
        for item in actions {
            self.qs.append_pair("actions[]", &item);
        }
        self
    }

    pub fn build(&mut self) -> AuditListAuditRecordsParams {
        AuditListAuditRecordsParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for AuditListAuditRecordsParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

impl AuditClient {
    /// ---
    ///
    /// # List audit records
    ///
    /// List audit trail records matching provided query params or default criteria.
    /// 
    /// The returned records are sorted by the `execution_time` from newest to oldest.
    /// 
    /// See [`Cursor-based pagination`](https://developer.pagerduty.com/docs/rest-api-v2/pagination/) for instructions on how to paginate through the result set.
    /// 
    /// Only admins, account owners, or global API tokens can access this endpoint.
    /// 
    /// For other role based access to audit records by resource ID, see the resource's API documentation. 
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#audit-record).
    /// <!-- theme: warning -->
    /// > ### Early Access
    /// > This endpoint's interface is under development and subject to change. Do not use it in production systems.
    /// > Your request must set an X-EARLY-ACCESS header with value `audit-early-access` to acknowledge this.
    /// >
    /// > Audit records for user and team resources started in August 2020 and records may be purged while the API is in early access.
    /// 
    /// 
    /// ---
    pub async fn list_audit_records(&self, query_params: AuditListAuditRecordsParams) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, "/audit/records", &AuditListAuditRecordsParamsBuilder::new().build().qs)?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, AuditListAuditRecordsResponse>(req)
            .await
    }

}