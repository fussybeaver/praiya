//! Method, error and parameter types for the Rulesets endpoint.
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

pub struct RulesetsClient {
    pub(crate) api_endpoint: String,
    pub(crate) workspace: String,
    pub(crate) client: Praiya,
}



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineListResponse20034 {
    pub offset: usize,
    pub more: bool,
    pub limit: usize,
    pub total: Option<u64>,
    pub inline20034: Vec<Value>,
}

/// Query parameters for the [List event rules](Rulesets::list_ruleset_event_rules()) endpoint.
#[derive(Default, Serialize)]
pub struct RulesetsListRulesetEventRulesParams {
    pub(crate) qs: String,
}

pub struct RulesetsListRulesetEventRulesParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> RulesetsListRulesetEventRulesParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    pub fn build(&mut self) -> RulesetsListRulesetEventRulesParams {
        RulesetsListRulesetEventRulesParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for RulesetsListRulesetEventRulesParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineListResponse20033 {
    pub offset: usize,
    pub more: bool,
    pub limit: usize,
    pub total: Option<u64>,
    pub inline20033: Vec<Value>,
}

/// Query parameters for the [List rulesets](Rulesets::list_rulesets()) endpoint.
#[derive(Default, Serialize)]
pub struct RulesetsListRulesetsParams {
    pub(crate) qs: String,
}

pub struct RulesetsListRulesetsParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> RulesetsListRulesetsParamsBuilder<'req> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new())
        }
    }

    pub fn build(&mut self) -> RulesetsListRulesetsParams {
        RulesetsListRulesetsParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for RulesetsListRulesetsParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

impl RulesetsClient {
    /// ---
    ///
    /// # Create a ruleset
    ///
    /// Create a new ruleset.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub async fn create_ruleset(&self, body: CreateRuleset) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, "/rulesets", "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, RulesetsCreateRulesetResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create an event rule
    ///
    /// Create a new event rule.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub async fn create_ruleset_event_rule(&self, id: &str, body: CreateRulesetEventRule) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/rulesets/{}/rules", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, RulesetsCreateRulesetEventRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete a ruleset
    ///
    /// Delete a ruleset.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub async fn delete_ruleset(&self, id: &str) -> Result<(), Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/rulesets/{}", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::DELETE),
            Body::empty());


        self.client
            .process_into_value::<, RulesetsDeleteRulesetResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete an event rule
    ///
    /// Delete an event rule.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub async fn delete_ruleset_event_rule(&self, id: &str, rule_id: &str) -> Result<(), Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/rulesets/{}/rules/{}", &id, &rule_id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::DELETE),
            Body::empty());


        self.client
            .process_into_value::<, RulesetsDeleteRulesetEventRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get a ruleset
    ///
    /// Get a ruleset.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub async fn get_ruleset(&self, id: &str) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/rulesets/{}", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, RulesetsGetRulesetResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get an event rule
    ///
    /// Get an event rule.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub async fn get_ruleset_event_rule(&self, id: &str, rule_id: &str) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/rulesets/{}/rules/{}", &id, &rule_id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            Body::empty());


        self.client
            .process_into_value::<, RulesetsGetRulesetEventRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List event rules
    ///
    /// List all event rules of a rulesets.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub fn list_ruleset_event_rules(&self, id: &str, query_params: RulesetsListRulesetEventRulesParams) -> impl Stream<Item = Result<Value, Error>> + '_ {
        let base_request = BaseRequest {
            host: String::clone(&self.api_endpoint),
            method: Method::GET,
            options: Arc::new(RulesetsListRulesetEventRulesParamsBuilder::new().build()),
            path: String::from("/rulesets/{}/rules"),
        };

        self.client
            .process_into_paginated_stream::<Value, InlineListResponse20034>(
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
    /// # List rulesets
    ///
    /// List all rulesets.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub fn list_rulesets(&self, query_params: RulesetsListRulesetsParams) -> impl Stream<Item = Result<Value, Error>> + '_ {
        let base_request = BaseRequest {
            host: String::clone(&self.api_endpoint),
            method: Method::GET,
            options: Arc::new(RulesetsListRulesetsParamsBuilder::new().build()),
            path: String::from("/rulesets"),
        };

        self.client
            .process_into_paginated_stream::<Value, InlineListResponse20033>(
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
    /// # Update a ruleset
    ///
    /// Update a ruleset.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub async fn update_ruleset(&self, id: &str, body: UpdateRuleset) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/rulesets/{}", &id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, RulesetsUpdateRulesetResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update an event rule
    ///
    /// Update an event rule. Note that the endpoint supports partial updates, so any number of the writable fields can be provided.
    /// 
    /// Rulesets allow you to route events to an endpoint and create collections of event rules, which define sets of actions to take based on event content.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#rulesets)
    /// 
    /// 
    /// ---
    pub async fn update_ruleset_event_rule(&self, id: &str, rule_id: &str, body: UpdateRulesetEventRule) -> Result<, Error> {
        let uri = Praiya::parse_url(&self.api_endpoint, &format!("/rulesets/{}/rules/{}", &id, &rule_id), "")?;
            
        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(body)?);


        self.client
            .process_into_value::<, RulesetsUpdateRulesetEventRuleResponse>(req)
            .await
    }

}
