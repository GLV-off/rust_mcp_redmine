//! Core Redmine API client, configuration, and error types.
//!
//! This crate provides [`RedmineClient`](client::RedmineClient) — a high-level
//! async wrapper around the `redmine-api` crate — along with CLI/environment
//! [`Config`](config::Config) and a unified [`CoreError`](error::CoreError) type.

pub mod client;
pub mod config;
pub mod error;
