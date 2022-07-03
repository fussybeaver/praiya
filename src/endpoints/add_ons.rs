//! Method, error and parameter types for the AddOns endpoint.
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

pub struct AddOnsClient {
    pub(crate) api_endpoint: String,
    pub(crate) workspace: String,
    pub(crate) client: Praiya,
}



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineListResponse2002 {
    pub offset: usize,
    pub more: bool,
    pub limit: usize,
    pub total: Option<u64>,
    pub inline2002: Vec<AddonReference>,
}

/// Query parameters for the [List installed Add-ons](AddOns::list_addon()) endpoint.
#[derive(Default, Serialize)]
pub struct AddOnsListAddonParams {
    pub(crate) qs: String,
}

pub struct AddOnsListAddonParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> AddOnsListAddonParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    /// Array of additional Models to include in response.
    pub fn include<I: IntoIterator<Item = &'req str>>(&mut self, include: I) -> &mut Self {
        for item in include {
            self.qs.append_pair("include[]", &item);
        }
        self
    }

    /// Filters the results, showing only Add-ons for the given services
    pub fn service_ids<I: IntoIterator<Item = &'req str>>(&mut self, service_ids: I) -> &mut Self {
        for item in service_ids {
            self.qs.append_pair("service_ids[]", &item);
        }
        self
    }

    /// Filters the results, showing only Add-ons of the given type
    pub fn filter(&mut self, filter: &'req str) -> &mut Self {
        self.qs.append_pair("filter", &filter);

        self
    }

    pub fn build(&mut self) -> AddOnsListAddonParams {
        AddOnsListAddonParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for AddOnsListAddonParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

impl AddOnsClient {
    /// ---
    ///
    /// # Install an Add-on
    ///
    /// Install an Add-on for your account.
    /// 
    /// Addon's are pieces of functionality that developers can write to insert new functionality into PagerDuty's UI.
    /// 
    /// Given a configuration containing a `src` parameter, that URL will be embedded in an `iframe` on a page that's available to users from a drop-down menu.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#add-ons)
    /// 
    /// 
    /// ---
    pub async fn create_addon(&self, body: CreateAddon) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, "/addons", "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, AddOnsCreateAddonResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete an Add-on
    ///
    /// Remove an existing Add-on.
    /// 
    /// Addon's are pieces of functionality that developers can write to insert new functionality into PagerDuty's UI.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#add-ons)
    /// 
    /// 
    /// ---
    pub async fn delete_addon(&self, id: &str) -> Result<(), Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/addons/{}", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::DELETE),
            Body::empty());


        self.client
            .process_into_value::<, AddOnsDeleteAddonResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get an Add-on
    ///
    /// Get details about an existing Add-on.
    /// 
    /// Addon's are pieces of functionality that developers can write to insert new functionality into PagerDuty's UI.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#add-ons)
    /// 
    /// 
    /// ---
    pub async fn get_addon(&self, id: &str) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/addons/{}", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, AddOnsGetAddonResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List installed Add-ons
    ///
    /// List all of the Add-ons installed on your account.
    /// 
    /// Addon's are pieces of functionality that developers can write to insert new functionality into PagerDuty's UI.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#add-ons)
    /// 
    /// 
    /// ---
    pub fn list_addon(&self, query_params: AddOnsListAddonParams) -> impl Stream<Item = Result<AddonReference, Error>> + '_ {
        let base_request = BaseRequest {
            host: String::clone(&self.api_endpoint),
            method: Method::GET,
            options: Arc::new(AddOnsListAddonParamsBuilder::new().build()),
            path: String::from("/addons"),
        };

        self.client
            .process_into_paginated_stream::<AddonReference, InlineListResponse2002>(
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
    /// # Update an Add-on
    ///
    /// Update an existing Add-on.
    /// 
    /// Addon's are pieces of functionality that developers can write to insert new functionality into PagerDuty's UI.
    /// 
    /// Given a configuration containing a `src` parameter, that URL will be embedded in an `iframe` on a page that's available to users from a drop-down menu.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#add-ons)
    /// 
    /// 
    /// ---
    pub async fn update_addon(&self, id: &str, body: UpdateAddon) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/addons/{}", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, AddOnsUpdateAddonResponse>(req)
            .await
    }

}
