use async_std::task;
use chat_app::FromServer;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::connection::Outbound;

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}
impl Group {
    pub fn new(name: Arc<String>) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Group { name, sender }
    }
    pub fn join(&self, outbound: Arc<Outbound>) {
        let mut rx = self.sender.subscribe();
        let group_name = self.name.clone();
        task::spawn(async move {
            loop {
                let packet = match rx.recv().await {
                    Ok(message) => FromServer::Message {
                        group_name: group_name.clone(),
                        message,
                    },
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        let err_msg = format!("Dropped {} messages from group {}", n, group_name);
                        FromServer::Error(err_msg)
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                };
                if outbound.send(packet).await.is_err() {
                    break;
                }
            }
        });
    }
    pub fn post(&self, message: Arc<String>) {
        self.sender.send(message);
    }
}
