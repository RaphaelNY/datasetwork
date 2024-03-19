use std::collections::HashMap;
use std::io;
use anyhow::Error;

#[derive(Debug)]
pub struct DatabaseConfig {
    pub host_port: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig{
    pub fn from_file(filename: &str) -> Result<Self, Error> {
        let config_str = std::fs::read_to_string(filename)?;
        let config: HashMap<String, toml::Value> = toml::from_str(&config_str)?;

        // 提取 [database] 表格中的字段
        let database_config = match config.get("database") {
            Some(database_table) => {
                let database_table = database_table
                    .as_table()
                    .ok_or(io::Error::new(io::ErrorKind::Other, "Expected a table for [database]"))?;

                let host_port = database_table
                    .get("host")
                    .and_then(|v| v.as_str())
                    .ok_or(io::Error::new(io::ErrorKind::Other, "host not found or not a string"))?
                    .to_string();

                let username = database_table
                    .get("username")
                    .and_then(|v| v.as_str())
                    .ok_or(io::Error::new(io::ErrorKind::Other, "username not found or not a string"))?
                    .to_string();

                let password = database_table
                    .get("password")
                    .and_then(|v| v.as_str())
                    .ok_or(io::Error::new(io::ErrorKind::Other, "password not found or not a string"))?
                    .to_string();

            DatabaseConfig { host_port, username, password }
        }
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "No [database] table found in config").into()),
        };

        Ok(database_config)
    }
}
