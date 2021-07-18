// RIFF WAV
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under the Boost Software License, Version 1.0
// (https://www.boost.org/LICENSE_1_0.txt or see accompanying file
// LICENSE_BOOST_1_0.txt)

// Reference: http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html

use fon::{chan::Ch16, stereo::Stereo16, Audio, Frame};
use std::convert::TryInto;
use std::{fs, io, mem::size_of};

pub mod chunk;

mod decoder;
mod encoder;

pub use decoder::Decoder;
pub use encoder::Encoder;

/// RIFF WAV Decoder Result Type.
pub type Result<T> = std::result::Result<T, Error>;

/// Decoder Error.
pub enum Error {
    /// The file is missing the RIFF Header.
    NotRiff,
    /// The file is a RIFF file, but not a WAV file.
    NotWav,
    /// The "fmt" chunk is missing from the file.
    FmtMissing,
    /// The size of the "fmt" chunk is invalid.
    FmtSize,
    /// The format is invalid.
    Format,
    /// Subformat contains invalid data.
    Subformat,
    /// Unknown chunk.
    Chunk([u8; 4]),
    /// An I/O Error
    Io(std::io::Error),
}

/// Write a 16-bit PCM WAV file
pub fn write<F: Frame>(audio: Audio<F>, filename: &str) -> io::Result<()>
where
    Ch16: From<F::Chan>,
{
    let audio =
        Audio::<Stereo16>::with_stream(audio.sample_rate().floor(), &audio);
    let mut buf = vec![];
    write_header(&mut buf, &audio);
    write_fmt_header(&mut buf, &audio);
    write_audio_data(&mut buf, &audio);
    fs::write(filename, buf)
}

fn write_header(buf: &mut Vec<u8>, audio: &Audio<Stereo16>) {
    // Predict size of WAV subchunks.
    let n: u32 = audio.len().try_into().unwrap();
    // RIFF Chunk: ckID
    buf.extend(b"RIFF");
    // RIFF Chunk: cksize
    buf.extend(&(36u32 + n).to_le_bytes());
    // RIFF Chunk: WAVEID
    buf.extend(b"WAVE");
}

fn write_fmt_header(buf: &mut Vec<u8>, audio: &Audio<Stereo16>) {
    // RIFF Subchunk: "fmt "
    buf.extend(b"fmt ");
    // Chunk size: 16, 18 or 40
    buf.extend(&(16u32).to_le_bytes());
    // 0: WAVE_FORMAT_PCM
    buf.extend(&(0x0001u16).to_le_bytes());
    // 2: Stereo
    buf.extend(&(2u16).to_le_bytes());
    // 4: Sampling Rate
    buf.extend(&(audio.sample_rate() as u32).to_le_bytes());
    // 8: Bytes per second (i16 * 2 * sample rate)
    buf.extend(&(4 * audio.sample_rate() as u32).to_le_bytes());
    // 12. Data block size (bytes: i16 * 2)
    buf.extend(&(size_of::<u16>() as u16 * 2u16).to_le_bytes());
    // 14. Bits per sample
    buf.extend(&(16u16).to_le_bytes());
}

fn write_audio_data(buf: &mut Vec<u8>, audio: &Audio<Stereo16>) {
    // RIFF Subchunk: "data"
    buf.extend(b"data");
    // cksize (Bytes): Stereo (2) * i16 (2) * Frame Length
    buf.extend(&(4 * audio.len() as u32).to_le_bytes());
    // Sampled data
    for sample in audio {
        for channel in sample.channels().iter().cloned() {
            let channel: i16 = channel.into();
            buf.extend(&channel.to_le_bytes());
        }
    }
}
