//! Method, error and parameter types for the Escalation Policies endpoint.

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

/// A client for the PagerDuty incidents API
pub struct EscalationPoliciesClient {
    pub(crate) api_endpoint: String,
    pub(crate) client: Praiya,
    pub(crate) from_email: Option<String>,
}

impl Praiya {
    pub fn escalation_policies(&self, from_email: Option<String>) -> EscalationPoliciesClient {
        EscalationPoliciesClient {
            api_endpoint: std::env::var("PAGERDUTY_API_ENDPOINT")
                .unwrap_or_else(|_| String::from(API_ENDPOINT)),
            client: Praiya::clone(self),
            from_email,
        }
    }
}

single_response_type!(EscalationPolicy, escalation_policy, CreateEscalationPolicy);

single_response_type!(EscalationPolicy, escalation_policy, GetEscalationPolicy);

list_response_type!(ListEscalationPolicy, escalation_policies, EscalationPolicy);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[EscalationPoliciesClient::get_escalation_policy]"]
#[allow(dead_code)]
struct GetEscalationPolicy {
    include: Vec<String>,
}

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[EscalationPoliciesClient::list_escalation_policies"]
#[allow(dead_code)]
struct ListEscalationPolicies {
    query: String,
    include: Vec<String>,
    sort_by: String,
    team_ids: Vec<String>,
    user_ids: Vec<String>,
}

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[EscalationPoliciesClient::list_escalation_policy_audit_records]"]
#[allow(dead_code)]
struct ListEscalationPolicyAuditRecords {
    since: chrono::DateTime<chrono::Utc>,
    until: chrono::DateTime<chrono::Utc>,
}

single_response_type!(EscalationPolicy, escalation_policy, UpdateEscalationPolicy);

impl EscalationPoliciesClient {
    /// ---
    ///
    /// # Create an escalation policy
    ///
    /// Creates a new escalation policy. At least one escalation rule must be provided.
    ///
    /// ---
    pub async fn create_escalation_policy(
        &self,
        body: CreateEscalationPolicy,
    ) -> Result<EscalationPolicy, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, "/escalation_policies", None)?;

        let mut builder = http::request::Builder::new();
        if let Some(from) = &self.from_email {
            builder = builder.header(FROM, String::clone(from));
        }
        let req = self.client.build_request(
            url,
            builder.method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateEscalationPolicyResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete an escalation policy
    ///
    /// Deletes an existing escalation policy and rules. The escalation policy must not be in use by any services.
    ///
    /// ---
    pub async fn delete_escalation_policy(&self, id: &str) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/escalation_policies/{}", &id),
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
    /// # Get an escalation policy
    ///
    /// Get information about an existing escalation policy and its rules.
    ///
    /// ---
    pub async fn get_escalation_policy(
        &self,
        id: &str,
        query_params: GetEscalationPolicyParams,
    ) -> Result<EscalationPolicy, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/escalation_policies/{}", &id),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetEscalationPolicyResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List escalation policies
    ///
    /// List all of the existing escalation policies.
    ///
    /// ---
    pub fn list_escalation_policies(
        &self,
        query_params: ListEscalationPoliciesParams,
    ) -> impl Stream<Item = Result<EscalationPolicy, Error>> + '_ {
        self.client
            .list_request::<_, _, ListEscalationPolicyResponse>(
                &self.api_endpoint,
                "/escalation_policies",
                query_params,
                PraiyaCustomHeaders::None,
            )
    }

    /// ---
    ///
    /// # List audit records for an escalation policy
    ///
    /// The returned records are sorted by the `execution_time` from newest to oldest.
    ///
    /// ---
    pub fn list_escalation_policy_audit_records(
        &self,
        id: &str,
        query_params: ListEscalationPolicyAuditRecordsParams,
    ) -> impl Stream<Item = Result<AuditRecord, Error>> + '_ {
        let header_map = std::collections::HashMap::new();
        // let audit_early_access: &str = PraiyaCustomHeaders::AuditEarlyAccess.into();
        // header_map.insert(String::from(audit_early_access), String::from("true"));

        let base_request = BaseRequest {
            host: String::from(&self.api_endpoint),
            method: http::Method::GET,
            options: std::sync::Arc::new(query_params),
            path: format!("/escalation_policies/{}/audit/records", &id),
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
    /// # Update an escalation policy
    ///
    /// Updates an existing escalation policy and rules.
    ///
    /// ---
    pub async fn update_escalation_policy(
        &self,
        id: &str,
        body: UpdateEscalationPolicy,
    ) -> Result<EscalationPolicy, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/escalation_policies/{}", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateEscalationPolicyResponse>(req)
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
    async fn test_create_escalation_policy() {
        let pagerduty = crate::Praiya::new("test");
        let create_escalation_policy = CreateEscalationPolicy {
            escalation_policy: EscalationPolicy {
                name: Some(String::from("Engineering Escalation Policy")),
                escalation_rules: Some(vec![EscalationRule {
                    escalation_delay_in_minutes: 30,
                    targets: vec![EscalationTargetReference {
                        id: Some(String::from("PEYSGVF")),
                        _type: EscalationTargetReferenceTypeEnum::USER_REFERENCE,
                        ..Default::default()
                    }],
                    ..Default::default()
                }]),
                services: Some(vec![Service {
                    id: Some(String::from("PIJ90N7")),
                    _type: ServiceTypeEnum::SERVICE_REFERENCE,
                    ..Default::default()
                }]),
                num_loops: Some(2),
                on_call_handoff_notifications: Some(
                    EscalationPolicyOnCallHandoffNotificationsEnum::IF_HAS_SERVICES,
                ),
                teams: Some(vec![Team {
                    id: Some(String::from("PQ9K7I8")),
                    _type: TeamTypeEnum::TEAM_REFERENCE,
                    ..Default::default()
                }]),
                description: Some(String::from("Here is the ep for the engineering team.")),
                ..Default::default()
            },
        };
        let escalation_policy = pagerduty
            .escalation_policies(Some(String::from("from@example.com")))
            .create_escalation_policy(create_escalation_policy)
            .await
            .unwrap();

        assert_eq!(escalation_policy.id, Some(String::from("PT20YPA")));
    }

    #[tokio::test]
    async fn test_delete_escalation_policy() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .escalation_policies(Some(String::from("from@example.com")))
            .delete_escalation_policy("PIJ90N7")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_get_escalation_policy() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetEscalationPolicyParamsBuilder::new();
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let escalation_policy = pagerduty
            .escalation_policies(Some(String::from("from@example.com")))
            .get_escalation_policy("PIJ90N7", opts)
            .await
            .unwrap();

        assert_eq!(escalation_policy.id, Some(String::from("PT20YPA")));
    }

    #[tokio::test]
    async fn test_list_escalation_policies() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListEscalationPoliciesParamsBuilder::new();
        opts_builder.include(vec![]);
        opts_builder.query("eng");
        opts_builder.sort_by("name:asc");
        opts_builder.team_ids(vec![]);
        opts_builder.user_ids(vec![]);
        let opts = opts_builder.build();

        let escalation_policy: Option<EscalationPolicy> = pagerduty
            .escalation_policies(Some(String::from("from@example.com")))
            .list_escalation_policies(opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            escalation_policy.unwrap().id.as_ref().unwrap(),
            &String::from("PANZZEQ")
        );
    }

    #[tokio::test]
    async fn test_list_escalation_policy_audit_records() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListEscalationPolicyAuditRecordsParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.since(&since);
        opts_builder.until(&now);
        let opts = opts_builder.build();

        let record: Option<AuditRecord> = pagerduty
            .escalation_policies(Some(String::from("from@example.com")))
            .list_escalation_policy_audit_records("PIJ90N7", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            record.unwrap().id,
            String::from("PD_ASSIGN_TEAM_TO_ESCALATION_POLICY")
        );
    }

    #[tokio::test]
    async fn test_update_escalation_policy() {
        let pagerduty = crate::Praiya::new("test");

        let update_escalation_policy = UpdateEscalationPolicy {
            escalation_policy: EscalationPolicy {
                name: Some(String::from("Engineering Escalation Policy")),
                escalation_rules: Some(vec![EscalationRule {
                    escalation_delay_in_minutes: 30,
                    targets: vec![EscalationTargetReference {
                        id: Some(String::from("PEYSGVF")),
                        _type: EscalationTargetReferenceTypeEnum::USER_REFERENCE,
                        ..Default::default()
                    }],
                    ..Default::default()
                }]),
                services: Some(vec![Service {
                    id: Some(String::from("PIJ90N7")),
                    _type: ServiceTypeEnum::SERVICE_REFERENCE,
                    ..Default::default()
                }]),
                num_loops: Some(2),
                on_call_handoff_notifications: Some(
                    EscalationPolicyOnCallHandoffNotificationsEnum::IF_HAS_SERVICES,
                ),
                teams: Some(vec![Team {
                    id: Some(String::from("PQ9K7I8")),
                    _type: TeamTypeEnum::TEAM_REFERENCE,
                    ..Default::default()
                }]),
                description: Some(String::from("Here is the ep for the engineering team.")),
                ..Default::default()
            },
        };

        let service = pagerduty
            .escalation_policies(Some(String::from("from@example.com")))
            .update_escalation_policy("PIJ90N7", update_escalation_policy)
            .await
            .unwrap();

        assert_eq!(service.id, Some(String::from("PT20YPA")));
    }
}
