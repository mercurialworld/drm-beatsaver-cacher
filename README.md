# BeatSaverCacher

Like TheBlackParrot's, but written in Rust 🦀🚀 (mainly because I also maintain the BeatSaver Rust library).

Currently a direct port of the original, written in JavaScript. Not a lot of Rusty code is used here.

## Running

Make sure you have `protoc` installed. Then, run:

```sh
cargo build
```

You should also have an environment set as follows: 

```sh
RUST_LOG="drm_beatsaver_cacher"
```

To test:

```sh
cargo test # add -- --show-output if you want the actual output
```