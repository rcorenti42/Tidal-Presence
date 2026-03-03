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
    println!("Starting WebSocket server on 0.0.0.0:3000");

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port");

    let mut discord =
        DiscordClient::connect(APP_ID).expect("Failed to connect to Discord IPC");

    println!("Connected to Discord IPC");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Android connected: {}", addr);

                let ws = match accept_async(stream).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        println!("WebSocket handshake failed: {}", e);
                        continue;
                    }
                };

                handle_connection(ws, &mut discord).await;

                println!("Android disconnected");
            }
            Err(e) => {
                println!("Accept error: {}", e);
            }
        }
    }
}

async fn handle_connection(
    mut ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    discord: &mut DiscordClient,
) {
    while let Some(msg) = ws.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() {
                    match serde_json::from_str::<TrackData>(msg.to_text().unwrap()) {
                        Ok(data) => update_presence(discord, data),
                        Err(e) => println!("JSON parse error: {}", e),
                    }
                }
            }
            Err(e) => {
                println!("WS error (client likely disconnected): {}", e);
                break;
            }
        }
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
        start.map(|s| s + (data.duration as i64 / 1000))
    } else {
        None
    };

    if let Err(e) = client.set_activity(
        data.title.as_deref().unwrap_or(""),
        data.artist.as_deref().unwrap_or(""),
        start,
        end,
    ) {
        println!("Discord IPC error: {}", e);
    }

    println!(
        "Updated: {} — {} | playing: {}",
        data.title.unwrap_or_default(),
        data.artist.unwrap_or_default(),
        data.playing
    );
}