//! Jellyfin API client module
//!
//! This module handles all communication with Jellyfin servers,
//! including authentication, API requests, and response parsing.

mod client;
mod error;

pub use client::*;
