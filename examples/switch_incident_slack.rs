//! This example will fetch triggered and acknowledged incidents move their
//! Slack channel to another designated channel.

use std::collections::HashSet;

use futures_util::TryStreamExt;
use praiya::{
    api::{incidents::IncidentsListIncidentsParamsBuilder},
    default_models::{Incident, ServiceReference},
    ParamsBuilder, Praiya, slack_models::SlackConnection,
};

// Replace with appropriate values...
const PAGERDUTY_TOKEN: &str = "";
const WORKSPACE_TEAM_ID: &str = "";
const SLACK_CHANNEL_ID: &str = "";
const EMAIL: &str = "";

#[tokio::main]
async fn main() -> Result<(), praiya::errors::Error> {
    let pagerduty = Praiya::new(PAGERDUTY_TOKEN);

    let params = {
        let mut option = IncidentsListIncidentsParamsBuilder::new();
        let statuses = vec!["triggered", "acknowledged"];
        option.statuses(statuses);
        option.build()
    };

    let incidents: Vec<Incident> = pagerduty
        .incidents(EMAIL)
        .list_incidents(params)
        .try_collect()
        .await.expect("failed to fetch incidents");

    if incidents.is_empty() {
        return Ok(());
    }

    let mut services = HashSet::new();
    for incident in &incidents {
        if let Some(ServiceReference { id: Some(id), .. }) = &incident.service {
            services.insert(String::clone(id));
        }
    }

    let slack_connection_api = pagerduty
        .slack_connections(WORKSPACE_TEAM_ID);

    let mut stream = slack_connection_api.get_connections();

    while let Ok(Some(connection)) = stream.try_next().await 
    {
        match &connection {
            slack_connection @ SlackConnection {
                source_id: Some(source_id),
                id: Some(slack_connection_id),
                ..
            } if services.contains(source_id) => {

                println!(
                    "Redirecting service {} to channel {}",
                    source_id, SLACK_CHANNEL_ID
                );

                let mut slack_connection = SlackConnection::clone(slack_connection);

                slack_connection.channel_id = Some(String::from(SLACK_CHANNEL_ID));
                slack_connection.source_name = None;
                slack_connection.channel_name = None;

                let update_connection = praiya::slack_models::UpdateConnection { slack_connection };

                slack_connection_api
                    .update_connection(&slack_connection_id, update_connection)
                    .await.expect("failed to update slack connection");
            }
            _ => ()
        }
    }

    println!("Redirecting services to channel id '{}'", SLACK_CHANNEL_ID);

    return Ok(());
}
