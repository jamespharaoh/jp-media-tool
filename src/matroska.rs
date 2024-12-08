#[ allow (dead_code) ]
pub mod elem_ids {

	pub const CLUSTER: u64 = 0x1f43b675;
	pub const CLUSTER_TIMESTAMP: u64 = 0xe7;
	pub const CLUSTER_POSITION: u64 = 0xa7;
	pub const CLUSTER_PREV_SIZE: u64 = 0xab;
	pub const CLUSTER_SIMPLE_BLOCK: u64 = 0xa3;
	pub const CLUSTER_BLOCK_GROUP: u64 = 0xa0;
	pub const CLUSTER_BLOCK_GROUP_BLOCK: u64 = 0xa1;
	pub const CLUSTER_BLOCK_ADDITIONS: u64 = 0x75a1;
	pub const CLUSTER_BLOCK_MORE: u64 = 0xa6;
	pub const CLUSTER_BLOCK_ADDITIONAL: u64 = 0xa5;
	pub const CLUSTER_BLOCK_ADD_ID: u64 = 0xee;
	pub const CLUSTER_BLOCK_DURATION: u64 = 0x9b;
	pub const CLUSTER_REFERENCE_PRIORITY: u64 = 0xfa;
	pub const CLUSTER_REFERENCE_BLOCK: u64 = 0xfb;
	pub const CLUSTER_CODEC_STATE: u64 = 0xa4;
	pub const CLUSTER_DISCARD_PADDING: u64 = 0x75a2;

	pub const TRACKS: u64 = 0x1654ae6b;
	pub const TRACK_ENTRY: u64 = 0xae;
	pub const TRACK_NUMBER: u64 = 0xd7;
	pub const TRACK_UID: u64 = 0x73c5;
	pub const TRACK_TYPE: u64 = 0x83;
	pub const TRACK_FLAG_ENABLED: u64 = 0xb9;
	pub const TRACK_FLAG_DEFAULT: u64 = 0x88;
	pub const TRACK_FLAG_FORCED: u64 = 0x55aa;
	pub const TRACK_FLAG_HEARING_IMPAIRED: u64 = 0x55ab;
	pub const TRACK_FLAG_VISUAL_IMPAIRED: u64 = 0x55ac;
	pub const TRACK_FLAG_TEXT_DESCRIPTIONS: u64 = 0x55ad;
	pub const TRACK_FLAG_ORIGINAL: u64 = 0x55ae;
	pub const TRACK_FLAG_COMMENTARY: u64 = 0x55af;
	pub const TRACK_FLAG_LACING: u64 = 0x9c;
	pub const TRACK_DEFAULT_DURATION: u64 = 0x23e383;
	pub const TRACK_DEFAULT_DECODED_FIELD_DURATION: u64 = 0x234e7a;
	pub const TRACK_MAX_BLOCK_ADDITION_ID: u64 = 0x55ee;
	pub const TRACK_BLOCK_ADDITION_MAPPING: u64 = 0x41e4;
	pub const TRACK_BLOCK_ADD_ID_VALUE: u64 = 0x41f0;
	pub const TRACK_BLOCK_ADD_ID_NAME: u64 = 0x41f4;
	pub const TRACK_BLOCK_ADD_ID_TYPE: u64 = 0x41e7;
	pub const TRACK_BLOCK_ADD_ID_EXTRA_DATA: u64 = 0x41ed;
	pub const TRACK_NAME: u64 = 0x536e;
	pub const TRACK_LANGUAGE: u64 = 0x22b59c;
	pub const TRACK_LANGUAGE_BCP47: u64 = 0x22b59d;
	pub const TRACK_CODEC_ID: u64 = 0x86;
	pub const TRACK_CODEC_PRIVATE: u64 = 0x63a2;
	pub const TRACK_CODEC_NAME: u64 = 0x258688;
	pub const TRACK_CODEC_DELAY: u64 = 0x56aa;
	pub const TRACK_SEEK_PRE_ROLL: u64 = 0x56bb;
	pub const TRACK_TRANSLATE: u64 = 0x6624;
	pub const TRACK_TRANSLATE_TRACK_ID: u64 = 0x66a5;
	pub const TRACK_TRANSLATE_CODEC: u64 = 0x66bf;
	pub const TRACK_TRANSLATE_EDITION_UID: u64 = 0x66fc;
	pub const TRACK_VIDEO: u64 = 0xe0;
	pub const TRACK_FLAG_INTERLACED: u64 = 0x9a;
	pub const TRACK_FIELD_ORDER: u64 = 0x9d;
	pub const TRACK_STEREO_MODE: u64 = 0x53b8;
	pub const TRACK_ALPHA_MODE: u64 = 0x53c0;
	pub const TRACK_PIXEL_WIDTH: u64 = 0xb0;
	pub const TRACK_PIXEL_HEIGHT: u64 = 0xba;
	pub const TRACK_PIXEL_CROP_BOTTOM: u64 = 0x54aa;
	pub const TRACK_PIXEL_CROP_TOP: u64 = 0x54bb;
	pub const TRACK_PIXEL_CROP_LEFT: u64 = 0x54cc;
	pub const TRACK_PIXEL_CROP_RIGHT: u64 = 0x54dd;
	pub const TRACK_DISPLAY_WIDTH: u64 = 0x54b0;
	pub const TRACK_DISPLAY_HEIGHT: u64 = 0x54ba;
	pub const TRACK_DISPLAY_UNIT: u64 = 0x54b2;
	pub const TRACK_UNCOMPRESSED_FOUR_CC: u64 = 0x2eb524;
	pub const TRACK_COLOUR: u64 = 0x55b0;
	pub const TRACK_MATRIX_COEFFICIENTS: u64 = 0x55b1;
	pub const TRACK_BITS_PER_CHANNEL: u64 = 0x55b2;
	pub const TRACK_CHROMA_SUBSAMPLING_HORZ: u64 = 0x55b3;
	pub const TRACK_CHROMA_SUBSAMPLING_VERT: u64 = 0x55b4;
	pub const TRACK_CB_SUBSAMPLING_HORZ: u64 = 0x55b5;
	pub const TRACK_CB_SUBSAMPLING_VERT: u64 = 0x55b6;
	pub const TRACK_CHROMA_SITING_HORZ: u64 = 0x55b7;
	pub const TRACK_CHROMA_SITING_VERT: u64 = 0x55b8;
	pub const TRACK_RANGE: u64 = 0x55b9;
	pub const TRACK_TRANSFER_CHARACTERISTICS: u64 = 0x55ba;
	pub const TRACK_PRIMARIES: u64 = 0x55bb;

	pub const SEEK_HEAD: u64 = 0x114d9b74;
	pub const SEEK_ENTRY: u64 = 0x4dbb;
	pub const SEEK_ENTRY_ID: u64 = 0x53ab;
	pub const SEEK_ENTRY_POSITION: u64 = 0x53ac;

	pub const SEGMENT: u64 = 0x18538067;
	pub const SEGMENT_INFO: u64 = 0x1549a966;
	pub const SEGMENT_UUID: u64 = 0x73a4;
	pub const SEGMENT_FILENAME: u64 = 0x7384;
	pub const SEGMENT_PREV_UUID: u64 = 0x3cb923;
	pub const SEGMENT_NEXT_UUID: u64 = 0x3eb923;
	pub const SEGMENT_NEXT_FILENAME: u64 = 0x3e83bb;
	pub const SEGMENT_FAMILY: u64 = 0x4444;
	pub const SEGMENT_CHAPTER_TRANSLATE: u64 = 0x6924;
	pub const SEGMENT_CHAPTER_TRANSLATE_ID: u64 = 0x69a5;
	pub const SEGMENT_CHAPTER_TRNSLATE_CODEC: u64 = 0x69bf;
	pub const SEGMENT_CHAPTER_TRANSLATE_EDITION_UID: u64 = 0x69fc;
	pub const SEGMENT_TIMESTAMP_SCALE: u64 = 0x2ad7b1;
	pub const SEGMENT_DURATION: u64 = 0x4489;
	pub const SEGMENT_DATE_UTC: u64 = 0x4461;
	pub const SEGMENT_TITLE: u64 = 0x7ba9;
	pub const SEGMENT_MUXING_APP: u64 = 0x4d80;
	pub const SEGMENT_WRITING_APP: u64 = 0x5741;

}