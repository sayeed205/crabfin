//! Crabfin API client module
//!
//! This module handles all communication with servers,
//! including authentication, API requests, response parsing, and
//! multi-server connection management.

mod client;
mod error;
mod manager;

pub use client::*;
pub use manager::*;
