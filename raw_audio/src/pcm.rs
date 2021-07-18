// RAW Audio
// Copyright © 2021 Jeron Aldaron Lau.
//
// Licensed under the Boost Software License, Version 1.0
// (https://www.boost.org/LICENSE_1_0.txt or see accompanying file
// LICENSE_BOOST_1_0.txt)

//! Supported Audio formats by this crate.

mod seal {
    pub trait Seal {}
}

/// A PCM Format
pub trait Pcm: seal::Seal {}

/// Unsigned 8-bit PCM
pub struct U8;
/// Signed 8-bit PCM
pub struct S8;
/// Signed 8-bit µ-Law Companded PCM
pub struct MuLaw;
/// Signed 8-bit A-Law Companded PCM
pub struct ALaw;

/// Unsigned 16-bit PCM Little Endian
pub struct U16Le;
/// Unsigned 16-bit PCM Big Endian
pub struct U16Be;
/// Signed 16-bit PCM Little Endian
pub struct S16Le;
/// Signed 16-bit PCM Big Endian
pub struct S16Be;

/// Unsigned 24-bit PCM Little Endian
pub struct U24Le;
/// Unsigned 24-bit PCM Big Endian
pub struct U24Be;
/// Signed 24-bit PCM Little Endian
pub struct S24Le;
/// Signed 24-bit PCM Big Endian
pub struct S24Be;

/// Unsigned 32-bit PCM Little Endian
pub struct U32Le;
/// Unsigned 32-bit PCM Big Endian
pub struct U32Be;
/// Signed 32-bit PCM Little Endian
pub struct S32Le;
/// Signed 32-bit PCM Big Endian
pub struct S32Be;

/// 32-bit Floating Point PCM Little Endian
pub struct F32Le;
/// 32-bit Floating Point PCM Big Endian
pub struct F32Be;
/// 64-bit Floating Point PCM Little Endian
pub struct F64Le;
/// 64-bit Floating Point PCM Big Endian
pub struct F64Be;

impl seal::Seal for U8 {}
impl Pcm for U8 {}
impl seal::Seal for S8 {}
impl Pcm for S8 {}
impl seal::Seal for MuLaw {}
impl Pcm for MuLaw {}
impl seal::Seal for ALaw {}
impl Pcm for ALaw {}

impl seal::Seal for U16Le {}
impl Pcm for U16Le {}
impl seal::Seal for U16Be {}
impl Pcm for U16Be {}
impl seal::Seal for S16Le {}
impl Pcm for S16Le {}
impl seal::Seal for S16Be {}
impl Pcm for S16Be {}

impl seal::Seal for U24Le {}
impl Pcm for U24Le {}
impl seal::Seal for U24Be {}
impl Pcm for U24Be {}
impl seal::Seal for S24Le {}
impl Pcm for S24Le {}
impl seal::Seal for S24Be {}
impl Pcm for S24Be {}

impl seal::Seal for U32Le {}
impl Pcm for U32Le {}
impl seal::Seal for U32Be {}
impl Pcm for U32Be {}
impl seal::Seal for S32Le {}
impl Pcm for S32Le {}
impl seal::Seal for S32Be {}
impl Pcm for S32Be {}

impl seal::Seal for F32Le {}
impl Pcm for F32Le {}
impl seal::Seal for F32Be {}
impl Pcm for F32Be {}
impl seal::Seal for F64Le {}
impl Pcm for F64Le {}
impl seal::Seal for F64Be {}
impl Pcm for F64Be {}
