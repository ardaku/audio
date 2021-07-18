// RIFF WAV
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under the Boost Software License, Version 1.0
// (https://www.boost.org/LICENSE_1_0.txt or see accompanying file
// LICENSE_BOOST_1_0.txt)

use crate::{Error, Result};

/// The "fmt " chunk.
#[repr(align(4))] // Align at 4 bytes (maximum size of aligned read)
pub struct Fmt(u32, [u8; 40]);

/// Standard WAV audio formats.
#[repr(u8)]
pub enum Format {
    /// PCM: The samples are stored as integers.
    Pcm,
    /// The samples are stored as IEEE floating-point numbers.
    Float,
    /// 8-bit ITU-T G.711 A-law
    ALaw,
    /// 8-bit ITU-T G.711 µ-law
    MuLaw,
}

impl Fmt {
    /// Get the Audio Format
    #[inline(always)]
    pub fn format(&self) -> Result<Option<Format>> {
        Ok(match self.1[0..2] {
            [0x00, 0x01] => Some(Format::Pcm),
            [0x00, 0x03] => Some(Format::Float),
            [0x00, 0x06] => Some(Format::ALaw),
            [0x00, 0x07] => Some(Format::MuLaw),
            // Extensible: Determined by SubFormat
            [0xFF, 0xFE] => None,
            // Unknown
            _ => return Err(Error::Format)
        })
    }

    /// Get the channel count.
    #[inline(always)]
    pub fn channels(&self) -> u16 {
        u16::from_le_bytes(self.1[2..4].try_into().unwrap())
    }

    #[inline(always)]
    /// Get the sample rate (blocks/frames) per second.
    pub fn rate(&self) -> u32 {
        u32::from_le_bytes(self.1[4..8].try_into().unwrap())
    }

    #[inline(always)]
    /// Get the number of bytes per second (`rate * frame`)
    pub fn bytes_per_sec(&self) -> u32 {
        u32::from_le_bytes(self.1[8..12].try_into().unwrap())
    }

    #[inline(always)]
    /// Get the frame size in bytes.
    pub fn frame(&self) -> u16 {
        u16::from_le_bytes(self.1[12..14].try_into().unwrap())
    }

    /// Get the bit depth (`frame / channels`).
    #[inline(always)]
    pub fn bit_depth(&self) -> u16 {
        u16::from_le_bytes(self.1[14..16].try_into().unwrap())
    }
    
    // Start optional fields.

    /// Get the size of the extension.
    #[inline(always)]
    pub fn ext_size(&self) -> Option<u16> {
        if self.0 > 16 {
            Some(u16::from_le_bytes(self.1[16..18].try_into().unwrap()))
        } else {
            None
        }
    }

    /// Number of valid bits per sample.
    #[inline(always)]
    pub fn valid_bits(&self) -> Option<u16> {
        if self.0 == 40 {
            Some(u16::from_le_bytes(self.1[18..20].try_into().unwrap()))
        } else {
            None
        }
    }

    /// Speaker position mask.
    #[inline(always)]
    pub fn speaker_position(&self) -> Option<u32> {
        if self.0 == 40 {
            Some(u32::from_le_bytes(self.1[20..24].try_into().unwrap()))
        } else {
            None
        }
    }
    
    /// Subformat GUID.
    #[inline(always)]
    pub fn subformat(&self) -> Result<Option<[u8; 2]>> {
        if self.0 == 40 {
            let guid: [u8; 16] = self.1[24..40].try_into().unwrap();
            if &guid[2..14] != b"\x00\x00\x00\x00\x10\x00\x80\x00\x00\xAA\x00\x38\x9B\x71" {
                return Err(Error::Subformat);
            }
            Ok(Some(guid[0..1].try_into().unwrap()))
        } else {
            Ok(None)
        }
    }
}
