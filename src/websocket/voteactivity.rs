use std::sync::Arc;

use futures::StreamExt;
use log::{debug, info, warn};
use serde::Deserialize;
use tokio::sync::RwLock;
use tokio_tungstenite::connect_async;

use crate::{
    cacher::protogen::generate_protobuf_votes,
    mapdata::{MapList, MapMetadata},
    websocket::{BASE_WS_URL, RECONNECT_DELAY},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteMessage {
    pub hash: String,
    pub map_id: String,
    pub upvotes: i32,
    pub downvotes: i32,
    pub score: f64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Undocumented message for the voting data websocket.
/// There is [only one event](https://git.gay/TheBlackParrot/BeatSaverCacher/src/commit/d663b1a7d273c7fd5626734b737b54edb01763ed/main.js#L14) according to Parrot.
pub enum VoteWebsocketMessage {
    Vote { msg: VoteMessage },
}

pub fn update_map(mut map: MapMetadata, new_votes: VoteMessage) -> MapMetadata {
    map.votes = generate_protobuf_votes(new_votes.upvotes, new_votes.downvotes, new_votes.score);

    map
}

pub async fn vote_socket(map_lock: Arc<RwLock<MapList>>) {
    loop {
        match connect_async(format!("{}/votes", BASE_WS_URL)).await {
            Ok((ws, _)) => {
                info!("[Socket] BeatSaver vote data socket connection established!");

                let (_, mut source) = ws.split();

                while let Some(Ok(msg)) = source.next().await {
                    if let Ok(m) = msg.to_text() {
                        match serde_json::from_str::<VoteWebsocketMessage>(m) {
                            Ok(fmt_msg) => match fmt_msg {
                                VoteWebsocketMessage::Vote { msg } => {
                                    info!("Received vote info for {}", msg.map_id);
                                    let maps = map_lock.write().await;

                                    let mut new_maps = maps.clone();

                                    let map_key = msg.map_id.clone();

                                    if let Some(map) = maps.map_metadata.get(&map_key) {
                                        new_maps
                                            .map_metadata
                                            .insert(map_key, update_map(map.clone(), msg));
                                    }
                                }
                            },
                            Err(_) => {
                                if m.starts_with("[ping") {
                                    debug!("ping");
                                }
                            }
                        }
                    } else {
                        debug!("Unknown message received: {msg}");
                    }
                }

                info!(
                    "[Socket] BeatSaver vote data socket connection closed, reconnecting in 15 seconds..."
                )
            }
            Err(e) => warn!(
                "[Socket] Failed to connect to BeatSaver's vote data socket: {e}. Should try again in around 15 seconds..."
            ),
        }

        tokio::time::sleep(RECONNECT_DELAY).await;
    }
}
