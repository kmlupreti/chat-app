use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub enum FromClient {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}
