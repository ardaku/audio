// RIFF WAV
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under the Boost Software License, Version 1.0
// (https://www.boost.org/LICENSE_1_0.txt or see accompanying file
// LICENSE_BOOST_1_0.txt)

use std::{convert::TryInto, io::Read};

use crate::{Error, Result};
use crate::chunk::Chunk;

/// WAV File Decoder.
pub struct Decoder<R: Read> {
    // The WAV file source.
    reader: R,
    // The size of the remaining data.
    size: u32,
    // The name of the next chunk.
    chunk_name: [u8; 4],
    // The size of the next chunk.
    chunk_size: u32,
}

impl<R: Read> Decoder<R> {
    /// Create a new WAV decoder. Returns `Err` if it's not a WAV file.
    pub fn new(mut reader: R) -> Result<Self> {
        // Read first 12 bytes (RIFF Header)
        let buf = &mut [0u8; 20];
        reader.read_exact(buf).map_err(Error::Io)?;

        // Check for the RIFF signature.
        if &buf[0..4] != b"RIFF" {
            return Err(Error::NotRiff);
        }

        // Get the RIFF Chunk Size minus 4 bytes for the WAVE signature.
        let size = u32::from_le_bytes(buf[4..8].try_into().unwrap()) - 4;

        // Check for the WAVE signature.
        if &buf[8..12] != b"WAVE" {
            return Err(Error::NotRiff);
        }

        // Look ahead to figure out how many bytes to read next.
        let chunk_name = buf[12..16].try_into().unwrap();
        let chunk_size = u32::from_le_bytes(buf[16..20].try_into().unwrap());

        Ok(Self {
            reader,
            size,
            chunk_name,
            chunk_size,
        })
    }

    /// Convert into a `Chunk` iterator.
    pub fn into_chunks(self) -> Chunks<R> {
        Chunks::new(self)
    }

    /// Convert into an audio `Stream`.
    pub fn into_stream(self) -> Stream<R> {
        Stream::new(self.into_chunks())
    }
}

pub struct Chunks<R: Read> {
    decoder: Decoder<R>,
}

impl<R: Read> Chunks<R> {
    pub fn new(decoder: Decoder<R>) -> Self {
        Self {
            decoder
        }
    }
}

impl<R: Read> Iterator for Chunks<R> {
    type Item = Result<Chunk>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.decoder.chunk_name {
            b"\0\0\0\0" => None,
            b"fmt " => {
                let buf = &mut [0; 40];
                if let Err(e) = self.decoder.reader.read_exact(buf) {
                    return Some(Err(Error::Io(e)));
                }
                Some(Ok(Chunk::Fmt(Fmt(
                    self.decoder.chunk_size,
                    *buf,
                ))))
            },
            a => Some(Err(Error::Chunk(*a))),
        }
    }
}

pub struct Stream<R: Read> {
    chunks: Chunks<R>,
}

impl<R: Read> Stream<R> {
    pub fn new(chunks: Chunks<R>) -> Self {
        Self {
            chunks,
        }
    }
}
