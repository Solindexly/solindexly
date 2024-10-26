// src/config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub rpc_url: String,
    pub tracked_program_id: String, // Program ID to track
}
