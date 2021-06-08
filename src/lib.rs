//! # PS Globo Backend
//!
//! This project is the server side of the BBB voting system built to practice for the Selective Process of Globo.
//!
//! ## Setup Instructions
//!
//! After installing the dependencies listed on the README:
//! - Setup the environment variables needed by [Settings][config::Settings::load]. Take a look at its documentation.
//!     - You may create a `.env` file on the project's root to add them.
//! - Run `docker-compose up -d` to run the database in the background.
//!     - You may skip it if you prefer to setup the database locally.
//! - Run `cargo test` to confirm everything is good.
//!
//! ## Project Structure
//!
//! The [app] module contains the server initialization, including the routes served by this service.
//!
//! To start the server, you'll need to take a look at [App] and [Settings][config::Settings].
//!

#![warn(missing_docs)]

pub mod app;
pub mod config;
pub mod db;
mod handlers;

pub use app::App;
