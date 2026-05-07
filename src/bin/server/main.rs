use crate::{connection::serve, group_table::GroupTable};
use async_std::{net::TcpListener, stream::StreamExt};
use chat_app::utils::{ChatResult, log_chat_error};
use std::{env::args, sync::Arc};

mod connection;
mod group;
mod group_table;

fn main() -> ChatResult<()> {
    let address = args().nth(1).expect("usage: server <address>");
    let chat_group_table = Arc::new(GroupTable::new());
    async_std::task::block_on(async {
        let listener = TcpListener::bind(address).await?;
        while let Some(socket_result) = listener.incoming().next().await {
            let socket = socket_result?;
            let groups = chat_group_table.clone();
            async_std::task::spawn(async {
                log_chat_error(serve(socket, groups).await);
            });
        }
        Ok(())
    })
}
