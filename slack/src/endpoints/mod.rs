//! Endpoints module and `PerPage` struct/impl
/* 
 * PagerDuty Slack Integration API
 *
 * This is API documentation for PagerDuty's integration with Slack.   To begin the integration, you will need to map your PagerDuty account to your Slack Workspace. After configuring an Extension on each Service, Incident notifications will be posted in the configured Slack channel.  Additional integration documentation can be found here: https://support.pagerduty.com/docs/slack-integration-guide.  > ### Important > >  Ensure you are using a User API Token to make requests to this API. Also, check that your user has necessary >  permissions (Owner or Global Admin). See https://developer.pagerduty.com/docs/rest-api-v2/authentication/ for the >  details.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: support@pagerduty.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

pub const GITHUB_BASE_API_URL: &str = if cfg!(feature = "mock") {
    "http://localhost:8080"
} else {
    "https://api.github.com"
};

pub mod slack_connections;

pub struct PerPage {
    per_page: u16,
    page: u16,
}

impl PerPage {
    pub fn new(per_page: u16) -> Self {
        PerPage { per_page, page: 0 }
    }

    pub fn page(&mut self, page: u16) -> &mut Self {
        self.page = page;
        self
    }
}

impl std::convert::AsRef<PerPage> for PerPage {
    fn as_ref(&self) -> &PerPage {
        self
    }
}