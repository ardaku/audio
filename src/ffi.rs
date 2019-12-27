//! Safe bindings to FFI.

use std::ffi::{c_void, CString};
use std::convert::TryInto;

extern "C" {
    // Saving OggOpus files.
    fn ope_comments_create() -> *mut c_void;
    fn ope_comments_add(
        comments: *mut c_void,
		tag: *const c_void, // TEXT
		val: *const c_void, // TEXT
	) -> i32;
    fn ope_encoder_create_pull(
        comments: *mut c_void,
		rate: i32,
		channels: i32,
		family: i32,
		error: *mut i32,
	) -> *mut c_void;
    fn ope_comments_destroy(comments: *mut c_void) -> ();
    fn ope_encoder_drain(enc: *mut c_void) -> i32;
    fn ope_encoder_destroy(enc: *mut c_void) -> ();
    fn ope_encoder_write(
        enc: *mut c_void,
		pcm: *const i16,
		samples_per_channel: i32
	) -> i32;
    fn ope_encoder_get_page(
     	enc: *mut c_void,
		page: *mut *mut u8,
		len: *mut i32,
		flush: i32,
	) -> i32;

    // OggOpus Internet Radio Streaming
    fn opus_encoder_create(a: i32, b: i32, c: i32, d: *mut i32) -> *mut OpusEncoder;
/*    fn opus_encoder_init(
     	st: *mut OpusEncoder,
		fs: i32,
		channels: i32,
		application: i32,
	) -> i32;*/
    fn opus_encode(
     	st: *mut OpusEncoder,
		pcm: *const i16,
		frame_size: i32,
		data: *mut u8,
		max_data_bytes: i32,
	) -> i32;
    fn ogg_stream_init(os: *mut OggStreamState, serialno: i32) -> i32;
    fn ogg_stream_packetin(os: *mut OggStreamState, op: *mut OggPacket) -> i32;
    fn ogg_stream_pageout(os: *mut OggStreamState, og: *mut OggPage) -> i32;
}

const NB_FRAMES: usize = 8;
const NB_TBANDS: usize = 18;
const ANALYSIS_BUF_SIZE: usize = 720; /* 30 ms at 24 kHz */
/* At that point we can stop counting frames because it no longer matters. */
const ANALYSIS_COUNT_MAX: usize = 10000;
const DETECT_SIZE: usize = 100;
const MAX_NEURONS: usize = 32;
const LEAK_BANDS: usize = 19;

#[repr(C)]
pub struct OggStreamState {
  body_data: *mut u8,    /* bytes from packet bodies */
  body_storage: isize,   /* storage elements allocated */
  body_fill: isize,      /* elements stored; fill mark */
  body_returned: isize,  /* elements of fill returned */

  lacing_vals: *mut i32, /* The values that will go to the segment table */
  granule_vals: *mut i64,/* granulepos values for headers. Not compact
                            this way, but it is simple coupled to the
                            lacing fifo */
  lacing_storage: isize,
  lacing_fill: isize,
  lacing_packet: isize,
  lacing_returned: isize,

  header: [u8; 282],     /* working space for header encode */
  header_fill: i32,

  e_o_s: i32,          /* set when we have buffered the last packet in the
                          logical bitstream */
  b_o_s: i32,          /* set after we've written the initial page
                          of a logical bitstream */
  serialno: isize,
  pageno: i32,
  packetno: i64,      /* sequence number for decode; the framing
                             knows where there's a hole in the data,
                             but we need coupling so that the codec
                             (which is in a seperate abstraction
                             layer) also knows about the gap */
  granulepos: i64,
}

#[repr(C)]
struct OggPacket {
  packet: *mut u8,
  bytes: isize,
  b_o_s: isize,
  e_o_s: isize,

  granulepos: i64,
  packetno: i64,
}

#[repr(C)]
struct OggPage {
    pub header: *mut u8,
    pub header_len: isize,
    pub body: *mut u8,
    pub body_len: isize,
}

impl OggStreamState {
    /// Create a new Ogg Stream container.
    pub fn new() -> Result<Self, ()> {
        let mut ogg_stream_state = std::mem::MaybeUninit::uninit();

        unsafe {
            let e = ogg_stream_init(ogg_stream_state.as_mut_ptr(), 0);
            if e != 0 {
                Err(())
            } else {
                Ok(ogg_stream_state.assume_init())
            }
        }
    }

    /// Encode bitstream with Container.
    pub fn encode(&mut self, data: &mut [u8], last_sample_pos: i64, packet_no: i64, begin: bool, end: bool) {
        // Create The OggPacket from Opus data.
        let mut packet = OggPacket {
            packet: data.as_mut_ptr(),
            bytes: data.len() as isize,
            b_o_s: if begin { 1 } else { 0 },
            e_o_s: if end { 1 } else { 0 },
            granulepos: last_sample_pos,
            packetno: packet_no,
        };

        // Add OggPacket to OggStreamState
        unsafe {
            if ogg_stream_packetin(self, &mut packet) != 0 {
                panic!("Failed encoding OggPacket into OggStreamState");
            }
        }
    }

    pub fn drain(&mut self) -> Option<(&[u8], &[u8])> {
        // Create an Ogg Page if enough data is available.
        let mut page = std::mem::MaybeUninit::uninit();
        unsafe {
            if ogg_stream_pageout(self, page.as_mut_ptr()) == 0 {
                None
            } else {
                let page = page.assume_init();
                let head = std::slice::from_raw_parts(page.header, page.header_len as usize);
                let body = std::slice::from_raw_parts(page.body, page.body_len as usize);
                Some((head, body))
            }
        }
    }
}

#[repr(C)]
struct AnalysisInfo {
   valid: i32,
   tonality: f32,
   tonality_slope: f32,
   noisiness: f32,
   activity: f32,
   music_prob: f32,
   music_prob_min: f32,
   music_prob_max: f32,
   bandwidth: i32,
   activity_probability: f32,
   max_pitch_ratio: f32,
   /* Store as Q6 char to save space. */
   leak_boost: [u8; LEAK_BANDS],
}

#[repr(C)]
struct TonalityAnalysisState {
   arch: i32,
   application: i32,
   fs: i32,
   angle: [f32; 240],
   d_angle: [f32; 240],
   d2_angle: [f32; 240],
   inmem: [i32; ANALYSIS_BUF_SIZE],
   mem_fill: i32, /* number of usable samples in the buffer */
   prev_band_tonality: [f32; NB_TBANDS],
   prev_tonality: f32,
   prev_bandwidth: i32,
   e: [[f32;NB_FRAMES];NB_TBANDS],
   log_e: [[f32;NB_FRAMES];NB_TBANDS],
   low_e: [f32;NB_TBANDS],
   high_e: [f32;NB_TBANDS],
   mean_e: [f32;NB_TBANDS+1],
   mem: [f32;32],
   cmean: [f32; 8],
   std: [f32; 9],
   e_tracker: f32,
   low_e_count: f32,
   e_count: i32,
   count: i32,
   analysis_offset: i32,
   write_pos: i32,
   read_pos: i32,
   read_subframe: i32,
   hp_ener_accum: f32,
   initialized: i32,
   rnn_state: [f32; MAX_NEURONS],
   downmix_state: [i32; 3],
   info: [AnalysisInfo; DETECT_SIZE],
}

// Structure for controlling encoder operation
#[repr(C)]
struct SilkEncControlStruct {
    // I: Number of channels; 1/2
    nChannelsAPI: i32,
    // I: Number of channels; 1/2
    nChannelsInternal: i32,
    // I: Input signal sampling rate in Hertz; 8000/12000/16000/24000/32000/44100/48000
    api_sample_rate: i32,
    // I: Maximum internal sampling rate in Hertz; 8000/12000/16000
    maxInternalSampleRate: i32,
    // I: Minimum internal sampling rate in Hertz; 8000/12000/16000
    minInternalSampleRate: i32,
    // I: Soft request for internal sampling rate in Hertz; 8000/12000/16000
    desiredInternalSampleRate: i32,
    // I: Number of samples per packet in milliseconds; 10/20/40/60
    payloadSize_ms: i32,
    // I: Bitrate during active speech in bits/second; internally limited
    bitRate: i32,
    // I: Uplink packet loss in percent (0-100)
    packetLossPercentage: i32,
    // I: Complexity mode; 0 is lowest, 10 is highest complexity
    complexity: i32,
    // I: Flag to enable in-band Forward Error Correction (FEC); 0/1
    useInBandFEC: i32,
    // I: Flag to actually code in-band Forward Error Correction (FEC) in the current packet; 0/1
    LBRR_coded: i32,
    // I: Flag to enable discontinuous transmission (DTX); 0/1
    useDTX: i32,
    // I: Flag to use constant bitrate
    useCBR: i32,
    // I: Maximum number of bits allowed for the frame
    maxBits: i32,
    // I: Causes a smooth downmix to mono
    toMono: i32,
    // I: Opus encoder is allowing us to switch bandwidth
    opusCanSwitch: i32,
    // I: Make frames as independent as possible (but still use LPC)
    reducedDependency: i32,
    // O: Internal sampling rate used, in Hertz; 8000/12000/16000
    internalSampleRate: i32,
    // O: Flag that bandwidth switching is allowed (because low voice activity)
    allowBandwidthSwitch: i32,
    // O: Flag that SILK runs in WB mode without variable LP filter (use for switching between WB/SWB/FB)
    inWBmodeWithoutVariableLP: i32,
    // O: Stereo width
    stereoWidth_Q14: i32,
    // O: Tells the Opus encoder we're ready to switch
    switchReady: i32,
    // O: SILK Signal type
    signalType: i32,
    // O: SILK offset (dithering)
    offset: i32,
}

const MAX_ENCODER_BUFFER: usize = 480;

#[repr(C)]
struct StereoWidthState {
    xx: i32,
    xy: i32,
    yy: i32,
    smoothed_width: i16,
    max_follower: i16,
}

/// Encoder for an OggOpus Internet Radio Stream.
#[repr(C)]
pub struct OpusEncoder {
    celt_enc_offset: i32,
    silk_enc_offset: i32,
    silk_mode: SilkEncControlStruct,
    application: i32,
    channels: i32,
    delay_compensation: i32,
    force_channels: i32,
    signal_type: i32,
    user_bandwidth: i32,
    max_bandwidth: i32,
    user_forced_mode: i32,
    voice_ratio: i32,
    fs: i32,
    use_vbr: i32,
    vbr_constraint: i32,
    variable_duration: i32,
    bitrate_bps: i32,
    user_bitrate_bps: i32,
    lsb_depth: i32,
    encoder_buffer: i32,
    lfe: i32,
    arch: i32,
    // general DTX for both SILK and CELT
    use_dtx: i32,
    analysis: TonalityAnalysisState,
    stream_channels: i32,
    hybrid_stereo_width_Q14: i16,
    variable_HP_smth2_Q15: i32,
    prev_HB_gain: i16,
    hp_mem: [i32; 4],
    mode: i32,
    prev_mode: i32,
    prev_channels: i32,
    prev_framesize: i32,
    bandwidth: i32,
    // Bandwidth determined automatically from the rate (before any other adjustment)
    auto_bandwidth: i32,
    silk_bw_switch: i32,
    // Sampling rate (at the API level)
    first: i32,
    energy_masking: *mut i16,
    width_mem: StereoWidthState,
    delay_buffer: [i16; MAX_ENCODER_BUFFER*2],
    detected_bandwidth: i32,
    nb_no_activity_frames: i32,
    peak_signal_energy: i32,
    nonfinal_frame: i32, /* current frame is not the final in a packet */
    rangeFinal: u32,
}

impl OpusEncoder {
    /// Create a new OggOpus Internet Radio Stream.
    /// * `hz`: The sample rate.
    /// * `channels`: 1 for mono, 2 for stereo.
    /// * `voip`: Whether or not to go for reducing latency
    /// (true: voice chat, false: music stream).
    pub fn new(hz: i32, channels: i32, voip: bool) -> Result<*mut Self, i32> {
//        let mut opus_encoder = std::mem::MaybeUninit::uninit();
        let mut e = std::mem::MaybeUninit::uninit();

        unsafe {
            let opus_encoder = opus_encoder_create(hz, channels,
                if voip { 2048 } else { 2049 }, e.as_mut_ptr()
	        );
            let e = e.assume_init();
            if e != 0 {
                Err(e)
            } else {
                Ok(opus_encoder)
            }
        }
    }

    /// Encode raw audio data as Opus.
    pub fn encode(self2: *mut Self, raw: &[i16; 1920 * 2], outbuf: &mut [u8])
        -> usize
    {
        unsafe {
            let ret = opus_encode(
                self2, // encoder state
			    raw.as_ptr(), // input
                1920, // One of the allowed frame sizes (samples per channel).
			    outbuf.as_mut_ptr(),
                outbuf.len().try_into().unwrap(),
	        );
            if ret >= 0 {
                ret as usize
            } else {
                panic!("Encoding error!");
            }
        }
    }
}

pub struct Comments(*mut c_void);

impl Comments {
    pub fn new() -> Self {
        Self(unsafe{ope_comments_create()})
    }

    pub fn add(&self, tag: &str, val: &str) -> i32 {
        let tag = CString::new(tag).expect("CString::new failed");
        let val = CString::new(val).expect("CString::new failed");

        unsafe {
            ope_comments_add(self.0, tag.as_ptr() as *const _, val.as_ptr() as *const _)
        }
    }
}

impl Drop for Comments {
    fn drop(&mut self) {
        unsafe {
            ope_comments_destroy(self.0)
        }
    }
}

pub struct Encoder(*mut c_void);

impl Encoder {
    pub fn new(comments: &Comments, rate: i32, channels: i32, family: i32)
        -> Result<Self, i32>
    {
        let mut error = std::mem::MaybeUninit::uninit();
        let enc = unsafe {
            ope_encoder_create_pull(
                comments.0,
                rate,
                channels,
                family,
                error.as_mut_ptr(),
            )
        };

        if enc.is_null() {
            unsafe {
                Err(error.assume_init())
            }
        } else {
            Ok(Self(enc))
        }
    }

    pub fn add(&self, samples: &[(i16, i16)], channels: i32) {
        if samples.is_empty() { return }
        unsafe {
            ope_encoder_write(self.0, &samples[0].0, samples.len() as i32 / channels);
        }
    }

    pub fn get(&self) -> Option<&[u8]> {
        let mut page_data = std::mem::MaybeUninit::uninit();
        let mut page_size = std::mem::MaybeUninit::uninit();

        unsafe {
            let page_available = ope_encoder_get_page(self.0, page_data.as_mut_ptr(), page_size.as_mut_ptr(), 0);
            if page_available != 0 {
                Some(std::slice::from_raw_parts(page_data.assume_init(), page_size.assume_init() as usize))
            } else {
                None
            }
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            ope_encoder_drain(self.0);
            ope_encoder_destroy(self.0);
        }
    }
}
