use std::sync::Arc;

use beatsaver_api::models::map::Map;
use futures::StreamExt;
use log::{debug, error, info, warn};
use prost::Message;
use serde::Deserialize;
use tokio::sync::RwLock;
use tokio_tungstenite::connect_async;

use crate::{
    cacher::cache_map_data,
    file::write_cache,
    mapdata::MapList,
    websocket::{BASE_WS_URL, RECONNECT_DELAY},
};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MapWebsocketMessage {
    MapDelete { msg: String },
    MapUpdate { msg: Box<Map> },
}

// [TODO] clean this up a little oh my god this is unhinged
pub async fn map_socket(map_lock: Arc<RwLock<MapList>>, cache_path: &str) {
    loop {
        match connect_async(format!("{}/maps", BASE_WS_URL)).await {
            Ok((ws, _)) => {
                info!("[Socket] BeatSaver map data socket connection established!");

                let (_, mut source) = ws.split();

                while let Some(Ok(msg)) = source.next().await {
                    if let Ok(m) = msg.to_text() {
                        match serde_json::from_str::<MapWebsocketMessage>(m) {
                            Ok(m) => {
                                let mut maps = map_lock.write().await;

                                match m {
                                    MapWebsocketMessage::MapUpdate { msg } => {
                                        info!(
                                            "Received new info for map id {}, attempting to cache",
                                            msg.id
                                        );

                                        if let Some(cached_map) = cache_map_data(&msg) {
                                            maps.map_metadata.insert(msg.id, cached_map);

                                            let _ =
                                                write_cache(maps.encode_to_vec(), cache_path).await;
                                        }
                                    }
                                    MapWebsocketMessage::MapDelete { msg } => {
                                        info!(
                                            "Received deletion info for map id {}, attempting to delete",
                                            msg
                                        );

                                        maps.map_metadata.remove_entry(&msg);
                                    }
                                }
                            }
                            Err(e) => {
                                if m.starts_with("[ping") {
                                    // debug!("ping");
                                } else {
                                    error!("Unable to deserialize message: {}", e);
                                }
                            }
                        }
                    } else {
                        debug!("Unknown message received: {}", msg);
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
