// RIFF WAV
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under the Boost Software License, Version 1.0
// (https://www.boost.org/LICENSE_1_0.txt or see accompanying file
// LICENSE_BOOST_1_0.txt)

use std::io::Write;

/// WAV File Encoder.
pub struct Encoder<W: Write> {
    writer: W,
}

impl<W: Write> Encoder<W> {
    ///
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}
