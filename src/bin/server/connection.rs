use std::sync::Arc;

use async_std::{io::BufReader, net::TcpStream, stream::StreamExt};
use async_std::{io::WriteExt, sync::Mutex};
use chat_app::FromClient;
use chat_app::{
    FromServer,
    utils::{ChatResult, receive_as_json, send_as_json},
};

use crate::group_table::GroupTable;

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(to_client: TcpStream) -> Self {
        Self(Mutex::new(to_client))
    }

    pub async fn send(&self, data: FromServer) -> ChatResult<()> {
        let mut mutex_guard = self.0.lock().await;
        send_as_json(&mut *mutex_guard, data).await?;
        mutex_guard.flush().await?;
        Ok(())
    }
}
pub async fn serve(socket: TcpStream, groups: Arc<GroupTable>) -> ChatResult<()> {
    let outbound = Arc::new(Outbound::new(socket.clone()));
    let buf_reader = BufReader::new(socket);
    let mut client_request_stream = receive_as_json(buf_reader);
    while let Some(request_result) = client_request_stream.next().await {
        let request = request_result?;
        match request {
            FromClient::Join { group_name } => {
                let group = groups.get_or_create(group_name);
                group.clone().join(outbound.clone());
            }
            FromClient::Post {
                group_name,
                message,
            } => {
                let group = groups
                    .get(&*group_name)
                    .expect("unable to join group {group_name}");
                group.post(message);
            }
        }
    }
    Ok(())
}
