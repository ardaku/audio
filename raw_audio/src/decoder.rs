// RAW Audio
// Copyright © 2021 Jeron Aldaron Lau.
//
// Licensed under the Boost Software License, Version 1.0
// (https://www.boost.org/LICENSE_1_0.txt or see accompanying file
// LICENSE_BOOST_1_0.txt)

use std::convert::TryInto;
use std::io::Read;
use std::marker::PhantomData;

use fon::chan::{Ch16, Ch32, Ch64, Ch8};
use fon::Frame;

use crate::pcm::{
    ALaw, F32Be, F32Le, F64Be, F64Le, MuLaw, Pcm, S16Be, S16Le, S24Be, S24Le,
    S32Be, S32Le, U16Be, U16Le, U24Be, U24Le, U32Be, U32Le, S8, U8,
};

const ALAW_DECOMP_TABLE: [i16; 256] = [
    // 0 ~ 127
    -5504, -5248, -6016, -5760, -4480, -4224, -4992, -4736, -7552, -7296, -8064,
    -7808, -6528, -6272, -7040, -6784, -2752, -2624, -3008, -2880, -2240,
    -2112, -2496, -2368, -3776, -3648, -4032, -3904, -3264, -3136, -3520,
    -3392, -22016, -20992, -24064, -23040, -17920, -16896, -19968, -18944,
    -30208, -29184, -32256, -31232, -26112, -25088, -28160, -27136, -11008,
    -10496, -12032, -11520, -8960, -8448, -9984, -9472, -15104, -14592, -16128,
    -15616, -13056, -12544, -14080, -13568, -344, -328, -376, -360, -280, -264,
    -312, -296, -472, -456, -504, -488, -408, -392, -440, -424, -88, -72, -120,
    -104, -24, -8, -56, -40, -216, -200, -248, -232, -152, -136, -184, -168,
    -1376, -1312, -1504, -1440, -1120, -1056, -1248, -1184, -1888, -1824,
    -2016, -1952, -1632, -1568, -1760, -1696, -688, -656, -752, -720, -560,
    -528, -624, -592, -944, -912, -1008, -976, -816, -784, -880, -848,
    // 128(-128) ~ 255(-1)
    5504, 5248, 6016, 5760, 4480, 4224, 4992, 4736, 7552, 7296, 8064, 7808,
    6528, 6272, 7040, 6784, 2752, 2624, 3008, 2880, 2240, 2112, 2496, 2368,
    3776, 3648, 4032, 3904, 3264, 3136, 3520, 3392, 22016, 20992, 24064, 23040,
    17920, 16896, 19968, 18944, 30208, 29184, 32256, 31232, 26112, 25088,
    28160, 27136, 11008, 10496, 12032, 11520, 8960, 8448, 9984, 9472, 15104,
    14592, 16128, 15616, 13056, 12544, 14080, 13568, 344, 328, 376, 360, 280,
    264, 312, 296, 472, 456, 504, 488, 408, 392, 440, 424, 88, 72, 120, 104,
    24, 8, 56, 40, 216, 200, 248, 232, 152, 136, 184, 168, 1376, 1312, 1504,
    1440, 1120, 1056, 1248, 1184, 1888, 1824, 2016, 1952, 1632, 1568, 1760,
    1696, 688, 656, 752, 720, 560, 528, 624, 592, 944, 912, 1008, 976, 816,
    784, 880, 848,
];

const MULAW_DECOMP_TABLE: [i16; 256] = [
    -32124, -31100, -30076, -29052, -28028, -27004, -25980, -24956, -23932,
    -22908, -21884, -20860, -19836, -18812, -17788, -16764, -15996, -15484,
    -14972, -14460, -13948, -13436, -12924, -12412, -11900, -11388, -10876,
    -10364, -9852, -9340, -8828, -8316, -7932, -7676, -7420, -7164, -6908,
    -6652, -6396, -6140, -5884, -5628, -5372, -5116, -4860, -4604, -4348,
    -4092, -3900, -3772, -3644, -3516, -3388, -3260, -3132, -3004, -2876,
    -2748, -2620, -2492, -2364, -2236, -2108, -1980, -1884, -1820, -1756,
    -1692, -1628, -1564, -1500, -1436, -1372, -1308, -1244, -1180, -1116,
    -1052, -988, -924, -876, -844, -812, -780, -748, -716, -684, -652, -620,
    -588, -556, -524, -492, -460, -428, -396, -372, -356, -340, -324, -308,
    -292, -276, -260, -244, -228, -212, -196, -180, -164, -148, -132, -120,
    -112, -104, -96, -88, -80, -72, -64, -56, -48, -40, -32, -24, -16, -8, -1,
    32124, 31100, 30076, 29052, 28028, 27004, 25980, 24956, 23932, 22908,
    21884, 20860, 19836, 18812, 17788, 16764, 15996, 15484, 14972, 14460,
    13948, 13436, 12924, 12412, 11900, 11388, 10876, 10364, 9852, 9340, 8828,
    8316, 7932, 7676, 7420, 7164, 6908, 6652, 6396, 6140, 5884, 5628, 5372,
    5116, 4860, 4604, 4348, 4092, 3900, 3772, 3644, 3516, 3388, 3260, 3132,
    3004, 2876, 2748, 2620, 2492, 2364, 2236, 2108, 1980, 1884, 1820, 1756,
    1692, 1628, 1564, 1500, 1436, 1372, 1308, 1244, 1180, 1116, 1052, 988, 924,
    876, 844, 812, 780, 748, 716, 684, 652, 620, 588, 556, 524, 492, 460, 428,
    396, 372, 356, 340, 324, 308, 292, 276, 260, 244, 228, 212, 196, 180, 164,
    148, 132, 120, 112, 104, 96, 88, 80, 72, 64, 56, 48, 40, 32, 24, 16, 8, 0,
];

/// Decoder for RAW audio
pub struct Decoder<R: Read, F: Frame, P: Pcm>(R, PhantomData<(F, P)>);

impl<R: Read, F: Frame, P: Pcm> Decoder<R, F, P> {
    /// Create a new RAW audio decoder.
    pub fn new<H: Into<f64>>(reader: R, pcm: P) -> Self {
        let _ = pcm;
        Self(reader, PhantomData)
    }

    // FIXME: Is this API needed?
    /*/// Decode the entire file all at once, appending to an audio buffer.
    pub fn decode(&mut self, audio: &mut Audio<F>) -> std::io::Result<()> {
        todo!()
    }

    /// Stream a number of samples from the file into a sink.
    pub fn stream<S: Sink<F>>(
        &mut self,
        sink: &mut S,
    ) -> std::io::Result<usize> {
        todo!()
    }*/
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, U8> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len()]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.iter()) {
            *chan = F::Chan::from(Ch8::new((buffer ^ 0x80) as i8));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, S8> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len()]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.iter()) {
            *chan = F::Chan::from(Ch8::new(*buffer as i8));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, MuLaw> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len()]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.iter()) {
            *chan =
                F::Chan::from(Ch16::new(MULAW_DECOMP_TABLE[*buffer as usize]));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, ALaw> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len()]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.iter()) {
            *chan =
                F::Chan::from(Ch16::new(ALAW_DECOMP_TABLE[*buffer as usize]));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, U16Le> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 2]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(2)) {
            *chan = F::Chan::from(Ch16::new(
                (u16::from_le_bytes(buffer.try_into().unwrap()) ^ 0x8000u16)
                    as i16,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, U16Be> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 2]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(2)) {
            *chan = F::Chan::from(Ch16::new(
                (u16::from_be_bytes(buffer.try_into().unwrap()) ^ 0x8000u16)
                    as i16,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, S16Le> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 2]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(2)) {
            *chan = F::Chan::from(Ch16::new(i16::from_le_bytes(
                buffer.try_into().unwrap(),
            )));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, S16Be> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 2]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(2)) {
            *chan = F::Chan::from(Ch16::new(i16::from_be_bytes(
                buffer.try_into().unwrap(),
            )));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, U24Le> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 3]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(3)) {
            // Sign extend!
            let buffer = [
                buffer[0],
                buffer[1],
                buffer[2] ^ 0x80,
                if buffer[2] & 0x80 == 0 { 0xFF } else { 0x00 },
            ];
            *chan = F::Chan::from(Ch64::new(
                (i32::from_le_bytes(buffer) as f64 + 0.5) / 8388607.5,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, U24Be> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 3]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(3)) {
            // Sign extend!
            let buffer = [
                if buffer[2] & 0x80 == 0 { 0xFF } else { 0x00 },
                buffer[0] ^ 0x80,
                buffer[1],
                buffer[2],
            ];
            *chan = F::Chan::from(Ch64::new(
                (i32::from_le_bytes(buffer) as f64 + 0.5) / 8388607.5,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, S24Le> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 3]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(3)) {
            // Sign extend!
            let buffer = [
                buffer[0],
                buffer[1],
                buffer[2],
                if buffer[2] & 0x80 != 0 { 0xFF } else { 0x00 },
            ];
            *chan = F::Chan::from(Ch64::new(
                (i32::from_le_bytes(buffer) as f64 + 0.5) / 8388607.5,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, S24Be> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 3]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(3)) {
            // Sign extend!
            let buffer = [
                if buffer[2] & 0x80 != 0 { 0xFF } else { 0x00 },
                buffer[0],
                buffer[1],
                buffer[2],
            ];
            *chan = F::Chan::from(Ch64::new(
                (i32::from_le_bytes(buffer) as f64 + 0.5) / 8388607.5,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, U32Le> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 4]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(4)) {
            *chan = F::Chan::from(Ch64::new(
                ((u32::from_le_bytes(buffer.try_into().unwrap()) ^ (1 << 31))
                    as f64
                    + 0.5)
                    / 2147483647.5,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, U32Be> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 4]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(4)) {
            *chan = F::Chan::from(Ch64::new(
                ((u32::from_be_bytes(buffer.try_into().unwrap()) ^ (1 << 31))
                    as f64
                    + 0.5)
                    / 2147483647.5,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, S32Le> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 4]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(2)) {
            *chan = F::Chan::from(Ch64::new(
                (u32::from_le_bytes(buffer.try_into().unwrap()) as f64 + 0.5)
                    / 2147483647.5,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, S32Be> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 4]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(2)) {
            *chan = F::Chan::from(Ch64::new(
                (u32::from_be_bytes(buffer.try_into().unwrap()) as f64 + 0.5)
                    / 2147483647.5,
            ));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, F32Le> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 4]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(4)) {
            *chan = F::Chan::from(Ch32::new(f32::from_le_bytes(
                buffer.try_into().unwrap(),
            )));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, F32Be> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 4]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(4)) {
            *chan = F::Chan::from(Ch32::new(f32::from_be_bytes(
                buffer.try_into().unwrap(),
            )));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, F64Le> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 8]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(8)) {
            *chan = F::Chan::from(Ch64::new(f64::from_le_bytes(
                buffer.try_into().unwrap(),
            )));
        }
        Some(Ok(F::from_channels(channels)))
    }
}

impl<R: Read, F: Frame> Iterator for Decoder<R, F, F64Be> {
    type Item = std::io::Result<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 64];
        let channels = &mut [F::Chan::default(); 8][0..F::CHAN_COUNT];
        if let Err(e) = self.0.read_exact(&mut buffer[..channels.len() * 8]) {
            return Some(Err(e));
        }
        for (chan, buffer) in channels.iter_mut().zip(buffer.chunks(8)) {
            *chan = F::Chan::from(Ch64::new(f64::from_be_bytes(
                buffer.try_into().unwrap(),
            )));
        }
        Some(Ok(F::from_channels(channels)))
    }
}
