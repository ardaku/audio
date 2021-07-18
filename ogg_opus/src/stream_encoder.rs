use crate::ffi::*;

unsafe impl Send for StreamEncoder {}

/// An Ogg Opus Stream Encoder.
pub struct StreamEncoder {
    encoder: *mut OpusEncoder,
    stream: OggStreamState,
    last_sample_pos: i64,
    packet_no: i64,
    begin: bool,
    headers: Vec<u8>,
    temp: Vec<u8>,
}

impl StreamEncoder {
    // Generate the identification header.
    fn ident_header(&mut self) {
        // Reference: https://tools.ietf.org/html/rfc7845.html#section-5.1

        // Clear temporary byte buffer.
        self.temp.clear();
        // 1. Magic Signature
        self.temp.extend(b"OpusHead");
        // 2. Version - u4.u4
        self.temp.push(1);
        // 3. Output Channel Count (Max = 2:Stereo) - u8
        self.temp.push(2);
        // 4. Pre-skip
        self.temp.extend(&(3_840u16).to_le_bytes());
        // 5. Sample rate of original, uncompressed input
        self.temp.extend(&(48_000u32).to_le_bytes());
        // 6. Output gain
        self.temp.extend(&(0i16).to_le_bytes());
        // 7. Channel Mapping Family (2 channels: stereo (left, right))
        self.temp.push(0);
        // FIXME: Do we add this garbage octet to finish page?  Probably not.
        // self.temp.push(0);

        self.packet_no += 1;

        self.stream
            .encode(&mut self.temp, 0, self.packet_no, false, false);
        let (head, body) = self.stream.drain().unwrap();

        self.headers.extend(head);
        self.headers.extend(body);
    }

    // Generate the comments header.
    fn comment_header(&mut self) {
        // Reference: https://tools.ietf.org/html/rfc7845.html#section-5.2

        let vendor = b"rust-ogg_opus";

        // Clear temporary byte buffer.
        self.temp.clear();
        // 1. Magic Signature
        self.temp.extend(b"OpusTags");
        // 2. Vendor String Length
        self.temp.extend(&(vendor.len() as u32).to_le_bytes());
        // 3. Vendor String
        self.temp.extend(vendor);
        // 4. User comment list length.  FIXME: Add user comments.
        self.temp.extend(&(0u32).to_le_bytes());

        self.packet_no += 1;

        self.stream
            .encode(&mut self.temp, 0, self.packet_no, false, false);
        while let Some((head, body)) = self.stream.drain() {
            self.headers.extend(head);
            self.headers.extend(body);
        }
    }

    // Generate both the identification header, and comments header.
    fn header_packets(&mut self) {
        self.ident_header();
        self.comment_header();
    }

    /// Create a new stream encoder.
    #[allow(clippy::new_without_default)]
    pub fn new(/*sample_rate: u32, channels: Channels, application: Application*/) -> Self {
        let temp = vec![];
        let headers = vec![];

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

        let mut this = Self {
            temp,
            headers,
            //            comments,
            stream,
            encoder,
            last_sample_pos: -1,
            packet_no: -1,
            begin: true,
        };

        // Generate header packets.
        this.header_packets();

        this
    }

    /// Encode an Opus packet into a page (get with `while let` on `.page()`).
    pub fn encode(&mut self, samples: &[i16; 1920 * 2]) {
        self.last_sample_pos += 1920;
        self.packet_no += 1;

        let mut packet = [0u8; 4000]; // This is the recommended size by xiph
        let packet_len = OpusEncoder::encode(self.encoder, samples, &mut packet);
        self.stream.encode(
            &mut packet[..packet_len],
            self.last_sample_pos,
            self.packet_no,
            self.begin,
            false, /*not the end*/
        );

        self.begin = false;
    }

    /// Get generated header
    pub fn head(&self) -> &[u8] {
        &self.headers
    }

    /// Drain opus pages
    pub fn page(&mut self) -> Option<(&[u8], &[u8])> {
        let ret = self.stream.drain()?;

        Some(ret)
    }
}
