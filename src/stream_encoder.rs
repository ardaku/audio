use crate::ffi::*;

/// An Opus Radio Stream Encoder.
pub struct StreamEncoder {
    encoder: *mut OpusEncoder,
    stream: OggStreamState,
    last_sample_pos: i64,
    packet_no: i64,
    begin: bool,
}

impl StreamEncoder {
    pub fn new(/*sample_rate: u32, channels: Channels, application: Application*/)
        -> Self
    {
        // Shouldn't fail.
        let encoder = OpusEncoder::new(48000, 2, false /*not VOIP*/).unwrap();
        let stream = OggStreamState::new().unwrap();

/*        let comments = Comments::new();
        comments.add("ARTIST", "Someone");
        comments.add("TITLE", "Some track");
        let encoder = Encoder::new(
            &comments, // Stream Info
            ,     // Sample Rate
            2,         // # of Channels
            0,         // 0 for mono/stereo, 1 for surround
        ).expect("Error encoding");*/

        Self {
//            comments,
            stream, encoder, last_sample_pos: -1, packet_no: -1, begin: true,
        }
    }

    /// Encode an Opus packet.
    pub fn encode(&mut self, samples: &[i16; 1920 * 2]) -> Option<(&[u8], &[u8])> {
        self.last_sample_pos += 1920;
        self.packet_no += 1;

        let mut packet = [0u8; 4000]; // This is the recommended size by xiph
        let packet_len = OpusEncoder::encode(self.encoder, samples, &mut packet);
        let ret = self.stream.encode(&mut packet[..packet_len], self.last_sample_pos, self.packet_no, self.begin, false /*not the end*/)?;

        self.begin = false;

        Some(ret)
    }
}
