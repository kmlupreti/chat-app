use std::{env::args, sync::Arc};

use async_std::{
    io::{BufReadExt, BufReader, stdin},
    net::TcpStream,
    prelude::FutureExt,
    stream::StreamExt,
    task,
};
use chat_app::{
    FromClient, FromServer,
    utils::{self, ChatResult},
};

async fn handle_replies(from_server: TcpStream) -> ChatResult<()> {
    let buf_reader = BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buf_reader);
    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message in group {}: {}", group_name, message);
            }
            FromServer::Error(msg) => {
                println!("error from server: {}", msg);
            }
        }
    }
    Ok(())
}
async fn send_commands(mut to_server: TcpStream) -> ChatResult<()> {
    println!(
        "Commands: 
              join GROUP
              post GROUP MESSAGE... 
              Type Control-D (on Unix) or Control-Z (on Windows) 
              to close the connection."
    );
    let mut command_lines = BufReader::new(stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;
        let request = match parse_command(command) {
            Some(request) => request,
            None => continue,
        };
        utils::send_as_json(&mut to_server, request).await?;
    }
    Ok(())
}

fn parse_command(command: String) -> Option<FromClient> {
    let mut command = command.split_whitespace();
    match command.next() {
        Some("join") => Some(FromClient::Join {
            group_name: Arc::new(command.next().unwrap().to_string()),
        }),
        Some("post") => {
            let group = command.next().unwrap().to_string();
            let message = command.collect::<Vec<_>>().join(" ");
            Some(FromClient::Post {
                group_name: Arc::new(group),
                message: Arc::new(message),
            })
        }
        _ => {
            eprintln!("invalid command received");
            None
        }
    }
}
fn main() -> ChatResult<()> {
    let address = args().nth(1).expect("usage: client <address>");
    task::block_on(async {
        let socket = TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;
        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);
        from_server.race(to_server).await?;
        Ok(())
    })
}
