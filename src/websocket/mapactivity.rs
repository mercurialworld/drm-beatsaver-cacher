use beatsaver_api::models::map::Map;
use futures::StreamExt;
use log::{debug, info, warn};
use serde::Deserialize;
use tokio_tungstenite::connect_async;

use crate::websocket::{BASE_WS_URL, RECONNECT_DELAY};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MapWebsocketMessage {
    MapUpdate { msg: String },
    MapDelete { msg: Box<Map> },
}

pub async fn map_socket() {
    loop {
        match connect_async(format!("{}/maps", BASE_WS_URL)).await {
            Ok((ws, _)) => {
                info!("[Socket] BeatSaver map data socket connection established!");

                let (_, mut source) = ws.split();

                while let Some(Ok(msg)) = source.next().await {
                    if let Ok(m) = msg.to_text()
                        && let Ok(fmt_msg) = serde_json::from_str::<MapWebsocketMessage>(m)
                    {
                        debug!("received: {fmt_msg:?}");
                    } else {
                        debug!("received: {msg}");
                    }
                }

                info!(
                    "[Socket] BeatSaver map data socket connection closed, reconnecting in 15 seconds..."
                )
            }
            Err(e) => warn!(
                "[Socket] Failed to connect to BeatSaver's map data socket: {e}. Should try again in around 15 seconds..."
            ),
        }

        tokio::time::sleep(RECONNECT_DELAY).await;
    }
}
