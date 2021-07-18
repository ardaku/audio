// Copyright 2016 Tad Hardesty
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// **Note**: Currently converting C to Rust - Use
// [libopus documentation](https://opus-codec.org/docs/opus_api-1.1.2/) to help.

//! Ogg Opus encoder and decoder in pure Rust (WIP - currently not pure Rust).
#![warn(missing_docs)]

extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate dl_api;

pub mod old;

mod ffi;
mod stream_encoder;

pub use stream_encoder::*;
