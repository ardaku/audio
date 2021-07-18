// RAW Audio
// Copyright © 2021 Jeron Aldaron Lau.
//
// Licensed under the Boost Software License, Version 1.0
// (https://www.boost.org/LICENSE_1_0.txt or see accompanying file
// LICENSE_BOOST_1_0.txt)
//
//! Crate for loading various RAW audio formats into fon types (*.pcm file
//! extension should be preferred to *.raw, as *.raw can be used for video as
//! well).  RAW audio formats are always interleaved.
//!
//! Channel order is assumed to follow the FLAC channel order (following
//! SMPTE/ITU-R recommendations):
//!  - 1 Channel: Mono (Mono)
//!  - 2 Channels: Stereo (Left, Right)
//!  - 3 Channels: Surround 3.0 (Left, Right, Center)
//!  - 4 Channels: Surround 4.0 (F.Left, F.Right, B.Left, B.Right)
//!  - 5 Channels: Surround 5.0 (F.Left, F.Right, F.Center, B.Left, B.Right)
//!  - 6 Channels: Surround 5.1 (F.Left, F.Right, F.Center, LFE, B.Left,
//!    B.Right)
//!  - 7 Channels: Surround 6.1 (F.Left, F.Right, F.Center, LFE, B.Center,
//!    S.Left, S.Right)
//!  - 8 Channels: Surround 7.1 (F.Left, F.Right, F.Center, LFE, B.Left,
//!    B.Right, S.Left, S.Right)
//!
//! For supported RAW Sample Formats see the [pcm](crate::pcm) module.

mod decoder;
mod encoder;
pub mod pcm;

pub use decoder::Decoder;
pub use encoder::Encoder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
