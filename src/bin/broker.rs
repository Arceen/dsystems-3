use futures_util::stream::StreamExt;
use futures_util::{future, TryStreamExt};
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    println!("Hello broker!");
    let stream = tokio::net::TcpListener::bind(("0.0.0.0", 8081)).await.unwrap();
    let (stream, _) = stream.accept().await.unwrap();
    let addr = stream.peer_addr().expect("something else");

    let ws_stream = accept_async(stream).await.expect("you should have known!");
    let (write, read) = ws_stream.split();
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary())).forward(write).await.unwrap();
}