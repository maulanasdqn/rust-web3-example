pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        Self { host, port }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
