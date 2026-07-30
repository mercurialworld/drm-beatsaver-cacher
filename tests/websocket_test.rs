use drm_beatsaver_cacher::websocket::mapactivity::MapWebsocketMessage;
use rstest::rstest;

mod common;

use crate::common::focus_json;

#[rstest]
#[test]
fn it_deserializes_map(focus_json: String) {
    let update_msg = format!("{{ \"type\": \"MAP_UPDATE\", \"msg\": {} }}", focus_json);

    let update_serialized = serde_json::from_str::<MapWebsocketMessage>(&update_msg).unwrap();

    match update_serialized {
        MapWebsocketMessage::MapDelete { msg: _ } => panic!("what did you do here"),
        MapWebsocketMessage::MapUpdate { msg: _ } => println!("{:?}", update_serialized),
    }
}
