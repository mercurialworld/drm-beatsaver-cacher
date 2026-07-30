use std::collections::HashMap;

use beatsaver_api::models::map::MapDetail;
use drm_beatsaver_cacher::{
    cacher::{cache_map_data, write_cache},
    mapdata::{MapList, MapMetadata},
};
use prost::Message;
use rstest::fixture;

pub(crate) async fn write_focus() -> usize {
    let focus_but_drm = focus();

    let mut maps: MapList = MapList {
        map_metadata: HashMap::new(),
    };

    maps.map_metadata.insert("4b6f1".into(), focus_but_drm);

    write_cache(maps.encode_to_vec(), "testMapData.proto.gz")
        .await
        .unwrap()
}

#[fixture]
pub(crate) fn focus() -> MapMetadata {
    let map: MapDetail = serde_json::from_str(
        r#"{
  "id": "4b6f1",
  "name": "Hearts2Hearts - FOCUS",
  "description": "This song is soo damn good!\r\n\r\nStream the MV: https://youtu.be/Ur7aK4FvK-U\r\nKpop Beat Saber Discord: https://discord.gg/c9uHGYP\r\n\r\nThanks to ttv@earblind69 for the testplay!",
  "uploader": {
    "id": 120215,
    "name": "Jonas",
    "avatar": "https://cdn.beatsaver.com/avatar/1b5f7d300423115407e04723f3629d212ec08291.png",
    "type": "DISCORD",
    "admin": false,
    "curator": true,
    "seniorCurator": true,
    "curatorTab": true,
    "verifiedMapper": true,
    "playlistUrl": "https://api.beatsaver.com/users/id/120215/playlist"
  },
  "metadata": {
    "bpm": 130.0,
    "duration": 179,
    "songName": "FOCUS",
    "songSubName": "",
    "songAuthorName": "Hearts2Hearts",
    "levelAuthorName": "Jonas"
  },
  "stats": {
    "plays": 0,
    "downloads": 0,
    "upvotes": 204,
    "downvotes": 9,
    "score": 0.9338,
    "reviews": 5,
    "sentiment": "VERY_POSITIVE"
  },
  "uploaded": "2025-10-20T23:06:54.743598Z",
  "automapper": false,
  "ranked": false,
  "qualified": false,
  "versions": [
    {
      "hash": "e5c654bfaf385feda0df68a406dfadda471df62e",
      "state": "Published",
      "createdAt": "2025-10-20T23:06:45.205511Z",
      "sageScore": 4,
      "diffs": [
        {
          "njs": 10.0,
          "offset": 0.49,
          "notes": 212,
          "bombs": 0,
          "obstacles": 382,
          "nps": 1.222,
          "length": 376.0,
          "characteristic": "Standard",
          "difficulty": "Easy",
          "events": 1915,
          "chroma": false,
          "me": false,
          "ne": false,
          "cinema": true,
          "seconds": 173.538,
          "paritySummary": {
            "errors": 0,
            "warns": 0,
            "resets": 0
          },
          "maxScore": 187795,
          "environment": "NiceEnvironment"
        },
        {
          "njs": 12.0,
          "offset": -0.11,
          "notes": 474,
          "bombs": 0,
          "obstacles": 382,
          "nps": 2.735,
          "length": 375.5,
          "characteristic": "Standard",
          "difficulty": "Normal",
          "events": 1915,
          "chroma": false,
          "me": false,
          "ne": false,
          "cinema": true,
          "seconds": 173.308,
          "paritySummary": {
            "errors": 0,
            "warns": 0,
            "resets": 0
          },
          "maxScore": 428835,
          "environment": "NiceEnvironment"
        },
        {
          "njs": 13.0,
          "offset": -0.5,
          "notes": 586,
          "bombs": 0,
          "obstacles": 382,
          "nps": 3.386,
          "length": 375.0,
          "characteristic": "Standard",
          "difficulty": "Hard",
          "events": 1915,
          "chroma": false,
          "me": false,
          "ne": false,
          "cinema": true,
          "seconds": 173.077,
          "paritySummary": {
            "errors": 0,
            "warns": 0,
            "resets": 0
          },
          "maxScore": 531875,
          "environment": "NiceEnvironment"
        },
        {
          "njs": 15.0,
          "offset": -0.7,
          "notes": 779,
          "bombs": 0,
          "obstacles": 398,
          "nps": 4.498,
          "length": 375.25,
          "characteristic": "Standard",
          "difficulty": "Expert",
          "events": 1915,
          "chroma": false,
          "me": false,
          "ne": false,
          "cinema": true,
          "seconds": 173.192,
          "paritySummary": {
            "errors": 0,
            "warns": 0,
            "resets": 0
          },
          "maxScore": 709435,
          "environment": "NiceEnvironment"
        },
        {
          "njs": 16.0,
          "offset": -0.85,
          "notes": 778,
          "bombs": 0,
          "obstacles": 414,
          "nps": 4.492,
          "length": 375.25,
          "characteristic": "Standard",
          "difficulty": "ExpertPlus",
          "events": 1915,
          "chroma": false,
          "me": false,
          "ne": false,
          "cinema": true,
          "seconds": 173.192,
          "paritySummary": {
            "errors": 0,
            "warns": 0,
            "resets": 0
          },
          "maxScore": 708515,
          "environment": "NiceEnvironment"
        }
      ],
      "downloadURL": "https://r2cdn.beatsaver.com/e5c654bfaf385feda0df68a406dfadda471df62e.zip",
      "coverURL": "https://cfcdn.beatsaver.com/e5c654bfaf385feda0df68a406dfadda471df62e.jpg",
      "previewURL": "https://cfcdn.beatsaver.com/e5c654bfaf385feda0df68a406dfadda471df62e.mp3"
    }
  ],
  "curator": {
    "id": 106226,
    "name": "Taranchola",
    "avatar": "https://cdn.beatsaver.com/avatar/0788f766a846415b2457a064b3b6937c80ea49e1.png",
    "type": "DISCORD",
    "admin": false,
    "curator": true,
    "seniorCurator": false,
    "curatorTab": true,
    "playlistUrl": "https://api.beatsaver.com/users/id/106226/playlist"
  },
  "curatedAt": "2025-10-22T00:33:51.715837Z",
  "createdAt": "2025-10-20T23:06:45.205511Z",
  "updatedAt": "2025-10-22T00:33:51.715837Z",
  "lastPublishedAt": "2025-10-20T23:06:54.743598Z",
  "tags": [
    "tech",
    "dance-style",
    "k-pop"
  ],
  "bookmarked": false,
  "declaredAi": "None",
  "blRanked": false,
  "blQualified": false
}"#,
    ).unwrap();

    cache_map_data(&map).unwrap()
}
