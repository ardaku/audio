//! RIFF WAV Chunks.
//!
//! # Key
//! - **Required** - Count must be exactly one.
//! - **Optional** - Count must be exactly one or zero.
//! - **Multiple** - Count can be any number, including zero.
//!
//! # Order
//! The RIFF WAV chunk order must be as follows:
//!  - **Required**: Constant RIFF header (abstracted out).
//!  - **Required**: Constant WAV header (abstracted out).
//!  - **Required**: `Fmt` "fmt "
//!  - **Optional**: `Fact`: "fact" (compression header for non-PCM data)
//!  - **Required**: `Data` "data" (raw data with padding byte if odd length)

mod fmt;

pub use fmt::{Fmt, Format};

/// A chunk in a RIFF WAV file.
pub enum Chunk {
    /// The "fmt " chunk.
    Fmt(Fmt),
}
