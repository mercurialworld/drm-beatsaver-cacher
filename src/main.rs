use std::sync::Arc;
use std::{fs::File, io::Read, path::Path};

use beatsaver_api::client::BeatSaverClient;
use chrono::DateTime;
use drm_beatsaver_cacher::{
    cacher::{init_cache, write_cache_uncompressed},
    config::CacherConfig,
    mapdata::MapList,
    websocket::{mapactivity::map_socket, voteactivity::vote_socket},
};
use log::{error, info};
use prost::Message;
use tokio::{join, sync::RwLock};

async fn load_cache(path: &str) -> Option<MapList> {
    let p = Path::new(path);

    let mut file = match File::open(p) {
        Ok(f) => f,
        Err(e) => {
            error!("Error opening {path}: {}", e);
            return None;
        }
    };

    let mut file_bytes: Vec<u8> = Vec::new();
    let size = match file.read_to_end(&mut file_bytes) {
        Ok(s) => s,
        Err(e) => {
            error!("Error reading {path}: {}", e);
            return None;
        }
    };

    let maps = MapList::decode(&file_bytes[..size]).unwrap();

    Some(maps)
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = CacherConfig::new("config.json").unwrap();
    let cache_path = config.cache_file;
    let maps = Arc::new(RwLock::new(MapList::default()));

    if config.init_cache_on_start {
        let beatsaver_api = BeatSaverClient::default();

        {
            let mut ms = maps.write().await;

            if let Some(mut m) = load_cache(&cache_path).await {
                info!("Cache has been successfully loaded, updating it...");

                // sort by date descending
                let mut sorted_maps: Vec<_> = m.map_metadata.iter().collect();
                sorted_maps.sort_by_key(|map| std::cmp::Reverse(map.1.uploaded));

                let secs = sorted_maps[0].1.uploaded;

                let recent_map_date = DateTime::from_timestamp(secs.into(), 0);
                info!(
                    "Last map cached was uploaded {}",
                    recent_map_date.unwrap().to_rfc3339()
                );

                let new_maps = init_cache(&beatsaver_api, recent_map_date).await;

                m.map_metadata.extend(new_maps.map_metadata);
                *ms = m;
            } else {
                info!("Creating new cache...");
                *ms = init_cache(&beatsaver_api, None).await;
            }

            info!("{} maps have been cached.", ms.map_metadata.len());

            let _write_res =
                write_cache_uncompressed((ms.encode_to_vec()).to_vec(), &cache_path).await;
        }
    }

    join!(
        map_socket(maps.clone(), &cache_path),
        vote_socket(maps.clone())
    );
}
