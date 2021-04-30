//! Definition and usage of configuration

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

/// Holds all the configuration values of the application.
#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Database Settings
    pub database: Database,
    /// Host of the application. Usually ´0.0.0.0´ or ´localhost´ in development.
    pub host: String,
    /// The port that the application will listen to connections.
    pub port: u16,
}

/// Database configuration used to interact will the DBMS.
#[derive(Debug, Deserialize)]
pub struct Database {
    /// Database Host
    pub host: String,
    /// Identifier used to authenticate
    pub user: String,
    /// Username's password
    pub password: String,
    /// Database's name
    pub name: String,
}

impl Settings {
    /// Creates an [Settings] instance based on the Environment Variables.
    ///
    /// ```
    /// # use cotid_server::config::Settings;
    /// # use std::env;
    /// #
    /// // Set the following environment variables in your .env file
    /// env::set_var("APP_HOST", "app_host");
    /// env::set_var("APP_PORT", "1234");
    /// env::set_var("APP_DATABASE__USER", "test");
    /// env::set_var("APP_DATABASE__PASSWORD", "123456");
    /// env::set_var("APP_DATABASE__HOST", "db_host");
    /// env::set_var("APP_DATABASE__NAME", "cotid");
    ///
    /// let settings = Settings::load();
    /// assert!(settings.is_ok());
    ///
    /// let settings = settings.unwrap();
    /// assert_eq!(settings.database.user, "test");
    /// ```
    ///
    pub fn load() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(Environment::with_prefix("app").separator("__"))?;
        config.try_into()
    }
}

impl Database {
    /// URI with all the information needed to connect with the DBMS.
    pub fn uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}:5432/{}",
            self.user, self.password, self.host, self.name
        )
    }
}
