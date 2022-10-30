//! [![crates.io](https://img.shields.io/crates/v/praiya.svg)](https://crates.io/crates/praiya)
//! [![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! [![docs](https://docs.rs/praiya/badge.svg)](https://docs.rs/praiya/)
//! [![GitHub workflow](https://github.com/github/docs/actions/workflows/main.yml/badge.svg)](https://github.com/fussybeaver/praiya/actions/workflows/main.yml)
//!
//!
//! # Praiya: an async PagerDuty API for Rust
//!
//! Praiya leverages the official [PagerDuty OpenAPI swagger
//! specification](https://github.com/PagerDuty/api-schema) to generate models and mock server
//! stubs, in order to closely match the real-world [PagerDuty
//! API](https://developer.pagerduty.com/api-reference/) behaviour. Praiya's async paradigm runs on
//! [Hyper](https://github.com/hyperium/hyper) and [Tokio](https://github.com/tokio-rs/tokio),
//! tests are run against a [Prism](https://stoplight.io/open-source/prism) server that matches the
//! OpenAPI specification responses.
//!
//! # Install
//!
//! Add the following to your `Cargo.toml` file.
//!
//! ```nocompile
//! [dependencies]
//! praiya = "*"
//! ```
//!
//! # API
//! ## Documentation
//!
//! [API docs](https://docs.rs/praiya/)
//!
//! Praiya has currently implemented the following API endpoints:
//!
//! - [ ] abilities
//! - [ ] add_ons
//! - [ ] analytics
//! - [ ] audit
//! - [ ] business_services
//! - [X] escalation_policies
//! - [ ] extension_schemas
//! - [ ] extensions
//! - [X] incidents
//! - [ ] log_entries
//! - [ ] maintenance_windows
//! - [ ] notifications
//! - [X] on_calls
//! - [ ] priorities
//! - [ ] response_plays
//! - [ ] rulesets
//! - [ ] schedules
//! - [ ] service_dependencies
//! - [X] services
//! - [X] slack_connections
//! - [ ] tags
//! - [ ] teams
//! - [X] users
//! - [ ] vendors
//!
//! # Usage
//!
//! ## Connecting to the PagerDuty API server
//!
//! A new `Praiya` client takes the PagerDuty API token and will build an SSL context:
//!
//! ```rust
//! praiya::Praiya::new("PAGERDUTY_TOKEN");
//!
//! ```
//!
//! ## Examples
//!
//! ### Listing incidents
//!
//! To list triggered and acknowledged incidents in your organisation:
//!
//! ```rust,no_run
//! use praiya::ParamsBuilder;
//!
//! use futures_util::TryStreamExt;
//!
//! let pagerduty = praiya::Praiya::new("PAGERDUTY_TOKEN");
//!
//! let mut opts_builder = praiya::endpoints::incidents::ListIncidentsParamsBuilder::new();
//! opts_builder.statuses(vec!["triggered", "acknowledged"]);
//! let opts = opts_builder.build();
//!
//! async move {
//!     let incidents: Vec<praiya::models::Incident> = pagerduty
//!         .incidents("from@example.com")
//!         .list_incidents(opts)
//!         .try_collect()
//!         .await
//!         .expect("Unable to list PagerDuty incidents");
//! };
//! ```
//!
//! # Development
//!
//! Contributions are welcome, please observe the following advice.
//!
//! ## Building the stubs
//!
//! Serialization stubs are generated through the [Swagger
//! library](https://github.com/swagger-api/swagger-codegen/). To generate these files, use the
//! following:
//!
//! ```bash
//! mvn -D org.slf4j.simpleLogger.defaultLogLevel=debug compiler:compile generate-resources
//! ```
//!
//! ## Mock test server
//!
//! The mock servers run with the [Prism](https://stoplight.io/open-source/prism) project against a
//! [forked branch](https://github.com/fussybeaver/pagerduty-api-schema/tree/praiya-master) of the
//! official PagerDuty API schema, in order to maintain stability of the Praiya CI pipelines.
//!
//! Mock servers are started with [Docker Compose](https://docs.docker.com/compose/):
//!
//! ```nocompile
//! docker-compose up -d
//! ```
//!
//! or alternatively use the npm library.
//!
//! ```nocompile
//! npm install -g @stoplight/prism-cli
//! # for example
//! prism mock https://raw.githubusercontent.com/fussybeaver/pagerduty-api-schema/praiya-master/reference/REST/openapiv3.json
//! ```
//!
//! ## Tests
//!
//! In order to run tests, point the client to the appropriate mock server:
//!
//! For the slack API:
//!
//! ```nocompile
//! env PAGERDUTY_API_ENDPOINT=http://127.0.0.1:8080 RUST_LOG=praiya=debug cargo test slack
//! ```
//!
//! For the default API's:
//!
//! ```nocompile
//! env PAGERDUTY_API_ENDPOINT=http://127.0.0.1:8081 RUST_LOG=praiya=debug cargo test incidents
//! env PAGERDUTY_API_ENDPOINT=http://127.0.0.1:8081 RUST_LOG=praiya=debug cargo test services
//! ...
//! ```
//!
//! ## Documentation
//!
//! This README is generated with [cargo-readme](https://github.com/livioribeiro/cargo-readme)
//!
//! ```nocompile
//! cargo readme --no-title > README.md
//! ```
//!
//! # License
//!
//! This software is licensed under the liberal [Apache License 2.0](https://opensource.org/licenses/Apache-2.0)
//!
#![allow(missing_docs, unused_imports)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
mod macros;

use std::collections::HashMap;

#[rustfmt::skip]
pub mod default_models;
pub mod endpoints;
pub mod errors;
mod praiya;
#[rustfmt::skip]
pub mod slack_models;

pub(crate) use crate::praiya::{
    BaseOption, BaseRequest, PaginatedResponse, PaginationQueryComponent, SingleResponse, SubSystem,
};
pub use crate::praiya::{ParamsBuilder, Praiya, DEFAULT_PAGERDUTY_API_LIMIT};

pub mod auth {
    pub enum Auth {
        Basic { user: String, pass: String },
        Token(String),
        Bearer(String),
        None,
    }
}

pub use default_models as models;
pub use endpoints as api;
