use anyhow::Context;
use anyhow::Error;

#[derive(Debug)]
pub struct DatabaseConfig {
    pub host_port: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig{
    pub fn from_file(filename: &str) -> Result<Self, Error> {
        let config = std::fs::read_to_string(filename)
            .with_context(|| format!("Error reading file: {}", filename))?;

        let config: toml::Value = toml::from_str(&config)
            .with_context(|| format!("Error parsing file: {}", filename))?;

        let host_port = config["host"].as_str().ok_or_else(|| Error::msg("host_port not found"))?.to_string();
        let username = config["username"].as_str().ok_or_else(|| Error::msg("username not found"))?.to_string();
        let password = config["password"].as_str().ok_or_else(|| Error::msg("password not found"))?.to_string();

        Ok(DatabaseConfig {
            host_port,
            username,
            password,
        })
    }
}
