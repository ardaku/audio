use crate::ffi::*;

pub struct StreamEncoder {
    comments: Comments,
    encoder: Encoder,
}

impl StreamEncoder {
    pub fn new(/*sample_rate: u32, channels: Channels, application: Application*/)
        -> Self
    {
        let comments = Comments::new();
        comments.add("ARTIST", "Someone");
        comments.add("TITLE", "Some track");
        let encoder = Encoder::new(
            &comments, // Stream Info
            48000,     // Sample Rate
            2,         // # of Channels
            0,         // 0 for mono/stereo, 1 for surround
        ).expect("Error encoding");

        Self {
            comments, encoder,
        }
    }

    pub fn encode(&self, samples: &[(i16, i16)]) -> Option<&[u8]> {
        if samples.is_empty() { return None }
        self.encoder.add(samples, 2);
        self.encoder.get()
    }
}
