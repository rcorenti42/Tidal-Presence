mod discord_ipc;

use discord_ipc::DiscordClient;
use futures_util::StreamExt;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

const APP_ID: &str = "APP_ID";

#[derive(Debug, Deserialize)]
struct TrackData {
    title: Option<String>,
    artist: Option<String>,
    duration: u64,
    position: u64,
    playing: bool,
}

#[tokio::main]
async fn main() {
    println!("Starting WebSocket server on 0.0.0.0:9001");

    let listener = TcpListener::bind("0.0.0.0:9001")
        .await
        .expect("Failed to bind port");

    let mut discord =
        DiscordClient::connect(APP_ID).expect("Failed to connect to Discord IPC");

    println!("Connected to Discord IPC");

    loop {
        let (stream, _) = listener.accept().await.expect("Accept failed");
        println!("Android connected");

        let mut ws = accept_async(stream)
            .await
            .expect("WebSocket accept failed");

        while let Some(msg) = ws.next().await {
            let msg = msg.expect("WS message error");

            if msg.is_text() {
                if let Ok(data) = serde_json::from_str::<TrackData>(msg.to_text().unwrap()) {
                    update_presence(&mut discord, data);
                }
            }
        }

        println!("Android disconnected");
    }
}

fn update_presence(client: &mut DiscordClient, data: TrackData) {
    if data.title.is_none() {
        let _ = client.clear();
        println!("Presence cleared");
        return;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let start = if data.playing {
        Some(now - (data.position as i64 / 1000))
    } else {
        None
    };

    let end = if data.playing {
        Some(start.unwrap() + (data.duration as i64 / 1000))
    } else {
        None
    };

    let _ = client.set_activity(
        data.title.as_deref().unwrap_or(""),
        data.artist.as_deref().unwrap_or(""),
        start,
        end,
    );

    println!(
        "Updated: {} — {} | playing: {}",
        data.title.unwrap_or_default(),
        data.artist.unwrap_or_default(),
        data.playing
    );
}