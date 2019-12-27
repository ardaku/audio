use crate::ffi::*;

/// An Ogg Opus Stream Encoder.
pub struct StreamEncoder {
    encoder: *mut OpusEncoder,
    stream: OggStreamState,
    last_sample_pos: i64,
    packet_no: i64,
    begin: bool,
    headers: Vec<u8>,
}

fn ident_header(data: &mut Vec<u8>) {
    // Reference: https://tools.ietf.org/html/rfc7845.html#section-5.1

    // 1. Magic Signature
    data.extend(b"OpusHead");
    // 2. Version - u4.u4
    data.push(1);
    // 3. Output Channel Count (Max = 2:Stereo) - u8
    data.push(2);
    // 4. Pre-skip
    data.extend(&(3_840u16).to_le_bytes());
    // 5. Sample rate of original, uncompressed input
    data.extend(&(48_000u32).to_le_bytes());
    // 6. Output gain
    data.extend(&(0i16).to_le_bytes());
    // 7. Channel Mapping Family (2 channels: stereo (left, right))
    data.push(0);
    // FIXME: Do we add this garbage octet to finish page?
    // data.push(0);
}

fn comment_header(data: &mut Vec<u8>) {
    // Reference: https://tools.ietf.org/html/rfc7845.html#section-5.2

    let vendor = b"rust-opus-no";

    // 1. Magic Signature
    data.extend(b"OpusTags");
    // 2. Vendor String Length
    data.extend(&(vendor.len() as u32).to_le_bytes());
    // 3. Vendor String
    data.extend(vendor);
    // 4. User comment list length.  FIXME: Add user comments.
    data.extend(&(0u32).to_le_bytes());
}

// Generate both the identification header, and comments header.
fn header_packets(data: &mut Vec<u8>) {
    ident_header(data);
    comment_header(data);
}

impl StreamEncoder {
    pub fn new(/*sample_rate: u32, channels: Channels, application: Application*/)
        -> Self
    {
        let mut headers = vec![];

        // Generate headers.
        header_packets(&mut headers);

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
            headers,
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
