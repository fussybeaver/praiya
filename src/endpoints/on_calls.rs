//! Method, error and parameter types for the On Calls endpoint.

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
pub struct OnCallsClient {
    pub(crate) api_endpoint: String,
    pub(crate) client: Praiya,
}

impl Praiya {
    pub fn on_calls(&self) -> OnCallsClient {
        OnCallsClient {
            api_endpoint: std::env::var("PAGERDUTY_API_ENDPOINT")
                .unwrap_or_else(|_| String::from(API_ENDPOINT)),
            client: Praiya::clone(self),
        }
    }
}

list_response_type!(ListOnCalls, oncalls, Oncall);

#[derive(praiya_macro::PraiyaParamsBuilder)]
#[doc = "[IncidentsClient::list_on_calls"]
#[allow(dead_code)]
struct ListOnCalls {
    earliest: bool,
    escalation_policy_ids: Vec<String>,
    include: Vec<String>,
    schedule_ids: Vec<String>,
    since: chrono::DateTime<chrono::Utc>,
    time_zone: chrono_tz::Tz,
    until: chrono::DateTime<chrono::Utc>,
    user_ids: Vec<String>,
}

impl OnCallsClient {
    /// ---
    ///
    /// # List all of the on-calls
    ///
    /// List the on-call entries during a given time range.
    ///
    /// ---
    pub fn list_on_calls(
        &self,
        query_params: ListOnCallsParams,
    ) -> impl Stream<Item = Result<Oncall, Error>> + '_ {
        self.client.list_request::<_, _, ListOnCallsResponse>(
            &self.api_endpoint,
            "/oncalls",
            query_params,
            PraiyaCustomHeaders::None,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::praiya::ParamsBuilder;
    use futures_util::TryStreamExt;

    #[tokio::test]
    async fn test_list_on_calls() {
        let pagerduty = crate::Praiya::new("test");

        let now = chrono::Utc::now();
        let since = now - chrono::Duration::days(1);
        let mut opts_builder = super::ListOnCallsParamsBuilder::new();
        opts_builder.earliest(true);
        opts_builder.escalation_policy_ids(vec![]);
        opts_builder.include(vec![]);
        opts_builder.schedule_ids(vec![]);
        opts_builder.since(&since);
        opts_builder.time_zone(&chrono_tz::EST);
        opts_builder.until(&now);
        opts_builder.user_ids(vec![]);
        let opts = opts_builder.build();

        let on_call: Option<Oncall> = pagerduty
            .on_calls()
            .list_on_calls(opts)
            .try_next()
            .await
            .unwrap();

        assert_eq!(
            on_call.unwrap().user.as_ref().unwrap().id.as_ref().unwrap(),
            &String::from("PT23IWX")
        );
    }
}
