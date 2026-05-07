use async_std::{
    io::{BufRead, BufReadExt, Write, WriteExt},
    stream::{Stream, StreamExt},
};
use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_as_json<R, D>(receiver: &mut R, data: D) -> ChatResult<()>
where
    R: Write + Unpin,
    D: Serialize,
{
    let mut json_string = serde_json::to_string(&data)?;
    json_string.push('\n');
    receiver.write_all(json_string.as_bytes()).await?;
    Ok(())
}

pub fn receive_as_json<S, D>(sender: S) -> impl Stream<Item = ChatResult<D>>
where
    S: BufRead + Unpin,
    D: DeserializeOwned,
{
    sender.lines().map(|line_result| -> ChatResult<D> {
        let line = line_result?;
        let parsed_data = serde_json::from_str(&line)?;
        Ok(parsed_data)
    })
}
pub fn log_chat_error(chat_result: ChatResult<()>) {
    if let Err(e) = chat_result {
        eprintln!("Error: {}", e);
    }
}
