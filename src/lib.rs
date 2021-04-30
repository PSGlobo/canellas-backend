//! Cotid Server

#![warn(missing_docs)]

pub mod app;
pub mod config;
pub mod db;
mod handlers;

pub use app::run_app;
