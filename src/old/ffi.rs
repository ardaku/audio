#![allow(non_camel_case_types)]

pub enum OpusEncoder { }
pub enum OpusDecoder { }
pub enum OpusRepacketizer { }

// ------ Constants from opus_defines.h ------

pub const OPUS_OK			   : i32 =  0;
pub const OPUS_BAD_ARG		  : i32 = -1;
pub const OPUS_BUFFER_TOO_SMALL : i32 = -2;
pub const OPUS_INTERNAL_ERROR   : i32 = -3;
pub const OPUS_INVALID_PACKET   : i32 = -4;
pub const OPUS_UNIMPLEMENTED	: i32 = -5;
pub const OPUS_INVALID_STATE	: i32 = -6;
pub const OPUS_ALLOC_FAIL	   : i32 = -7;
