pub struct SolanaConfig {
    pub rpc_url: String,
}

impl SolanaConfig {
    pub fn devnet() -> Self {
        Self {
            rpc_url: "https://api.devnet.solana.com".to_string(),
        }
    }

    pub fn from_env() -> Self {
        let rpc_url = std::env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        Self { rpc_url }
    }
}
