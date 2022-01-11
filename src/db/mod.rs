use anyhow::Result;
use async_trait::async_trait;
use openidconnect::{Nonce, RedirectUrl};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
mod redis;
#[cfg(not(target_arch = "wasm32"))]
pub use redis::RedisClient;
#[cfg(target_arch = "wasm32")]
mod cf;
#[cfg(target_arch = "wasm32")]
pub use cf::CFClient;

const KV_CLIENT_PREFIX: &str = "clients";
const ENTRY_LIFETIME: usize = 30;

#[derive(Clone, Serialize, Deserialize)]
pub struct CodeEntry {
    pub exchange_count: usize,
    pub address: String,
    pub nonce: Option<Nonce>,
    pub client_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClientEntry {
    pub secret: String,
    pub redirect_uris: Vec<RedirectUrl>,
}

// Using a trait to easily pass async functions with async_trait
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DBClient {
    async fn set_client(&self, client_id: String, client_entry: ClientEntry) -> Result<()>;
    async fn get_client(&self, client_id: String) -> Result<Option<ClientEntry>>;
    async fn set_code(&self, code: String, code_entry: CodeEntry) -> Result<()>;
    async fn get_code(&self, code: String) -> Result<Option<CodeEntry>>;
}