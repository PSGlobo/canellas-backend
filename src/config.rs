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
    pub username: String,
    /// Username's password
    pub password: String,
}

impl Settings {
    /// Creates an [Settings] instance based on the Environment Variables.
    ///
    /// See [tests] to check out how to set the ENV VARS locally (through .env).
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
            "postgres://{}:{}@{}:5432/cotid", // TODO extract database name to ENV VAR
            self.username, self.password, self.host
        )
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::Settings;

    /// Look at the source to see how you could set the ENV VARS in a development environment.
    fn init_env() {
        env::set_var("APP_HOST", "app_host");
        env::set_var("APP_PORT", "1234");
        env::set_var("APP_DATABASE__USERNAME", "test");
        env::set_var("APP_DATABASE__PASSWORD", "123456");
        env::set_var("APP_DATABASE__HOST", "db_host");
    }
    #[test]
    fn load_settings() {
        init_env();
        let settings = Settings::load();
        assert!(settings.is_ok());

        let settings = settings.unwrap();
        assert_eq!(settings.database.username, "test");
    }
}
