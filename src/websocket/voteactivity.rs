use futures::StreamExt;
use log::{debug, info, warn};
use serde::Deserialize;
use tokio_tungstenite::connect_async;

use crate::websocket::{BASE_WS_URL, RECONNECT_DELAY};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VoteMessage {
    pub hash: String,
    pub map_id: String,
    pub upvotes: i32,
    pub downvotes: i32,
    pub score: f64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum VoteWebsocketMessage {
    Vote { msg: VoteMessage },
}

pub async fn vote_socket() {
    loop {
        match connect_async(format!("{}/votes", BASE_WS_URL)).await {
            Ok((ws, _)) => {
                info!("[Socket] BeatSaver vote data socket connection established!");

                let (_, mut source) = ws.split();

                while let Some(Ok(msg)) = source.next().await {
                    if let Ok(m) = msg.to_text()
                        && let Ok(fmt_msg) = serde_json::from_str::<VoteWebsocketMessage>(m)
                    {
                        debug!("received: {fmt_msg:?}");
                    } else {
                        debug!("received: {msg}");
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
