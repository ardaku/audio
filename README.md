# Rust Ogg Opus Library
Ogg Opus encoder and decoder in pure Rust (WIP - currently not pure Rust).

## Goals
- APIs to encode/decode an Opus stream (optionally inside) an Ogg container.
- Re-implementation of functionality from libogg, libopus, and libopusfile.
- Fast
- No Unsafe
- Pure Rust
- High Level and Low Level APIs

## Getting Started
Examples can be found in the [Documentation](https://docs.rs/ogg_opus) and the
examples folder.

- Internet radio (Opus stream) server
  [internet_radio](https://github.com/libcala/ogg_opus/blob/master/examples/internet_radio.rs)
- Save to opus file
  [opus_file](https://github.com/libcala/ogg_opus/blob/master/examples/opus_file.rs)
- Play audio from Opus file or internet radio (Opus stream)
  [player](https://github.com/libcala/ogg_opus/blob/master/examples/player.rs)

## License
The `ogg_opus` crate is distributed under any of

- The terms of the
  [MIT License](https://github.com/libcala/ogg_opus/blob/master/LICENSE-MIT)
- The terms of the
  [Apache License (Version 2.0)](https://github.com/libcala/ogg_opus/blob/master/LICENSE-APACHE)
- The terms of the
  [Zlib License](https://github.com/libcala/ogg_opus/blob/master/LICENSE-ZLIB)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.
