//! Method, error and parameter types for the Incidents endpoint.

use futures_core::Stream;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::Error;
use crate::praiya::{NoopParams, PraiyaCustomHeaders};
use crate::slack_models::*;
use crate::{
    BaseOption, BaseRequest, PaginatedResponse, PaginationQueryComponent, Praiya, SingleResponse,
    SubSystem, DEFAULT_PAGERDUTY_API_LIMIT,
};

pub const API_ENDPOINT: &str = "https://app.pagerduty.com/integration-slack/";

/// A client for the PagerDuty slack connections API
pub struct SlackConnectionsClient {
    pub(crate) api_endpoint: String,
    pub(crate) client: Praiya,
    pub(crate) slack_workspace_id: String,
}

impl Praiya {
    pub fn slack_connections(&self, slack_workspace_id: &str) -> SlackConnectionsClient {
        SlackConnectionsClient {
            api_endpoint: std::env::var("PAGERDUTY_API_ENDPOINT")
                .unwrap_or_else(|_| String::from(API_ENDPOINT)),
            client: Praiya::clone(self),
            slack_workspace_id: String::from(slack_workspace_id),
        }
    }
}

single_response_type!(SlackConnection, slack_connection, CreateConnection);

single_response_type!(SlackConnection, slack_connection, DeleteConnection);

single_response_type!(SlackConnection, slack_connection, GetConnection);

list_response_type!(ListConnection, slack_connections, SlackConnection);

single_response_type!(SlackConnection, slack_connection, UpdateConnection);

impl SlackConnectionsClient {
    /// ---
    ///
    /// # Create a Slack Connection
    ///
    /// Creates a Slack Connection
    ///
    ///
    /// ---
    pub async fn create_connection(
        &self,
        body: CreateConnection,
    ) -> Result<SlackConnection, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!("./workspaces/{}/connections", &self.slack_workspace_id),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, CreateConnectionResponse>(req)
            .await
    }

    /// ---
    ///
    /// # Delete a Slack Connection
    ///
    /// Delete an existing Slack Connection.
    ///
    /// ---
    pub async fn delete_connection(&self, connection_id: &str) -> Result<(), Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "./workspaces/{}/connections/{}",
                &self.slack_workspace_id, &connection_id
            ),
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
    /// # Get a Slack Connection
    ///
    /// Get details about an existing Slack Connection.
    ///
    /// ---
    pub async fn get_connection(&self, connection_id: &str) -> Result<SlackConnection, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "./workspaces/{}/connections/{}",
                &self.slack_workspace_id, &connection_id
            ),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::GET),
            hyper::Body::empty(),
        );

        self.client
            .process_into_value::<_, GetConnectionResponse>(req)
            .await
    }

    /// ---
    ///
    /// # List Slack Connections
    ///
    /// Returns a list of Slack Connections.
    ///
    /// ---
    pub fn get_connections(&self) -> impl Stream<Item = Result<SlackConnection, Error>> + '_ {
        self.client.list_request::<_, _, ListConnectionResponse>(
            &self.api_endpoint,
            &format!("./workspaces/{}/connections", &self.slack_workspace_id),
            NoopParams {},
            PraiyaCustomHeaders::None,
        )
    }

    /// ---
    ///
    /// # Update a Slack Connection
    ///
    /// Update an existing Slack Connection.
    ///
    /// ---
    pub async fn update_connection(
        &self,
        connection_id: &str,
        body: UpdateConnection,
    ) -> Result<SlackConnection, Error> {
        let url = Praiya::parse_url(
            &self.api_endpoint,
            &format!(
                "./workspaces/{}/connections/{}",
                &self.slack_workspace_id, &connection_id
            ),
            None,
        )?;

        let req = self.client.build_request(
            url,
            http::request::Builder::new().method(http::method::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.client
            .process_into_value::<_, UpdateConnectionResponse>(req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::praiya::ParamsBuilder;
    use crate::slack_models::*;
    use futures_util::TryStreamExt;

    #[tokio::test]
    async fn test_create_slack_connection() {
        let pagerduty = crate::Praiya::new("test");
        let slack_team_id = "slack_workspace_id";
        let create_connection = CreateConnection {
            slack_connection: SlackConnection {
                source_id: Some(String::from("A1234B5")),
                source_type: Some(SlackConnectionSourceType::TEAM_REFERENCE),
                channel_id: Some(String::from("A123B456C7D")),
                notification_type: Some(SlackConnectionNotifiationType::RESPONDER),
                config: Some(SlackConnectionConfig {
                    events: Some(vec![]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        let connection = pagerduty
            .slack_connections(slack_team_id)
            .create_connection(create_connection)
            .await
            .unwrap();

        assert_eq!(connection.id, Some(String::from("A12BCDE")));
    }

    #[tokio::test]
    async fn test_delete_slack_connection() {
        let pagerduty = crate::Praiya::new("test");
        let slack_team_id = "slack_workspace_id";
        let unit = pagerduty
            .slack_connections(slack_team_id)
            .delete_connection("A12BCDE")
            .await
            .unwrap();

        assert_eq!(unit, ());
    }

    #[tokio::test]
    async fn test_get_slack_connection() {
        let pagerduty = crate::Praiya::new("test");
        let slack_team_id = "slack_workspace_id";
        let connection = pagerduty
            .slack_connections(slack_team_id)
            .get_connection("A12BCDE")
            .await
            .unwrap();

        assert_eq!(connection.id, Some(String::from("A12BCDE")));
    }

    #[tokio::test]
    async fn test_get_slack_connections() {
        let pagerduty = crate::Praiya::new("test");
        let slack_team_id = "slack_workspace_id";

        let connection: Option<SlackConnection> = pagerduty
            .slack_connections(slack_team_id)
            .get_connections()
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            connection.unwrap().id.as_ref().unwrap(),
            &String::from("A12BCDE")
        );
    }

    #[tokio::test]
    async fn test_update_slack_connection() {
        let pagerduty = crate::Praiya::new("test");
        let slack_team_id = "slack_workspace_id";

        let update_connection = UpdateConnection {
            slack_connection: SlackConnection {
                source_id: Some(String::from("A1234B5")),
                source_type: Some(SlackConnectionSourceType::TEAM_REFERENCE),
                channel_id: Some(String::from("A123B456C7D")),
                notification_type: Some(SlackConnectionNotifiationType::RESPONDER),
                config: Some(SlackConnectionConfig {
                    events: Some(vec![]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };

        let connection = pagerduty
            .slack_connections(slack_team_id)
            .update_connection("A12BCDE", update_connection)
            .await
            .unwrap();

        assert_eq!(connection.id, Some(String::from("A12BCDE")));
    }
}
