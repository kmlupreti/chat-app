use std::{env::args, sync::Arc};

use async_std::{
    io::{BufReadExt, BufReader, stdin},
    net::TcpStream,
    stream::StreamExt,
};
use chat_app::{FromClient, utils::ChatResult};

async fn send_commands(to_server: TcpStream) -> ChatResult<()> {
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
        _ => None,
    }
}
fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let command = args.join(" ");
    println!("{:?}", parse_command(command));
}
