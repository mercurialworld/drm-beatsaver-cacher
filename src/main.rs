use beatsaver_api::client::BeatSaverClient;
use drm_beatsaver_cacher::cacher::{init_cache, write_cache};
use prost::Message;

#[tokio::main]
async fn main() {
    env_logger::init();

    let beatsaver_api = BeatSaverClient::default();

    let maps = init_cache(&beatsaver_api).await;

    let _write_res = write_cache(maps.encode_to_vec(), "mapData.proto.gz").await;
}
