//! Method, error and parameter types for the Notifications endpoint.
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

pub struct NotificationsClient {
    pub(crate) api_endpoint: String,
    pub(crate) workspace: String,
    pub(crate) client: Praiya,
}



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineListResponse20028 {
    pub offset: usize,
    pub more: bool,
    pub limit: usize,
    pub total: Option<u64>,
    pub inline20028: Vec<Notification>,
}

/// Query parameters for the [List notifications](Notifications::list_notifications()) endpoint.
#[derive(Default, Serialize)]
pub struct NotificationsListNotificationsParams {
    pub(crate) qs: String,
}

pub struct NotificationsListNotificationsParamsBuilder<'req> {
    qs: form_urlencoded::Serializer<'req, String>,
}

impl<'req> NotificationsListNotificationsParamsBuilder<'req> {
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

    /// The start of the date range over which you want to search. The time element is optional.
    pub fn since(&mut self, since: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("since", &serde_urlencoded::to_string(&since).unwrap_or_default());

        self
    }

    /// The end of the date range over which you want to search. This should be in the same format as since. The size of the date range must be less than 3 months.
    pub fn until(&mut self, until: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.qs.append_pair("until", &serde_urlencoded::to_string(&until).unwrap_or_default());

        self
    }

    /// Return notification of this type only.
    pub fn filter(&mut self, filter: &'req str) -> &mut Self {
        self.qs.append_pair("filter", &filter);

        self
    }

    /// Array of additional details to include.
    pub fn include<I: IntoIterator<Item = &'req str>>(&mut self, include: I) -> &mut Self {
        for item in include {
            self.qs.append_pair("include[]", &item);
        }
        self
    }

    pub fn build(&mut self) -> NotificationsListNotificationsParams {
        NotificationsListNotificationsParams {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for NotificationsListNotificationsParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
      let mut query = form_urlencoded::Serializer::new(self.qs.clone());
      query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("offset", &pagination.limit.to_string());
        query.finish()
    }
}

impl NotificationsClient {
    /// ---
    ///
    /// # List notifications
    ///
    /// List notifications for a given time range, optionally filtered by type (sms_notification, email_notification, phone_notification, or push_notification).
    /// 
    /// A Notification is created when an Incident is triggered or escalated.
    /// 
    /// For more information see the [API Concepts Document](../../docs/CONCEPTS.md#notifications)
    /// 
    /// 
    /// ---
    pub fn list_notifications(&self, query_params: NotificationsListNotificationsParams) -> impl Stream<Item = Result<Notification, Error>> + '_ {
        let base_request = BaseRequest {
            host: String::clone(&self.api_endpoint),
            method: Method::GET,
            options: Arc::new(NotificationsListNotificationsParamsBuilder::new().build()),
            path: String::from("/notifications"),
        };

        self.client
            .process_into_paginated_stream::<Notification, InlineListResponse20028>(
                base_request,
                PaginationQueryComponent {
                    offset: 0,
                    limit: DEFAULT_PAGERDUTY_API_LIMIT,
                },
            )
            .boxed()

    }

}