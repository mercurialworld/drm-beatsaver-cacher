use std::io::Read;
use std::{collections::HashMap, fs::File, path::Path};

use drm_beatsaver_cacher::mapdata::MapMetadata;
use drm_beatsaver_cacher::{cacher::write_cache, mapdata::MapList};
use flate2::read::GzDecoder;
use prost::Message;
use rstest::*;

use crate::common::focus;

mod common;

#[rstest]
fn it_caches_focus(focus: MapMetadata) {
    // https://beatsaver.com/maps/4b6f1
    let focus_but_drm = focus;

    // DRM-specific:

    // curator
    assert_eq!(focus_but_drm.curator_name, Some("Taranchola".into()));

    // mod requirements/suggestions (should just be cinema)
    assert_eq!(focus_but_drm.mods, 1);

    // upload time and last updated are different
    assert_ne!(focus_but_drm.uploaded, focus_but_drm.last_updated);
}

#[rstest]
#[tokio::test]
async fn it_writes(focus: MapMetadata) {
    let focus_but_drm = focus;

    let mut maps: MapList = MapList {
        map_metadata: HashMap::new(),
    };

    maps.map_metadata.insert("4b6f1".into(), focus_but_drm);

    write_cache(maps.encode_to_vec(), "testMapData.proto.gz")
        .await
        .unwrap();
}

#[rstest]
fn it_reads(focus: MapMetadata) {
    let focus_but_drm = focus;

    let path = Path::new("testMapData.proto.gz");
    let display = path.display();

    // Open file
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents
    let mut file_bytes: Vec<u8> = Vec::new();
    let size = match file.read_to_end(&mut file_bytes) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(s) => s,
    };

    // Decompress file, then write as bytes
    let mut d = GzDecoder::new(&file_bytes[..size]);
    let mut map_buffer: Vec<u8> = Vec::new();
    let map_buf_size = d.read_to_end(&mut map_buffer).unwrap();

    // Now we can actually get data
    let proto_cache = MapList::decode(&map_buffer[..map_buf_size]).unwrap();

    // validation
    assert!(proto_cache.map_metadata.contains_key("4b6f1"));
    assert_eq!(proto_cache.map_metadata.len(), 1);

    let focus_but_cached = proto_cache.map_metadata.get("4b6f1").unwrap();
    println!("{:?}", focus_but_cached);

    assert_eq!(*focus_but_cached, focus_but_drm);
}
