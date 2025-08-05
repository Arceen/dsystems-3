use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    while let Ok((stream_sock, _)) = listener.accept().await {
        println!("Socket received from: {}", stream_sock.peer_addr().unwrap());
        let _ = tokio::spawn(async move {
            let mut ws_socket = tokio_tungstenite::accept_async(stream_sock).await.unwrap();
            while let Some(msg) = ws_socket.next().await {
                ws_socket.send(msg.unwrap()).await;
            }
        }).await;
    }
}
