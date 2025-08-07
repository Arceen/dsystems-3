use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

#[tokio::main]
async fn main() {
    let request = "ws://127.0.0.1:8080".into_client_request().unwrap();
    if let Ok((stream, response)) = tokio_tungstenite::connect_async(request).await {
        let _ = tokio::spawn(async move {
            let (mut write, read) = stream.split();
            let read_handler = tokio::spawn(async move {
                read.for_each(|msg| async {
                    let data = msg.unwrap().into_data();
                    println!("{}", String::from_utf8(data.to_vec()).unwrap());
                })
                .await;
            });
            let write_handler = tokio::spawn(async move {
                let mut i = 0_u32;
                loop {
                    println!("sending stuff!");
                    let _ = write.send(Message::Text(format!("{}\n", i).into())).await;
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    i += 1;
                }
            });
            let _ = tokio::join!(read_handler, write_handler);
        })
        .await
        .expect("could not await client thread");
    }
}
