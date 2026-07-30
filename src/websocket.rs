use std::time::Duration;

pub mod mapactivity;
pub mod voteactivity;

pub(crate) const BASE_WS_URL: &str = "wss://ws.beatsaver.com";
pub(crate) const RECONNECT_DELAY: Duration = Duration::from_secs(15);
