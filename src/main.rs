use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use beatsaver_api::client::BeatSaverClient;
use chrono::DateTime;
use drm_beatsaver_cacher::file::{read_gzip, write_cache};
use drm_beatsaver_cacher::routes::health;
use drm_beatsaver_cacher::{
    cacher::init_cache,
    config::CacherConfig,
    mapdata::MapList,
    websocket::{mapactivity::map_socket, voteactivity::vote_socket},
};
use log::{debug, info};
use prost::Message;
use tokio::net::TcpListener;
use tokio::{join, sync::RwLock};
use tower_http::services::ServeFile;
use tower_http::trace::TraceLayer;

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());

    let _ = axum::serve(listener, app.layer(TraceLayer::new_for_http())).await;
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

            if let Some(mut m) = read_gzip::<MapList>(&cache_path).await {
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

            let _write_res = write_cache((ms.encode_to_vec()).to_vec(), &cache_path).await;
        }
    }

    // api
    let app: Router = Router::new()
        .route("/health", get(health))
        .route_service("/cache", ServeFile::new(&cache_path));

    // webserver and sockets
    join!(
        serve(app, 5000),
        map_socket(maps.clone(), &cache_path),
        vote_socket(maps.clone())
    );
}
