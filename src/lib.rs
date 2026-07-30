pub mod cacher;
pub mod config;
pub mod websocket;

pub mod mapdata {
    include!(concat!(env!("OUT_DIR"), "\\cached_beat_saver_data.rs"));
}
