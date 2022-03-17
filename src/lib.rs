//! [![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! [![docs](https://docs.rs/roctogen/badge.svg)](https://docs.rs/roctogen/)
//! [![GitHub workflow](https://github.com/github/docs/actions/workflows/default.yml/badge.svg)](https://github.com/fussybeaver/roctogen/actions/workflows/default.yml)
//!
//! # Roctogen: a rust client library for the GitHub v3 API  
//!
//! This client API is generated from the [upstream OpenAPI
//! specification](https://github.com/github/rest-api-description/). The library currently supports
//! webassembly and both tokio and non-tokio based asynchronous requests and minimal dependency blocking 
//! synchronous requests with a choice of different clients, enabled through cargo features:
//!
//!   - `isahc` feature (*sync* and non-tokio based *async*): [Isahc HTTP client](https://github.com/sagebind/isahc)
//!   - `reqwest` feature (*async*) [Reqwest client](https://github.com/seanmonstar/reqwest)
//!   - `ureq` feature (*sync*) [Ureq client](https://github.com/algesten/ureq) 
//!
//! # Install
//!
//! Add the following to your `Cargo.toml` file
//!
//! ```nocompile
//! [dependencies]
//! roctogen = "0.11"
//! ```
//!
//! # API
//! ## Documentation
//!
//! [API docs](https://docs.rs/roctogen/latest).
//! 
//! [Endpoints](https://docs.rs/roctogen/latest/roctogen/endpoints/index.html).
//!
//! Supported endpoints:
//! ---
//! 
//!   - [Priorities](https://docs.rs/roctogen/latest/roctogen/endpoints/priorities/struct.Priorities.html)
//!   - [LogEntries](https://docs.rs/roctogen/latest/roctogen/endpoints/log_entries/struct.LogEntries.html)
//!   - [Schedules](https://docs.rs/roctogen/latest/roctogen/endpoints/schedules/struct.Schedules.html)
//!   - [BusinessServices](https://docs.rs/roctogen/latest/roctogen/endpoints/business_services/struct.BusinessServices.html)
//!   - [Abilities](https://docs.rs/roctogen/latest/roctogen/endpoints/abilities/struct.Abilities.html)
//!   - [ExtensionSchemas](https://docs.rs/roctogen/latest/roctogen/endpoints/extension_schemas/struct.ExtensionSchemas.html)
//!   - [Rulesets](https://docs.rs/roctogen/latest/roctogen/endpoints/rulesets/struct.Rulesets.html)
//!   - [Vendors](https://docs.rs/roctogen/latest/roctogen/endpoints/vendors/struct.Vendors.html)
//!   - [EscalationPolicies](https://docs.rs/roctogen/latest/roctogen/endpoints/escalation_policies/struct.EscalationPolicies.html)
//!   - [ResponsePlays](https://docs.rs/roctogen/latest/roctogen/endpoints/response_plays/struct.ResponsePlays.html)
//!   - [AddOns](https://docs.rs/roctogen/latest/roctogen/endpoints/add_ons/struct.AddOns.html)
//!   - [OnCalls](https://docs.rs/roctogen/latest/roctogen/endpoints/on_calls/struct.OnCalls.html)
//!   - [Incidents](https://docs.rs/roctogen/latest/roctogen/endpoints/incidents/struct.Incidents.html)
//!   - [ServiceDependencies](https://docs.rs/roctogen/latest/roctogen/endpoints/service_dependencies/struct.ServiceDependencies.html)
//!   - [Users](https://docs.rs/roctogen/latest/roctogen/endpoints/users/struct.Users.html)
//!   - [Extensions](https://docs.rs/roctogen/latest/roctogen/endpoints/extensions/struct.Extensions.html)
//!   - [Analytics](https://docs.rs/roctogen/latest/roctogen/endpoints/analytics/struct.Analytics.html)
//!   - [Audit](https://docs.rs/roctogen/latest/roctogen/endpoints/audit/struct.Audit.html)
//!   - [Services](https://docs.rs/roctogen/latest/roctogen/endpoints/services/struct.Services.html)
//!   - [Teams](https://docs.rs/roctogen/latest/roctogen/endpoints/teams/struct.Teams.html)
//!   - [MaintenanceWindows](https://docs.rs/roctogen/latest/roctogen/endpoints/maintenance_windows/struct.MaintenanceWindows.html)
//!   - [Notifications](https://docs.rs/roctogen/latest/roctogen/endpoints/notifications/struct.Notifications.html)
//!   - [Tags](https://docs.rs/roctogen/latest/roctogen/endpoints/tags/struct.Tags.html)
//!
//! # Usage
//!
//! A quick example of this library:
//!
//! ```no_run
//! use roctogen::api::{self, repos};
//! use roctogen::auth::Auth;
//!
//! let auth = Auth::None;
//! let per_page = api::PerPage::new(10);
//! 
//! let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
//! params = params.author("fussybeaver").page(2);
//!
//! repos::new(&auth).list_commits("fussybeaver", "bollard", Some(params));
//! ```
//!
//! ## Async
//!
//! All the `async` methods are suffixed with `_async`, and are available on the wasm target or `isahc` and `reqwest` adapters.
//!
//! ## Webassembly
//!
//! To compile for webassembly, you can use [`wasm-pack`](https://github.com/rustwasm/wasm-pack) or compile with the
//! `wasm32-unknown-unknown` target:
//!
//! ```nocompile
//! $ wasm-pack build
//! ```
//!
//! ```nocompile
//! $ cargo build --target wasm32-unknown-unknown
//! ```
//!
//! If you are building a [cloudflare worker](https://workers.cloudflare.com/), you would use the
//! `wrangler` wrapper:
//!
//! ```nocompile
//! $ wrangler preview --watch
//! ```
//!
//! ## Client adapters
//!
//! Building on non-`wasm` targets generally requires adopting a feature for the desired
//! client adapter. 
//!
//! ### Isahc
//!
//! Compiling for the [`isahc`](https://github.com/sagebind/isahc) client required the `isahc` feature:
//!
//! ```nocompile
//! $ cargo build --features isahc
//! ```
//!
//! ### Reqwest
//!
//! Compiling for the [`reqwest`](https://github.com/seanmonstar/reqwest) client required the `reqwest` feature:
//!
//! ```nocompile
//! $ cargo build --features reqwest
//! ```
//!
//! ### Ureq
//!
//! Compiling for the [`ureq`](https://github.com/algesten/ureq) client required the `ureq` feature:
//!
//! ```nocompile
//! $ cargo build --features ureq
//! ```
//!
//! # GitHub preview features
//!
//! GitHub supports a phased rollout of non-stable endpoints behind header flags. These are
//! supported in this library through cargo feature flags. 
//!
//! ```nocompile
//! $ cargo build --features squirrel-girl
//! ```
//!
//! # Generate the API
//!
//! The majority of code is generated through the [Swagger OpenAPI
//! generator](https://github.com/swagger-api/swagger-codegen) (version 3).  Building requires the
//! [`mvn`](https://maven.apache.org/install.html) Java build tool, pegged at Java version 8 (so
//! you'll need an appropriate JDK).
//!
//! ```nocompile
//! $ mvn -D org.slf4j.simpleLogger.defaultLogLevel=info clean compiler:compile generate-resources
//! ```
//!
//! # Tests 
//!
//! Beware, tests that are not run with the `mock` feature are currently still doing real HTTP requests to the GitHub API.
//!
//! Run the wasm tests:
//!
//! ```nocompile
//! $ wasm-pack test --firefox --headless 
//! ```
//!
//! Run the sync tests:
//!
//! ```nocompile
//! $ cargo test --features isahc,mercy,squirrel-girl,inertia,starfox --target x86_64-unknown-linux-gnu -- --nocapture
//! ```
//!
//! In order to avoid GitHub's API rate limiting, you can run the non-wasm tests using wiremock.
//! You'll need to start wiremock in the background:
//!
//! ```nocompile
//! $ docker run -d --name wiremock -p 8080:8080 -v $PWD/tests/stubs:/home/wiremock
//! rodolpheche/wiremock
//! ```
//!
//! ### Regenerate the wiremock stubs
//!
//! You should regenerate the stubs if the remote API has changed:
//!
//! ```nocompile
//! $ docker run -d --name wiremock -p 8080:8080 -v $PWD/tests/stubs:/home/wiremock -u (id -u):(id -g) rodolpheche/wiremock --verbose --proxy-all="https://api.github.com" --record-mappings
//! ```
//!
#![allow(
    missing_docs,
    unused_imports,
)]

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

pub mod adapters;
pub mod endpoints;
pub mod models;

pub mod auth {
    pub enum Auth {
        Basic {
            user: String,
            pass: String,
        },
        Token(String),
        Bearer(String),
        None,
    }
}

pub use endpoints as api;
