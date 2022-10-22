//! Method, error and parameter types for the Incidents endpoint.

use futures_core::Stream;
use futures_util::StreamExt;
use http::header::FROM;
use http::request::Builder;
use http::Method;
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
pub struct UsersClient {
    pub(crate) api_endpoint: String,
    pub(crate) client: Praiya,
}

impl Praiya {
    pub fn users(&self) -> UsersClient {
        UsersClient {
            api_endpoint: std::env::var("PAGERDUTY_API_ENDPOINT")
                .unwrap_or_else(|_| String::from(API_ENDPOINT)),
            client: Praiya::clone(self),
        }
    }
}

single_response_type!(User, user, CreateUser);

single_response_type!(ContactMethod, contact_method, CreateContactMethod);

single_response_type!(
    HandoffNotificationRule,
    oncall_handoff_notification_rule,
    CreateUserHandoffNotificationRule
);

single_response_type!(NotificationRule, notification_rule, CreateNotificationRule);

plural_response_type!(
    NotificationSubscriptionWithContext,
    subscriptions,
    CreateUserNotificationSubscribers
);

single_response_type!(
    StatusUpdateNotificationRule,
    status_update_notification_rule,
    CreateStatusUpdateNotificationRule
);

single_response_type!(User, user, GetCurrentUser);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[UsersClient::get_current_user]"]
#[allow(dead_code)]
struct GetCurrentUser {
    include: Vec<String>,
}

single_response_type!(User, user, GetUser);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[UsersClient::get_user]"]
#[allow(dead_code)]
struct GetUser {
    include: Vec<String>,
}

// Declared here, because InlineResponse is filtered in the swagger template,
// and lacking support for better named InlineResponse
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserContactMethodEnum {
    PHONE_CONTACT_METHOD(PhoneContactMethod),
    PUSH_CONTACT_METHOD(PushContactMethod),
    EMAIL_CONTACT_METHOD(EmailContactMethod),
}

single_response_type!(
    GetUserContactMethodEnum,
    contact_method,
    GetUserContactMethod
);

single_response_type!(
    HandoffNotificationRule,
    oncall_handoff_notification_rule,
    GetUserHandoffNotificationRule
);

list_response_type!(
    ListUserHandoffNotificationRules,
    oncall_handoff_notification_rules,
    HandoffNotificationRule
);

list_response_type!(
    ListUserContactMethods,
    contact_methods,
    GetUserContactMethodEnum
);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[UsersClient::get_user_notification_rule]"]
#[allow(dead_code)]
struct GetUserNotificationRule {
    include: Vec<String>,
}

single_response_type!(NotificationRule, notification_rule, GetUserNotificationRule);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[UsersClient::list_user_notification_rules]"]
#[allow(dead_code)]
struct GetUserNotificationRules {
    include: Vec<String>,
}

list_response_type!(
    ListUserNotificationRules,
    notification_rules,
    NotificationRule
);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[UsersClient::get_user_status_update_notification_rule]"]
#[allow(dead_code)]
struct GetUserStatusUpdateNotificationRule {
    include: Vec<String>,
}

single_response_type!(
    StatusUpdateNotificationRule,
    notification_rule,
    GetUserStatusUpdateNotificationRule
);

single_response_type!(UserSession, user_session, GetUserSession);

plural_response_type!(UserSession, user_sessions, GetUserSessions);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[UsersClient::get_user_status_update_notification_rules]"]
#[allow(dead_code)]
struct GetUserStatusUpdateNotificationRules {
    include: Vec<String>,
}

list_response_type!(
    ListUserStatusUpdateNotificationRules,
    status_update_notification_rules,
    StatusUpdateNotificationRule
);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[UsersClient::list_users]"]
#[allow(dead_code)]
struct ListUsers {
    query: String,
    include: Vec<String>,
    team_ids: Vec<String>,
}

list_response_type!(ListUsers, users, User);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[ServicesClient::list_users_audit_records]"]
#[allow(dead_code)]
struct ListUsersAuditRecords {
    since: chrono::DateTime<chrono::Utc>,
    until: chrono::DateTime<chrono::Utc>,
}

single_response_type!(User, user, UpdateUser);

single_response_type!(
    UpdateUserContactMethodContactMethodEnum,
    contact_method,
    UpdateUserContactMethod
);

single_response_type!(
    HandoffNotificationRule,
    oncall_handoff_notification_rule,
    UpdateUserHandoffNotificationRule
);

single_response_type!(
    NotificationRule,
    notification_rule,
    UpdateUserNotificationRule
);

single_response_type!(
    StatusUpdateNotificationRule,
    status_update_notification_rule,
    UpdateUserStatusUpdateNotificationRule
);

impl UsersClient {
    /// ---
    ///
    /// # Create a user
    ///
    /// Create a new user. Note that you must also supply a `password` property to create a user--it will not be returned by any API.
    ///
    ///
    /// ---
    pub async fn create_user(&self, body: CreateUser, from_email: &str) -> Result<User, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, "/users", None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new()
                .header(FROM, from_email)
                .method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateUserResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create a user contact method
    ///
    /// Create a new contact method for the User.
    ///
    ///
    /// ---
    pub async fn create_user_contact_method(
        &self,
        id: &str,
        body: CreateUserContactMethod,
    ) -> Result<ContactMethod, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/contact_methods", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateContactMethodResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create a User Handoff Notification Rule
    ///
    /// Create a new Handoff Notification Rule.
    ///
    ///
    /// ---
    pub async fn create_user_handoff_notification_rule(
        &self,
        id: &str,
        body: CreateUserHandoffNotificationRule,
    ) -> Result<HandoffNotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/oncall_handoff_notification_rules", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateUserHandoffNotificationRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create a user notification rule
    ///
    /// Create a new notification rule.
    ///
    ///
    /// ---
    pub async fn create_user_notification_rule(
        &self,
        id: &str,
        body: CreateUserNotificationRule,
    ) -> Result<NotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/notification_rules", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateNotificationRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create notification subcription
    ///
    /// Create a new Notification Subscription for the given user.
    ///
    ///
    /// ---
    pub async fn create_user_notification_subscriptions(
        &self,
        id: &str,
        body: CreateUserNotificationSubscriptions,
    ) -> Result<Vec<NotificationSubscriptionWithContext>, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/notification_subscriptions", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateUserNotificationSubscribersResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create a user status update notification rule
    ///
    /// Create a new status update notification rule.
    ///
    ///
    /// ---
    pub async fn create_user_status_update_notification_rule(
        &self,
        id: &str,
        body: CreateUserStatusUpdateNotificationRule,
    ) -> Result<StatusUpdateNotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/status_update_notification_rules", &id),
            None,
        )?;

        let mut builder = http::request::Builder::new();
        let early_access: &str = PraiyaCustomHeaders::EarlyAccess(None).into();
        builder = builder.header(early_access, "status-update-notification-rules");

        let req = self.client.build_request(
            url,
            builder.method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateStatusUpdateNotificationRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete a user
    ///
    /// Remove an existing user.
    ///
    /// Returns 400 if the user has assigned incidents unless your [pricing plan](https://support.pagerduty.com/docs/offboarding) has the `offboarding` feature and the account is [configured](https://support.pagerduty.com/docs/offboarding#section-additional-configurations) appropriately.
    ///
    /// Note that the incidents reassignment process is asynchronous and has no guarantee to complete before the api call return.
    ///
    /// [*Learn more about `offboarding` feature*](https://support.pagerduty.com/docs/offboarding).
    ///
    ///
    /// ---
    pub async fn delete_user(&self, id: &str) -> Result<(), Error> {
        let url = Praiya::parse_url(&self.api_endpoint, &format!("/users/{}", &id), None)?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Delete a user&#x27;s contact method
    ///
    /// Remove a user's contact method.
    ///
    ///
    /// ---
    pub async fn delete_user_contact_method(
        &self,
        id: &str,
        contact_method_id: &str,
    ) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/contact_methods/{}", &id, &contact_method_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Delete a User&#x27;s Handoff Notification rule
    ///
    /// Remove a User's Handoff Notification Rule.
    ///
    ///
    /// ---
    pub async fn delete_user_handoff_notification_rule(
        &self,
        id: &str,
        oncall_handoff_notification_rule_id: &str,
    ) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/oncall_handoff_notification_rules/{}",
                &id, &oncall_handoff_notification_rule_id
            ),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Delete a user&#x27;s notification rule
    ///
    /// Remove a user's notification rule.
    ///
    ///
    /// ---
    pub async fn delete_user_notification_rule(
        &self,
        id: &str,
        notification_rule_id: &str,
    ) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/notification_rules/{}",
                &id, &notification_rule_id
            ),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Delete a user&#x27;s session
    ///
    /// Delete a user's session.
    ///
    ///
    /// ---
    pub async fn delete_user_session(
        &self,
        id: &str,
        _type: &str,
        session_id: &str,
    ) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/sessions/{}/{}", &id, &_type, &session_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Delete all user sessions
    ///
    /// Delete all user sessions.
    ///
    ///
    /// ---
    pub async fn delete_user_sessions(&self, id: &str) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/sessions", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Delete a user&#x27;s status update notification rule
    ///
    /// Remove a user's status update notification rule.
    ///
    ///
    /// ---
    pub async fn delete_user_status_update_notification_rule(
        &self,
        id: &str,
        status_update_notification_rule_id: &str,
    ) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/status_update_notification_rules/{}",
                &id, &status_update_notification_rule_id
            ),
            None,
        )?;

        let mut builder = http::request::Builder::new();
        let early_access: &str = PraiyaCustomHeaders::EarlyAccess(None).into();
        builder = builder.header(early_access, "status-update-notification-rules");

        let req =
            self.client
                .build_request(url, builder.method(Method::DELETE), hyper::Body::empty());

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Get the current user
    ///
    /// Get details about the current user.
    ///
    /// This endpoint can only be used with a [user-level API key](https://support.pagerduty.com/docs/using-the-api#section-generating-a-personal-rest-api-key) or a key generated through an OAuth flow. This will not work if the request is made with an account-level access token.
    ///
    ///
    /// ---
    pub async fn get_current_user(
        &self,
        query_params: GetCurrentUserParams,
    ) -> Result<User, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, "/users/me", Some(&query_params.qs))?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetCurrentUserResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get a user
    ///
    /// Get details about an existing user.
    ///
    ///
    /// ---
    pub async fn get_user(&self, id: &str, query_params: GetUserParams) -> Result<User, Error> {
        let uri = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}", &id),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetUserResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get a user&#x27;s contact method
    ///
    /// Get details about a User's contact method.
    ///
    ///
    /// ---
    pub async fn get_user_contact_method(
        &self,
        id: &str,
        contact_method_id: &str,
    ) -> Result<GetUserContactMethodEnum, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/contact_methods/{}", &id, &contact_method_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetUserContactMethodResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get a user&#x27;s handoff notification rule
    ///
    /// Get details about a User's Handoff Notification Rule.
    ///
    ///
    /// ---
    pub async fn get_user_handoff_notification_rule(
        &self,
        id: &str,
        oncall_handoff_notification_rule_id: &str,
    ) -> Result<HandoffNotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/oncall_handoff_notification_rules/{}",
                &id, &oncall_handoff_notification_rule_id
            ),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetUserHandoffNotificationRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List a User&#x27;s Handoff Notification Rules
    ///
    /// List Handoff Notification Rules of your PagerDuty User.
    ///
    ///
    /// ---
    pub fn list_user_handoff_notification_rules(
        &self,
        id: &str,
    ) -> impl Stream<Item = Result<HandoffNotificationRule, Error>> + '_ {
        self.client
            .list_request::<_, _, ListUserHandoffNotificationRulesResponse>(
                &self.api_endpoint,
                &format!("/users/{}/oncall_handoff_notification_rules", &id),
                NoopParams {},
                PraiyaCustomHeaders::None,
            )
    }

    /// ---
    ///
    /// # List a user&#x27;s contact methods
    ///
    /// List contact methods of your PagerDuty user.
    ///
    ///
    /// ---
    pub fn list_user_contact_methods(
        &self,
        id: &str,
    ) -> impl Stream<Item = Result<GetUserContactMethodEnum, Error>> + '_ {
        self.client
            .list_request::<_, _, ListUserContactMethodsResponse>(
                &self.api_endpoint,
                &format!("/users/{}/contact_methods", &id),
                NoopParams {},
                PraiyaCustomHeaders::None,
            )
    }

    /// ---
    ///
    /// # Get a user&#x27;s notification rule
    ///
    /// Get details about a user's notification rule.
    ///
    ///
    /// ---
    pub async fn get_user_notification_rule(
        &self,
        id: &str,
        notification_rule_id: &str,
        query_params: GetUserNotificationRuleParams,
    ) -> Result<NotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/notification_rules/{}",
                &id, &notification_rule_id
            ),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetUserNotificationRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List a user&#x27;s notification rules
    ///
    /// List notification rules of your PagerDuty user.
    ///
    ///
    /// ---
    pub fn list_user_notification_rules(
        &self,
        id: &str,
        query_params: GetUserNotificationRulesParams,
    ) -> impl Stream<Item = Result<NotificationRule, Error>> + '_ {
        self.client
            .list_request::<_, _, ListUserNotificationRulesResponse>(
                &self.api_endpoint,
                &format!("/users/{}/notification_rules", &id),
                query_params,
                PraiyaCustomHeaders::None,
            )
    }

    /// ---
    ///
    /// # Get a user&#x27;s status update notification rule
    ///
    /// Get details about a user's status update notification rule.
    ///
    ///
    /// ---
    pub async fn get_user_status_update_notification_rule(
        &self,
        id: &str,
        status_update_notification_rule_id: &str,
        query_params: GetUserStatusUpdateNotificationRuleParams,
    ) -> Result<StatusUpdateNotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/status_update_notification_rules/{}",
                &id, &status_update_notification_rule_id
            ),
            Some(&query_params.qs),
        )?;

        let mut builder = http::request::Builder::new();
        let early_access: &str = PraiyaCustomHeaders::EarlyAccess(None).into();
        builder = builder.header(early_access, "status-update-notification-rules");

        let req = self.client.build_request(
            url,
            builder.method(http::method::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetUserStatusUpdateNotificationRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get a user&#x27;s session
    ///
    /// Get details about a user's session.
    ///
    ///
    /// ---
    pub async fn get_user_session(
        &self,
        id: &str,
        _type: &str,
        session_id: &str,
    ) -> Result<UserSession, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/sessions/{}/{}", &id, &_type, &session_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetUserSessionResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List a user&#x27;s active sessions
    ///
    /// List active sessions of a PagerDuty user.
    ///
    ///
    /// ---
    pub async fn get_user_sessions(&self, id: &str) -> Result<Vec<UserSession>, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/sessions", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetUserSessionsResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List a user&#x27;s status update notification rules
    ///
    /// List status update notification rules of your PagerDuty user.
    ///
    ///
    /// ---
    pub fn list_user_status_update_notification_rules(
        &self,
        id: &str,
        query_params: GetUserStatusUpdateNotificationRulesParams,
    ) -> impl Stream<Item = Result<StatusUpdateNotificationRule, Error>> + '_ {
        self.client
            .list_request::<_, _, ListUserStatusUpdateNotificationRulesResponse>(
                &self.api_endpoint,
                &format!("/users/{}/status_update_notification_rules", &id),
                query_params,
                PraiyaCustomHeaders::EarlyAccess(Some("status-update-notification-rules")),
            )
    }

    /// ---
    ///
    /// # List users
    ///
    /// List users of your PagerDuty account, optionally filtered by a search query.
    ///
    ///
    /// ---
    pub fn list_users(
        &self,
        query_params: ListUsersParams,
    ) -> impl Stream<Item = Result<User, Error>> + '_ {
        self.client.list_request::<_, _, ListUsersResponse>(
            &self.api_endpoint,
            "/users",
            query_params,
            PraiyaCustomHeaders::None,
        )
    }

    /// ---
    ///
    /// # List audit records for a user
    ///
    /// The response will include audit records with changes that are made to the identified user not changes made by the identified user.
    ///
    ///
    /// The returned records are sorted by the `execution_time` from newest to oldest.
    ///
    ///
    /// ---
    pub fn list_users_audit_records(
        &self,
        id: &str,
        query_params: ListUsersAuditRecordsParams,
    ) -> impl Stream<Item = Result<AuditRecord, Error>> + '_ {
        let mut header_map = std::collections::HashMap::new();
        let audit_early_access: &str = PraiyaCustomHeaders::AuditEarlyAccess.into();
        header_map.insert(String::from(audit_early_access), String::from("true"));

        let base_request = BaseRequest {
            host: String::from(&self.api_endpoint),
            method: http::Method::GET,
            options: std::sync::Arc::new(query_params),
            path: format!("/users/{}/audit/records", &id),
            headers: header_map,
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
    /// # Remove notification subscription
    ///
    /// Unsubscribe the given User from Notifications on the matching subscribable entity.
    ///
    ///
    /// ---
    pub async fn unsubscribe_user_notification_subscription(
        &self,
        id: &str,
        body: UnsubscribeUserNotificationSubscriptions,
    ) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/notification_subscriptions/unsubscribe", &id),
            None,
        )?;

        let mut builder = http::request::Builder::new();
        let early_access: &str = PraiyaCustomHeaders::EarlyAccess(None).into();
        builder = builder.header(early_access, "true");

        let req = self.client.build_request(
            url,
            builder.method(Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Update a user
    ///
    /// Update an existing user. Note that you may also supply a `password` property--it will not be returned by any API.
    ///
    ///
    /// ---
    pub async fn update_user(&self, id: &str, body: UpdateUser) -> Result<User, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, &format!("/users/{}", &id), None)?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateUserResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update a user&#x27;s contact method
    ///
    /// Update a User's contact method.
    ///
    ///
    /// ---
    pub async fn update_user_contact_method(
        &self,
        id: &str,
        contact_method_id: &str,
        body: UpdateUserContactMethod,
    ) -> Result<UpdateUserContactMethodContactMethodEnum, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/users/{}/contact_methods/{}", &id, &contact_method_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateUserContactMethodResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update a User&#x27;s Handoff Notification Rule
    ///
    /// Update a User's Handoff Notification Rule.
    ///
    ///
    /// ---
    pub async fn update_user_handoff_notification_rule(
        &self,
        id: &str,
        oncall_handoff_notification_rule_id: &str,
        body: UpdateUserHandoffNotification,
    ) -> Result<HandoffNotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/oncall_handoff_notification_rules/{}",
                &id, &oncall_handoff_notification_rule_id
            ),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateUserHandoffNotificationRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update a user&#x27;s notification rule
    ///
    /// Update a user's notification rule.
    ///
    ///
    /// ---
    pub async fn update_user_notification_rule(
        &self,
        id: &str,
        notification_rule_id: &str,
        body: UpdateUserNotificationRule,
    ) -> Result<NotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/notification_rules/{}",
                &id, &notification_rule_id
            ),
            None,
        )?;

        let req = self.client.build_request(
            url,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateUserNotificationRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update a user&#x27;s status update notification rule
    ///
    /// Update a user's status update notification rule.
    ///
    ///
    /// ---
    pub async fn update_user_status_update_notification_rule(
        &self,
        id: &str,
        status_update_notification_rule_id: &str,
        body: UpdateUserStatusUpdateNotificationRule,
    ) -> Result<StatusUpdateNotificationRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "/users/{}/status_update_notification_rules/{}",
                &id, &status_update_notification_rule_id
            ),
            None,
        )?;

        let mut builder = http::request::Builder::new();
        let early_access: &str = PraiyaCustomHeaders::EarlyAccess(None).into();
        builder = builder.header(early_access, "status-update-notification-rules");

        let req = self.client.build_request(
            url,
            builder.method(Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateUserStatusUpdateNotificationRuleResponse>(req)
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
    async fn test_create_user() {
        let pagerduty = crate::Praiya::new("test");
        let create_user = CreateUser {
            user: User {
                _type: UserTypeEnum::USER,
                name: Some(String::from("Earline Greenholt")),
                email: Some(String::from("125.greenholt.earline@graham.name")),
                time_zone: Some(String::from("America/Lima")),
                color: Some(String::from("green")),
                role: Some(UserRoleEnum::ADMIN),
                job_title: Some(String::from("Director of Engineering")),
                avatar_url: Some(String::from("https://secure.gravatar.com/avatar/1d1a39d4635208d5664082a6c654a73f.png?d=mm&r=PG")),
                description: Some(String::from("I'm the boss")),
                ..Default::default()
            },
        };
        let user = pagerduty
            .users()
            .create_user(create_user, "from@example.com")
            .await
            .unwrap();

        assert_eq!(user.id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_create_user_contact_method() {
        let pagerduty = crate::Praiya::new("test");
        let create_user_contact_method = CreateUserContactMethod {
            contact_method: CreateUserContactMethodContactMethodEnum::EMAIL_CONTACT_METHOD(
                EmailContactMethod {
                    _type: EmailContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
                    label: String::from("work"),
                    address: String::from("grady.haylie.126@hickle.net"),
                    send_short_email: Some(false),
                    ..Default::default()
                },
            ),
        };

        let contact_method = pagerduty
            .users()
            .create_user_contact_method("PXPGF42", create_user_contact_method)
            .await
            .unwrap();

        assert_eq!(contact_method.id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_create_user_handoff_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let create_user_handoff_notification_rule = CreateUserHandoffNotificationRule {
            oncall_handoff_notification_rule: HandoffNotificationRule {
                id: String::from("PXPGF43"),
                handoff_type: HandoffNotificationRuleHandoffTypeEnum::BOTH,
                notify_advance_in_minutes: Some(180),
                contact_method: ContactMethod {
                    id: Some(String::from("PXPGF42")),
                    _type: ContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
                    ..Default::default()
                },
            },
        };

        let handoff_notification_rule = pagerduty
            .users()
            .create_user_handoff_notification_rule("PXPGF42", create_user_handoff_notification_rule)
            .await
            .unwrap();

        assert_eq!(handoff_notification_rule.id, String::from("PXPGF42"));
    }

    #[tokio::test]
    async fn test_create_user_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let create_user_notification_rule = CreateUserNotificationRule {
            notification_rule: NotificationRule {
                _type: NotificationRuleTypeEnum::ASSIGNMENT_NOTIFICATION_RULE,
                start_delay_in_minutes: Some(0),
                contact_method: Some(ContactMethod {
                    id: Some(String::from("PXPGF42")),
                    _type: ContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
                    ..Default::default()
                }),
                urgency: Some(NotificationRuleUrgencyEnum::HIGH),
                ..Default::default()
            },
        };

        let notification_rule = pagerduty
            .users()
            .create_user_notification_rule("PXPGF42", create_user_notification_rule)
            .await
            .unwrap();

        assert_eq!(notification_rule.id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_create_user_notification_subscribers() {
        let pagerduty = crate::Praiya::new("test");
        let create_user_notification_subscriptions = CreateUserNotificationSubscriptions {
            subscribables: vec![
                NotificationSubscribable {
                    subscribable_type: Some(NotificationSubscribableSubscribableTypeEnum::INCIDENT),
                    subscribable_id: Some(String::from("PD1234")),
                },
                NotificationSubscribable {
                    subscribable_type: Some(
                        NotificationSubscribableSubscribableTypeEnum::BUSINESS_SERVICE,
                    ),
                    subscribable_id: Some(String::from("PD1234")),
                },
                NotificationSubscribable {
                    subscribable_type: Some(
                        NotificationSubscribableSubscribableTypeEnum::BUSINESS_SERVICE,
                    ),
                    subscribable_id: Some(String::from("PD1235")),
                },
            ],
        };

        let subscriptions = pagerduty
            .users()
            .create_user_notification_subscriptions(
                "PXPGF42",
                create_user_notification_subscriptions,
            )
            .await
            .unwrap();

        assert_eq!(subscriptions[0].account_id, Some(String::from("PD1234")));
    }

    #[tokio::test]
    async fn test_create_user_status_update_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let create_user_status_notification_rule = CreateUserStatusUpdateNotificationRule {
            status_update_notification_rule: StatusUpdateNotificationRule {
                contact_method: ContactMethod {
                    id: Some(String::from("PXPGF42")),
                    _type: ContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
                    ..Default::default()
                },
            },
        };

        let status_update_notification_rule = pagerduty
            .users()
            .create_user_status_update_notification_rule(
                "PXPGF42",
                create_user_status_notification_rule,
            )
            .await
            .unwrap();

        assert_eq!(
            status_update_notification_rule.contact_method.id,
            Some(String::from("PXPGF42"))
        );
    }

    #[tokio::test]
    async fn test_delete_user() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty.users().delete_user("PXPGF42").await.unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_delete_user_contact_method() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .users()
            .delete_user_contact_method("PXPGF42", "PXPGF42")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_delete_user_handoff_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .users()
            .delete_user_handoff_notification_rule("PXPGF42", "PXPGF42")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_delete_user_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .users()
            .delete_user_notification_rule("PXPGF42", "PXPGF42")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_delete_user_session() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .users()
            .delete_user_session("PXPGF42", "browser", "PXPGF42")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_delete_user_sessions() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .users()
            .delete_user_sessions("PXPGF42")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_delete_user_status_update_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .users()
            .delete_user_status_update_notification_rule("PXPGF42", "PXPGF42")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_get_current_user() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetCurrentUserParamsBuilder::new();
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let user = pagerduty.users().get_current_user(opts).await.unwrap();

        assert_eq!(user.id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_get_user() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetUserParamsBuilder::new();
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let user = pagerduty.users().get_user("PT4KHLK", opts).await.unwrap();

        assert_eq!(user.id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_get_user_contact_method() {
        let pagerduty = crate::Praiya::new("test");

        let contact_method = pagerduty
            .users()
            .get_user_contact_method("PXPGF42", "PT4KHLK")
            .await
            .unwrap();

        match contact_method {
            super::GetUserContactMethodEnum::EMAIL_CONTACT_METHOD(contact_method) => {
                assert_eq!(contact_method.id, Some(String::from("PXPGF42")));
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[tokio::test]
    async fn test_get_user_handoff_notification_rule() {
        let pagerduty = crate::Praiya::new("test");

        let handoff_notification_rule = pagerduty
            .users()
            .get_user_handoff_notification_rule("PXPGF42", "PT4KHLK")
            .await
            .unwrap();

        assert_eq!(handoff_notification_rule.id, String::from("PXPGF42"));
    }

    #[tokio::test]
    async fn test_list_user_handoff_notification_rule() {
        let pagerduty = crate::Praiya::new("test");

        let handoff_notification_rule: Option<HandoffNotificationRule> = pagerduty
            .users()
            .list_user_handoff_notification_rules("PXPGF42")
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            handoff_notification_rule.unwrap().id,
            String::from("PXPGF42")
        );
    }

    #[tokio::test]
    async fn test_list_user_contact_methods() {
        let pagerduty = crate::Praiya::new("test");

        let contact_methods: Option<super::GetUserContactMethodEnum> = pagerduty
            .users()
            .list_user_contact_methods("PXPGF42")
            .try_next()
            .await
            .unwrap();

        match contact_methods.unwrap() {
            super::GetUserContactMethodEnum::EMAIL_CONTACT_METHOD(contact_method) => {
                assert_eq!(contact_method.id, Some(String::from("PXPGF42")));
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[tokio::test]
    async fn test_get_user_notification_rule() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetUserNotificationRuleParamsBuilder::new();
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let notification_rule = pagerduty
            .users()
            .get_user_notification_rule("PXPGF42", "PT4KHLK", opts)
            .await
            .unwrap();

        assert_eq!(notification_rule.id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_get_user_status_update_notification_rule() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetUserStatusUpdateNotificationRuleParamsBuilder::new();
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let status_update_notification_rule = pagerduty
            .users()
            .get_user_status_update_notification_rule("PXPGF42", "PTDVERC", opts)
            .await
            .unwrap();

        assert_eq!(
            status_update_notification_rule.contact_method.id,
            Some(String::from("PTDVERC"))
        );
    }

    #[tokio::test]
    async fn test_get_user_session() {
        let pagerduty = crate::Praiya::new("test");

        let user_session = pagerduty
            .users()
            .get_user_session("PXPGF42", "browser", "PTDVERC")
            .await
            .unwrap();

        assert_eq!(user_session.id, String::from("PXPGF42"));
    }

    #[tokio::test]
    async fn test_get_user_sessions() {
        let pagerduty = crate::Praiya::new("test");

        let user_sessions = pagerduty
            .users()
            .get_user_sessions("PXPGF42")
            .await
            .unwrap();

        assert_eq!(user_sessions[0].id, String::from("PXPGF42"));
    }

    #[tokio::test]
    async fn test_list_user_status_update_notification_rules() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetUserStatusUpdateNotificationRulesParamsBuilder::new();
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let status_update_notification_rule: Option<StatusUpdateNotificationRule> = pagerduty
            .users()
            .list_user_status_update_notification_rules("PXPGF42", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            status_update_notification_rule.unwrap().contact_method.id,
            Some(String::from("PXPGF42"))
        );
    }

    #[tokio::test]
    async fn test_list_users() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListUsersParamsBuilder::new();
        opts_builder.include(vec![]);
        opts_builder.team_ids(vec![]);
        opts_builder.query("active");
        let opts = opts_builder.build();

        let user: Option<User> = pagerduty.users().list_users(opts).try_next().await.unwrap();

        assert_eq!(user.unwrap().id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_list_users_audit_records() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListUsersAuditRecordsParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.since(&since);
        opts_builder.until(&now);
        let opts = opts_builder.build();

        let record: Option<AuditRecord> = pagerduty
            .users()
            .list_users_audit_records("PIJ90N7", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            record.unwrap().id,
            String::from("PD_ADD_HIGH_URGENCY_NOTIFICATION")
        );
    }

    #[tokio::test]
    async fn test_unsubscribe_user_notification_subscribers() {
        let pagerduty = crate::Praiya::new("test");

        let unsubscribe_user_notification_subscriptions =
            UnsubscribeUserNotificationSubscriptions {
                subscribables: vec![
                    NotificationSubscribable {
                        subscribable_type: Some(
                            NotificationSubscribableSubscribableTypeEnum::INCIDENT,
                        ),
                        subscribable_id: Some(String::from("PD1234")),
                    },
                    NotificationSubscribable {
                        subscribable_type: Some(
                            NotificationSubscribableSubscribableTypeEnum::BUSINESS_SERVICE,
                        ),
                        subscribable_id: Some(String::from("PD1234")),
                    },
                ],
            };
        let unit = pagerduty
            .users()
            .unsubscribe_user_notification_subscription(
                "PXPGF42",
                unsubscribe_user_notification_subscriptions,
            )
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_update_user() {
        let pagerduty = crate::Praiya::new("test");
        let update_user = UpdateUser {
            user: User {
                _type: UserTypeEnum::USER,
                name: Some(String::from("Earline Greenholt")),
                email: Some(String::from("125.greenholt.earline@graham.name")),
                time_zone: Some(String::from("America/Lima")),
                color: Some(String::from("green")),
                role: Some(UserRoleEnum::ADMIN),
                job_title: Some(String::from("Director of Engineering")),
                avatar_url: Some(String::from("https://secure.gravatar.com/avatar/1d1a39d4635208d5664082a6c654a73f.png?d=mm&r=PG")),
                description: Some(String::from("I'm the boss")),
                ..Default::default()
            },
        };
        let user = pagerduty
            .users()
            .update_user("PXPGF42", update_user)
            .await
            .unwrap();

        assert_eq!(user.id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_update_user_contact_method() {
        let pagerduty = crate::Praiya::new("test");
        let update_user_contact_method = UpdateUserContactMethod {
            contact_method: UpdateUserContactMethodContactMethodEnum::EMAIL_CONTACT_METHOD(
                EmailContactMethod {
                    _type: EmailContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
                    label: String::from("work"),
                    address: String::from("grady.haylie.126@hickle.net"),
                    send_short_email: Some(false),
                    ..Default::default()
                },
            ),
        };

        let contact_method = pagerduty
            .users()
            .update_user_contact_method("PXPGF42", "PTDVERC", update_user_contact_method)
            .await
            .unwrap();

        match contact_method {
            super::UpdateUserContactMethodContactMethodEnum::EMAIL_CONTACT_METHOD(
                contact_method,
            ) => {
                assert_eq!(contact_method.id, Some(String::from("PXPGF42")));
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[tokio::test]
    async fn test_update_user_handoff_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let update_user_handoff_notification = UpdateUserHandoffNotification {
            oncall_handoff_notification_rule: HandoffNotificationRule {
                id: String::from("PXPGF43"),
                handoff_type: HandoffNotificationRuleHandoffTypeEnum::BOTH,
                notify_advance_in_minutes: Some(60),
                contact_method: ContactMethod {
                    id: Some(String::from("PXPGF42")),
                    _type: ContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
                    ..Default::default()
                },
            },
        };

        let handoff_notification_rule = pagerduty
            .users()
            .update_user_handoff_notification_rule(
                "PXPGF42",
                "PTDVERC",
                update_user_handoff_notification,
            )
            .await
            .unwrap();

        assert_eq!(handoff_notification_rule.id, String::from("PXPGF42"));
    }

    #[tokio::test]
    async fn test_update_user_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let update_user_notification_rule = UpdateUserNotificationRule {
            notification_rule: NotificationRule {
                _type: NotificationRuleTypeEnum::ASSIGNMENT_NOTIFICATION_RULE,
                start_delay_in_minutes: Some(0),
                contact_method: Some(ContactMethod {
                    id: Some(String::from("PXPGF42")),
                    _type: ContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
                    ..Default::default()
                }),
                urgency: Some(NotificationRuleUrgencyEnum::HIGH),
                ..Default::default()
            },
        };

        let notification_rule = pagerduty
            .users()
            .update_user_notification_rule("PXPGF42", "PXPGF42", update_user_notification_rule)
            .await
            .unwrap();

        assert_eq!(notification_rule.id, Some(String::from("PXPGF42")));
    }

    #[tokio::test]
    async fn test_update_user_status_update_notification_rule() {
        let pagerduty = crate::Praiya::new("test");
        let update_user_status_notification_rule = UpdateUserStatusUpdateNotificationRule {
            status_update_notification_rule: StatusUpdateNotificationRule {
                contact_method: ContactMethod {
                    id: Some(String::from("PXPGF42")),
                    _type: ContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
                    ..Default::default()
                },
            },
        };

        let status_update_notification_rule = pagerduty
            .users()
            .update_user_status_update_notification_rule(
                "PXPGF42",
                "PXPGF42",
                update_user_status_notification_rule,
            )
            .await
            .unwrap();

        assert_eq!(
            status_update_notification_rule.contact_method.id,
            Some(String::from("PXPGF42"))
        );
    }
}
