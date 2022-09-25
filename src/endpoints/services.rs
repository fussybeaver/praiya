//! Method, error and parameter types for the Incidents endpoint.

use futures_core::Stream;
use futures_util::StreamExt;
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

/// A client for the PagerDuty slack connections API
pub struct ServicesClient {
    pub(crate) api_endpoint: String,
    pub(crate) client: Praiya,
}

impl Praiya {
    pub fn services(&self) -> ServicesClient {
        ServicesClient {
            api_endpoint: std::env::var("PAGERDUTY_API_ENDPOINT")
                .unwrap_or_else(|_| String::from(API_ENDPOINT)),
            client: Praiya::clone(self),
        }
    }
}

single_response_type!(Service, service, CreateService);

single_response_type!(ServiceEventRule, ruleset, CreateServiceEventRule);

single_response_type!(Integration, integration, CreateServiceIntegration);

single_response_type!(Service, service, GetService);

single_response_type!(ServiceEventRule, rule, GetServiceEventRule);

single_response_type!(Integration, integration, GetServiceIntegration);

list_response_type!(ListServiceIntegration, integrations, Integration);

list_response_type!(ListServiceEventRules, rules, ServiceEventRule);

list_response_type!(ListService, services, Service);

single_response_type!(Service, service, UpdateService);

single_response_type!(Integration, integration, UpdateServiceIntegration);

single_response_type!(ServiceEventRule, rule, UpdateServiceEventRule);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[ServicesClient::get_service]"]
#[allow(dead_code)]
struct GetService {
    include: Vec<String>,
}

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[ServicesClient::get_service_integration]"]
#[allow(dead_code)]
struct GetServiceIntegration {
    include: Vec<String>,
}

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[ServicesClient::list_service_audit_records]"]
#[allow(dead_code)]
struct ListServiceAuditRecords {
    since: chrono::DateTime<chrono::Utc>,
    until: chrono::DateTime<chrono::Utc>,
}

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[ServicesClient::list_services]"]
#[allow(dead_code)]
struct ListServices {
    statuses: Vec<String>,
    time_zone: chrono_tz::Tz,
    team_ids: Vec<String>,
    include: Vec<String>,
}

impl ServicesClient {
    /// ---
    ///
    /// # Create a service
    ///
    /// Create a new service.
    ///
    ///
    /// ---
    pub async fn create_service(&self, body: CreateService) -> Result<Service, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, "/services", None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateServiceResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Create an Event Rule on a Service
    ///
    /// Create a new Event Rule on a Service.
    ///
    ///
    /// ---
    pub async fn create_service_event_rule(
        &self,
        id: &str,
        body: CreateServiceEventRule,
    ) -> Result<ServiceEventRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/services/{}/rules", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateServiceEventRuleResponse>(req)
            .await
    }
    /// ---
    ///
    /// # Create a new integration
    ///
    /// Create a new integration belonging to a Service.
    ///
    ///
    /// ---
    pub async fn create_service_integration(
        &self,
        id: &str,
        body: CreateServiceIntegration,
    ) -> Result<Integration, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/services/{}/integrations", &id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateServiceIntegrationResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete a service
    ///
    /// Delete an existing service.
    ///
    /// Once the service is deleted, it will not be accessible from the web UI and new incidents won't be able to be created for this service.
    ///
    ///
    /// ---
    pub async fn delete_service(&self, id: &str) -> Result<(), Error> {
        let url = Praiya::parse_url(&self.api_endpoint, &format!("/services/{}", &id), None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Delete an Event Rule from a Service
    ///
    /// Delete an Event Rule from a Service.

    ///
    /// ---
    pub async fn delete_service_event_rule(&self, id: &str, rule_id: &str) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/services/{}/rules/{}", &id, &rule_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::Method::DELETE),
            hyper::Body::empty(),
        );

        self.client.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Get a service
    ///
    /// Get details about an existing service.
    ///
    ///
    /// ---
    pub async fn get_service(
        &self,
        id: &str,
        query_params: GetServiceParams,
    ) -> Result<Service, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/services/{}", &id),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetServiceResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Get an Event Rule from a Service
    ///
    /// Get an Event Rule from a Service.

    ///
    /// ---
    pub async fn get_service_event_rule(
        &self,
        id: &str,
        rule_id: &str,
    ) -> Result<ServiceEventRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/services/{}/rules/{}", &id, &rule_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetServiceEventRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # View an integration
    ///
    /// Get details about an integration belonging to a service.
    ///
    ///
    /// ---
    pub async fn get_service_integration(
        &self,
        id: &str,
        integration_id: &str,
        query_params: GetServiceIntegrationParams,
    ) -> Result<Integration, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/services/{}/integrations/{}", &id, &integration_id),
            Some(&query_params.qs),
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetServiceIntegrationResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List audit records for a service
    ///
    /// The returned records are sorted by the `execution_time` from newest to oldest.
    ///
    ///
    /// ---
    pub fn list_service_audit_records(
        &self,
        id: &str,
        query_params: ListServiceAuditRecordsParams,
    ) -> impl Stream<Item = Result<AuditRecord, Error>> + '_ {
        let mut header_map = std::collections::HashMap::new();
        let audit_early_access: &str = PraiyaCustomHeaders::AuditEarlyAccess.into();
        header_map.insert(String::from(audit_early_access), String::from("true"));

        let base_request = BaseRequest {
            host: String::from(&self.api_endpoint),
            method: http::Method::GET,
            options: std::sync::Arc::new(query_params),
            path: format!("/services/{}/audit/records", &id),
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
    /// # List Service&#x27;s Event Rules
    ///
    /// List Event Rules on a Service.

    ///
    /// ---
    pub fn list_service_event_rules(
        &self,
        id: &str,
    ) -> impl Stream<Item = Result<ServiceEventRule, Error>> + '_ {
        self.client
            .list_request::<_, _, ListServiceEventRulesResponse>(
                &self.api_endpoint,
                &format!("/services/{}/rules", &id),
                NoopParams {},
                PraiyaCustomHeaders::None,
            )
    }

    /// ---
    ///
    /// # List services
    ///
    /// List existing Services.
    ///
    ///
    /// ---
    pub fn list_services(
        &self,
        query_params: ListServicesParams,
    ) -> impl Stream<Item = Result<Service, Error>> + '_ {
        self.client.list_request::<_, _, ListServiceResponse>(
            &self.api_endpoint,
            "/services",
            query_params,
            PraiyaCustomHeaders::None,
        )
    }

    /// ---
    ///
    /// # Update a service
    ///
    /// Update an existing service.
    ///
    ///
    /// ---
    pub async fn update_service(&self, id: &str, body: UpdateService) -> Result<Service, Error> {
        let url = Praiya::parse_url(&self.api_endpoint, &format!("/services/{}", &id), None)?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateServiceResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update an Event Rule on a Service
    ///
    /// Update an Event Rule on a Service. Note that the endpoint supports partial updates, so any number of the writable fields can be provided.

    ///
    /// ---
    pub async fn update_service_event_rule(
        &self,
        id: &str,
        rule_id: &str,
        body: UpdateServiceEventRule,
    ) -> Result<ServiceEventRule, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/services/{}/rules/{}", &id, &rule_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateServiceEventRuleResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Update an existing integration
    ///
    /// Update an integration belonging to a Service.
    ///
    ///
    /// ---
    pub async fn update_service_integration(
        &self,
        id: &str,
        integration_id: &str,
        body: UpdateServiceIntegration,
    ) -> Result<Integration, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("/services/{}/integrations/{}", &id, &integration_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateServiceIntegrationResponse>(req)
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
    async fn test_create_service() {
        let pagerduty = crate::Praiya::new("test");
        let create_service = CreateService {
            service: Service {
                name: Some(String::from("My Web App")),
                description: Some(String::from("My cool web application that does things.")),
                auto_resolve_timeout: Some(14400),
                acknowledgement_timeout: Some(600),
                status: Some(ServiceStatusEnum::ACTIVE),
                escalation_policy: EscalationPolicyReference {
                    id: Some(String::from("PWIP6CQ")),
                    ..Default::default()
                },
                incident_urgency_rule: Some(IncidentUrgencyRule {
                    _type: IncidentUrgencyRuleTypeEnum::USE_SUPPORT_HOURS,
                    during_support_hours: Some(IncidentUrgencyType {
                        urgency: Some(IncidentUrgencyTypeUrgencyEnum::HIGH),
                        ..Default::default()
                    }),
                    outside_support_hours: Some(IncidentUrgencyType {
                        urgency: Some(IncidentUrgencyTypeUrgencyEnum::LOW),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                support_hours: Some(SupportHours {
                    time_zone: Some(String::from("America/Lima")),
                    start_time: Some(String::from("09:00:00")),
                    end_time: Some(String::from("17:00:00")),
                    days_of_week: Some(vec![1, 2, 3, 4, 5]),
                    ..Default::default()
                }),
                scheduled_actions: Some(vec![ScheduledAction {
                    at: ScheduledActionAt {
                        name: ScheduledActionAtNameEnum::START,
                        ..Default::default()
                    },
                    to_urgency: ScheduledActionToUrgencyEnum::HIGH,
                    ..Default::default()
                }]),
                alert_creation: Some(ServiceAlertCreationEnum::ALERTS_AND_INCIDENTS),
                alert_grouping: Some(ServiceAlertGroupingEnum::TIME),
                alert_grouping_timeout: Some(2),
                ..Default::default()
            },
        };
        let service = pagerduty
            .services()
            .create_service(create_service)
            .await
            .unwrap();

        assert_eq!(service.id, Some(String::from("PIJ90N7")));
    }

    #[tokio::test]
    async fn test_create_service_event_rule() {
        let pagerduty = crate::Praiya::new("test");
        let create_service_event_rule = CreateServiceEventRule {
            rule: ServiceEventRule {
                position: Some(0),
                disabled: Some(false),
                conditions: Some(EventRuleConditions {
                    operator: EventRuleConditionsOperatorEnum::AND,
                    subconditions: vec![EventRuleConditionsSubconditions {
                        operator: EventRuleConditionsSubconditionsOperatorEnum::CONTAINS,
                        parameters: EventRuleConditionsParameters {
                            value: String::from("mysql"),
                            path: String::from("class"),
                            ..Default::default()
                        },
                    }],
                }),
                time_frame: Some(EventRuleTimeFrame {
                    active_between: Some(EventRuleTimeFrameActiveBetween {
                        start_time: 1577880000000,
                        end_time: 1580558400000,
                    }),
                    ..Default::default()
                }),
                actions: Some(EventRuleActionsCommon {
                    annotate: Some(EventRuleActionsCommonAnnotate {
                        value: String::from("This incident was modified by an Event Rule"),
                    }),
                    priority: Some(EventRuleActionsCommonPriority {
                        value: String::from("PCMUB6F"),
                    }),
                    severity: Some(EventRuleActionsCommonSeverity {
                        value: EventRuleActionsCommonSeverityValueEnum::WARNING,
                    }),
                    extractions: Some(vec![
                        EventRuleActionsCommonExtractionsItems::ExtractionsItems0 {
                            target: String::from("dedup_key"),
                            source: String::from("custom_details.error_summary"),
                            regex: String::from("Host (.*) is experiencing errors"),
                        },
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };

        let service_event_rule = pagerduty
            .services()
            .create_service_event_rule("PIJ90N7", create_service_event_rule)
            .await
            .unwrap();

        assert_eq!(
            service_event_rule.id,
            Some(String::from("14e56445-ebab-4dd0-ba9d-fc28a41b7e7b"))
        );
    }

    #[tokio::test]
    async fn test_create_service_integration() {
        let pagerduty = crate::Praiya::new("test");
        let create_service_integration = CreateServiceIntegration {
            integration: Integration {
                _type: IntegrationTypeEnum::GENERIC_EMAIL_INBOUND_INTEGRATION,
                name: Some(String::from("email")),
                service: Some(ServiceReference {
                    id: Some(String::from("PQL78HM")),
                    ..Default::default()
                }),
                vendor: Some(VendorReference {
                    id: Some(String::from("PZD94QK")),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        let integration = pagerduty
            .services()
            .create_service_integration("PIJ90N7", create_service_integration)
            .await
            .unwrap();

        assert_eq!(integration.id, Some(String::from("PE1U9CH")));
    }

    #[tokio::test]
    async fn test_delete_service() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .services()
            .delete_service("PIJ90N7")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_delete_service_event_rule() {
        let pagerduty = crate::Praiya::new("test");
        let unit = pagerduty
            .services()
            .delete_service_event_rule("PIJ90N7", "14e56445-ebab-4dd0-ba9d-fc28a41b7e7b")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_get_service() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetServiceParamsBuilder::new();
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let service = pagerduty
            .services()
            .get_service("PIJ90N7", opts)
            .await
            .unwrap();

        assert_eq!(service.id, Some(String::from("PIJ90N7")));
    }

    #[tokio::test]
    async fn test_get_service_event_rule() {
        let pagerduty = crate::Praiya::new("test");

        let rule = pagerduty
            .services()
            .get_service_event_rule("PIJ90N7", "14e56445-ebab-4dd0-ba9d-fc28a41b7e7b")
            .await
            .unwrap();

        assert_eq!(
            rule.id,
            Some(String::from("14e56445-ebab-4dd0-ba9d-fc28a41b7e7b"))
        );
    }

    #[tokio::test]
    async fn test_get_service_integration() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::GetServiceIntegrationParamsBuilder::new();
        opts_builder.include(vec![]);
        let opts = opts_builder.build();

        let integration = pagerduty
            .services()
            .get_service_integration("PIJ90N7", "PE1U9CH", opts)
            .await
            .unwrap();

        assert_eq!(integration.id, Some(String::from("PE1U9CH")));
    }

    #[tokio::test]
    async fn test_list_service_audit_records() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListServiceAuditRecordsParamsBuilder::new();
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        opts_builder.since(&since);
        opts_builder.until(&now);
        let opts = opts_builder.build();

        let record: Option<AuditRecord> = pagerduty
            .services()
            .list_service_audit_records("PIJ90N7", opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            record.unwrap().id,
            String::from("PDRECORDID1_SERVICE_CREATED")
        );
    }

    #[tokio::test]
    async fn test_list_service_event_rules() {
        let pagerduty = crate::Praiya::new("test");

        let rule: Option<ServiceEventRule> = pagerduty
            .services()
            .list_service_event_rules("PIJ90N7")
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            rule.unwrap().id.unwrap(),
            String::from("14e56445-ebab-4dd0-ba9d-fc28a41b7e7b")
        );
    }

    #[tokio::test]
    async fn test_list_services() {
        let pagerduty = crate::Praiya::new("test");

        let mut opts_builder = super::ListServicesParamsBuilder::new();
        opts_builder.include(vec![]);
        opts_builder.team_ids(vec![]);
        opts_builder.statuses(vec!["triggered", "acknowledged"]);
        opts_builder.time_zone(&chrono_tz::EST);
        let opts = opts_builder.build();

        let service: Option<Service> = pagerduty
            .services()
            .list_services(opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            service.unwrap().id.as_ref().unwrap(),
            &String::from("PIJ90N7")
        );
    }

    #[tokio::test]
    async fn test_update_service() {
        let pagerduty = crate::Praiya::new("test");

        let update_service = UpdateService {
            service: Service {
                name: Some(String::from("My Web App")),
                description: Some(String::from("My cool web application that does things.")),
                auto_resolve_timeout: Some(14400),
                acknowledgement_timeout: Some(600),
                status: Some(ServiceStatusEnum::ACTIVE),
                escalation_policy: EscalationPolicyReference {
                    id: Some(String::from("PWIP6CQ")),
                    ..Default::default()
                },
                incident_urgency_rule: Some(IncidentUrgencyRule {
                    _type: IncidentUrgencyRuleTypeEnum::USE_SUPPORT_HOURS,
                    during_support_hours: Some(IncidentUrgencyType {
                        urgency: Some(IncidentUrgencyTypeUrgencyEnum::HIGH),
                        ..Default::default()
                    }),
                    outside_support_hours: Some(IncidentUrgencyType {
                        urgency: Some(IncidentUrgencyTypeUrgencyEnum::LOW),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                support_hours: Some(SupportHours {
                    time_zone: Some(String::from("America/Lima")),
                    start_time: Some(String::from("09:00:00")),
                    end_time: Some(String::from("17:00:00")),
                    days_of_week: Some(vec![1, 2, 3, 4, 5]),
                    ..Default::default()
                }),
                scheduled_actions: Some(vec![ScheduledAction {
                    at: ScheduledActionAt {
                        name: ScheduledActionAtNameEnum::START,
                        ..Default::default()
                    },
                    to_urgency: ScheduledActionToUrgencyEnum::HIGH,
                    ..Default::default()
                }]),
                alert_creation: Some(ServiceAlertCreationEnum::ALERTS_AND_INCIDENTS),
                alert_grouping: Some(ServiceAlertGroupingEnum::TIME),
                alert_grouping_timeout: Some(2),
                ..Default::default()
            },
        };

        let service = pagerduty
            .services()
            .update_service("PIJ90N7", update_service)
            .await
            .unwrap();

        assert_eq!(service.id, Some(String::from("PIJ90N7")));
    }

    #[tokio::test]
    async fn test_update_service_event_rule() {
        let pagerduty = crate::Praiya::new("test");

        let update_service_event_rule = UpdateServiceEventRule {
            rule_id: String::from("7123bdd1-74e8-4aa7-aa38-4a9ebe123456"),
            rule: Some(ServiceEventRule {
                position: Some(0),
                disabled: Some(false),
                conditions: Some(EventRuleConditions {
                    operator: EventRuleConditionsOperatorEnum::AND,
                    subconditions: vec![EventRuleConditionsSubconditions {
                        operator: EventRuleConditionsSubconditionsOperatorEnum::CONTAINS,
                        parameters: EventRuleConditionsParameters {
                            value: String::from("mysql"),
                            path: String::from("class"),
                            ..Default::default()
                        },
                    }],
                }),
                time_frame: Some(EventRuleTimeFrame {
                    active_between: Some(EventRuleTimeFrameActiveBetween {
                        start_time: 1577880000000,
                        end_time: 1580558400000,
                    }),
                    ..Default::default()
                }),
                actions: Some(EventRuleActionsCommon {
                    annotate: Some(EventRuleActionsCommonAnnotate {
                        value: String::from("This incident was modified by an Event Rule"),
                    }),
                    priority: Some(EventRuleActionsCommonPriority {
                        value: String::from("PCMUB6F"),
                    }),
                    severity: Some(EventRuleActionsCommonSeverity {
                        value: EventRuleActionsCommonSeverityValueEnum::WARNING,
                    }),
                    extractions: Some(vec![
                        EventRuleActionsCommonExtractionsItems::ExtractionsItems0 {
                            target: String::from("dedup_key"),
                            source: String::from("custom_details.error_summary"),
                            regex: String::from("Host (.*) is experiencing errors"),
                        },
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };

        let rule = pagerduty
            .services()
            .update_service_event_rule(
                "PIJ90N7",
                "14e56445-ebab-4dd0-ba9d-fc28a41b7e7b",
                update_service_event_rule,
            )
            .await
            .unwrap();

        assert_eq!(
            rule.id,
            Some(String::from("14e56445-ebab-4dd0-ba9d-fc28a41b7e7b"))
        );
    }

    #[tokio::test]
    async fn test_update_service_integration() {
        let pagerduty = crate::Praiya::new("test");

        let update_service_integration = UpdateServiceIntegration {
            integration: Integration {
                _type: IntegrationTypeEnum::GENERIC_EMAIL_INBOUND_INTEGRATION,
                name: Some(String::from("Email")),
                service: Some(ServiceReference {
                    id: Some(String::from("PQL78HM")),
                    summary: Some(String::from("My Email-Based Integration")),
                    ..Default::default()
                }),
                integration_email: Some(String::from(
                    "my-email-based-integration@subdomain.pagerduty.com",
                )),
                vendor: Some(VendorReference {
                    id: Some(String::from("PZD94QK")),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };

        let service = pagerduty
            .services()
            .update_service_integration("PIJ90N7", "PE1U9CH", update_service_integration)
            .await
            .unwrap();

        assert_eq!(service.id, Some(String::from("PE1U9CH")));
    }
}
