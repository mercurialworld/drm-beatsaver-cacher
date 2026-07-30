use std::io::Read;
use std::{fs::File, path::Path};

use drm_beatsaver_cacher::mapdata::MapList;
use drm_beatsaver_cacher::mapdata::MapMetadata;
use flate2::read::GzDecoder;
use prost::Message;
use rstest::*;

use crate::common::{focus, write_focus};

mod common;

#[rstest]
#[test]
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

#[tokio::test]
async fn it_writes() {
    let _ = write_focus().await;
}

#[rstest]
#[tokio::test]
async fn it_reads(focus: MapMetadata) {
    let focus_but_drm = focus;

    let written_size = write_focus().await;

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

    assert_eq!(written_size, size);

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
