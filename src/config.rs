use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub auth_token: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| {
                    "postgresql://app:UjIwa8vxDILjn2FExvRKPO1L5lJ@127.0.0.1:5432/postgres"
                        .to_string()
                }),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            auth_token: env::var("AUTH_TOKEN").ok(),
        })
    }
}

