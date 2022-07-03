//! Method, error and parameter types for the ExtensionSchemas endpoint.
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

pub struct ExtensionSchemasClient {
    pub(crate) api_endpoint: String,
    pub(crate) workspace: String,
    pub(crate) client: Praiya,
}



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineListResponse20012 {
    pub offset: usize,
    pub more: bool,
    pub limit: usize,
    pub total: Option<u64>,
    pub inline20012: Vec<ExtensionSchema>,
}

/// Query parameters for the [List extension schemas](ExtensionSchemas::list_extension_schemas()) endpoint.
#[derive(Default, Serialize)]
pub struct ExtensionSchemasListExtensionSchemasParams {
    pub(crate) qs: String,
}

pub struct ExtensionSchemasListExtensionSchemasParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> ExtensionSchemasListExtensionSchemasParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    pub fn build(&mut self) -> ExtensionSchemasListExtensionSchemasParams {
        ExtensionSchemasListExtensionSchemasParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for ExtensionSchemasListExtensionSchemasParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

impl ExtensionSchemasClient {
    /// ---
    ///
    /// # Get an extension vendor
    ///
    /// Get details about one specific extension vendor.
    /// 
    /// A PagerDuty extension vendor represents a specific type of outbound extension such as Generic Webhook, Slack, ServiceNow.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#extension-schemas)
    /// 
    /// 
    /// ---
    pub async fn get_extension_schema(&self, id: &str) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/extension_schemas/{}", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, ExtensionSchemasGetExtensionSchemaResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List extension schemas
    ///
    /// List all extension schemas.
    /// 
    /// A PagerDuty extension vendor represents a specific type of outbound extension such as Generic Webhook, Slack, ServiceNow.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#extension-schemas)
    /// 
    /// 
    /// ---
    pub fn list_extension_schemas(&self, query_params: ExtensionSchemasListExtensionSchemasParams) -> impl Stream<Item = Result<ExtensionSchema, Error>> + '_ {
        let base_request = BaseRequest {
            host: String::clone(&self.api_endpoint),
            method: Method::GET,
            options: Arc::new(ExtensionSchemasListExtensionSchemasParamsBuilder::new().build()),
            path: String::from("/extension_schemas"),
        };

        self.client
            .process_into_paginated_stream::<ExtensionSchema, InlineListResponse20012>(
                base_request,
                PaginationQueryComponent {
                    offset: 0,
                    limit: DEFAULT_PAGERDUTY_API_LIMIT,
                },
            )
            .boxed()

    }

}
