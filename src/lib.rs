//! [![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! [![docs](https://docs.rs/roctogen/badge.svg)](https://docs.rs/roctogen/)
//! [![GitHub workflow](https://github.com/github/docs/actions/workflows/default.yml/badge.svg)](https://github.com/fussybeaver/roctogen/actions/workflows/default.yml)
//!
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
#![allow(missing_docs, unused_imports)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
mod gen;

use std::collections::HashMap;

pub mod adapters;
pub mod endpoints;
pub mod errors;
pub mod models;
mod praiya;

pub(crate) use crate::praiya::{
    BaseOption, BaseRequest, PaginatedResponse, PaginationQueryComponent, SingleResponse, SubSystem,
};
pub use crate::praiya::{Praiya, DEFAULT_PAGERDUTY_API_LIMIT};

pub mod auth {
    pub enum Auth {
        Basic { user: String, pass: String },
        Token(String),
        Bearer(String),
        None,
    }
}

pub use endpoints as api;
