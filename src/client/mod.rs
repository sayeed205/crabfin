//! Jellyfin API client module
//! 
//! This module handles all communication with Jellyfin servers,
//! including authentication, API requests, and response parsing.

pub mod api;
pub mod http;

pub use api::*;
pub use http::*;