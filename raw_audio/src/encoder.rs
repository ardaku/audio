// RAW Audio
// Copyright © 2021 Jeron Aldaron Lau.
//
// Licensed under the Boost Software License, Version 1.0
// (https://www.boost.org/LICENSE_1_0.txt or see accompanying file
// LICENSE_BOOST_1_0.txt)

use std::io::Write;
use std::marker::PhantomData;

use fon::chan::{Ch16, Ch32, Ch8, Channel};
use fon::{Frame, Stream};

use crate::pcm::{
    ALaw, F32Be, F32Le, F64Be, F64Le, MuLaw, Pcm, S16Be, S16Le, S24Be, S24Le,
    S32Be, S32Le, U16Be, U16Le, U24Be, U24Le, U32Be, U32Le, S8, U8,
};

/// Encoder for RAW Audio
pub struct Encoder<W: Write, F: Frame, P: Pcm>(W, PhantomData<(F, P)>);

// 32-bit Linear PCM channel.
fn pcm_chan_32<C: Channel>(chan: C) -> i32 {
    let input = chan.to_f64() * 2147483647.5;

    if input < 0.0 {
        let input = -input;
        let fract = input % 1.0;
        let mut whole = input - fract;
        if fract > f64::EPSILON {
            whole += 1.0;
        }
        (-whole) as i32
    } else {
        input as i32
    }
}

impl<W: Write, F: Frame, P: Pcm> Encoder<W, F, P> {
    /// Create a new raw Audio encoder.
    pub fn new(writer: W, pcm: P) -> Self {
        let _ = pcm;
        Self(writer, PhantomData)
    }
}

impl<W: Write, F: Frame> Encoder<W, F, U8> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch8 = chan.into();
                let chan: i8 = chan.into();
                self.0.write_all(&[chan as u8 ^ 0x80])?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, S8> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch8 = chan.into();
                let chan: i8 = chan.into();
                self.0.write_all(&chan.to_le_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, MuLaw> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch16 = chan.into();
                let chan: i16 = chan.into();
                // reduce to 14 bits.
                let chan: u8 = match chan >> 2 {
                    x if x <= -8159 => 0x00,
                    x if x <= -4064 => ((x + 8159) >> 8) as u8,
                    x if x <= -2016 => 0x10 | ((x + 4063) >> 7) as u8,
                    x if x <= -992 => 0x20 | ((x + 2015) >> 6) as u8,
                    x if x <= -480 => 0x30 | ((x + 991) >> 5) as u8,
                    x if x <= -224 => 0x40 | ((x + 479) >> 4) as u8,
                    x if x <= -96 => 0x50 | ((x + 223) >> 3) as u8,
                    x if x <= -32 => 0x60 | ((x + 95) >> 2) as u8,
                    x if x <= -1 => 0x70 | ((x + 31) >> 1) as u8,
                    x if x <= 30 => 0xF0 | ((30 - x) >> 1) as u8,
                    x if x <= 94 => 0xE0 | ((94 - x) >> 2) as u8,
                    x if x <= 222 => 0xD0 | ((222 - x) >> 3) as u8,
                    x if x <= 478 => 0xC0 | ((478 - x) >> 4) as u8,
                    x if x <= 990 => 0xB0 | ((990 - x) >> 5) as u8,
                    x if x <= 2014 => 0xA0 | ((2014 - x) >> 6) as u8,
                    x if x <= 4062 => 0x90 | ((4062 - x) >> 7) as u8,
                    x if x <= 8158 => 0x80 | ((8158 - x) >> 8) as u8,
                    _ => 0x80,
                };
                self.0.write_all(&[chan])?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, ALaw> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch16 = chan.into();
                let chan: i16 = chan.into();

                const C_CLIP: i16 = 32635;
                const LOG_TABLE: [u8; 128] = [
                    1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5,
                    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6,
                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                    6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                    7, 7, 7, 7, 7, 7, 7, 7,
                ];

                let mut chan = if chan == -32768 { -32767 } else { chan };
                let sign = ((!chan) >> 8) as u8 & 0x80;
                if sign == 0 {
                    chan = -chan;
                }
                if chan > C_CLIP {
                    chan = C_CLIP;
                }
                let mut chan: u8 = if chan >= 256 {
                    let exponent = LOG_TABLE[((chan >> 8) & 0x7F) as usize];
                    let mantissa = ((chan >> (exponent + 3)) & 0x0F) as u8;
                    (exponent << 4) | mantissa
                } else {
                    (chan >> 4) as u8
                };
                chan ^= sign ^ 0x55;

                self.0.write_all(&[chan])?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, U16Le> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch16 = chan.into();
                let chan: i16 = chan.into();
                self.0.write_all(&(chan ^ 0x8000u16 as i16).to_le_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, U16Be> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch16 = chan.into();
                let chan: i16 = chan.into();
                self.0.write_all(&(chan ^ 0x8000u16 as i16).to_be_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, S16Le> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch16 = chan.into();
                let chan: i16 = chan.into();
                self.0.write_all(&chan.to_le_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, S16Be> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch16 = chan.into();
                let chan: i16 = chan.into();
                self.0.write_all(&chan.to_be_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, U24Le> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                let chan = pcm_chan_32(*chan).to_le_bytes();
                self.0.write_all(&[chan[0], chan[1], chan[2] ^ 0x80])?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, U24Be> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                let chan = pcm_chan_32(*chan).to_be_bytes();
                self.0.write_all(&[chan[1] ^ 0x80, chan[2], chan[3]])?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, S24Le> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                let chan = pcm_chan_32(*chan).to_le_bytes();
                self.0.write_all(&[chan[0], chan[1], chan[2]])?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, S24Be> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                let chan = pcm_chan_32(*chan).to_be_bytes();
                self.0.write_all(&[chan[1], chan[2], chan[3]])?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, U32Le> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                let chan = pcm_chan_32(*chan) ^ (1 << 31);
                self.0.write_all(&chan.to_le_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, U32Be> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                let chan = pcm_chan_32(*chan) ^ (1 << 31);
                self.0.write_all(&chan.to_be_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, S32Le> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                let chan = pcm_chan_32(*chan);
                self.0.write_all(&chan.to_le_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, S32Be> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                let chan = pcm_chan_32(*chan);
                self.0.write_all(&chan.to_be_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, F32Le> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch32 = chan.into();
                let chan: f32 = chan.into();
                self.0.write_all(&chan.to_le_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, F32Be> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels().iter().cloned() {
                let chan: Ch32 = chan.into();
                let chan: f32 = chan.into();
                self.0.write_all(&chan.to_be_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, F64Le> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                self.0.write_all(&chan.to_f64().to_le_bytes())?;
            }
        }
        Ok(())
    }
}

impl<W: Write, F: Frame> Encoder<W, F, F64Be> {
    /// Append encoded data from a stream to the output.  This can be called
    /// multiple times to encode as needed instead of all at once.
    pub fn encode<S: Stream<F>>(&mut self, stream: S) -> std::io::Result<()> {
        assert!(stream.len().is_some());
        for frame in stream.into_iter() {
            for chan in frame.channels() {
                self.0.write_all(&chan.to_f64().to_be_bytes())?;
            }
        }
        Ok(())
    }
}
