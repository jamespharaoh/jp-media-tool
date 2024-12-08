use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::fs::File;

fn main () -> anyhow::Result <()> {
	let file = BufReader::new (File::open ("test.mkv") ?);
	let mut ebml = EbmlReader::new (file);
	let (ebml_id, ebml_len) = ebml.read () ?.ok_or_else (|| anyhow::format_err! ("No ebml element")) ?;
	println! ("Got ebml element id=0x{ebml_id:x}, len={ebml_len}");
	anyhow::ensure! (ebml_id == elem_ids::EBML);
	ebml.nest ();
	while let Some ((elem_id, elem_len)) = ebml.read () ? {
		println! ("  Got element id=0x{elem_id:x}, len={elem_len}");
		ebml.skip () ?;
	}
	ebml.unnest () ?;
	let (body_id, body_len) = ebml.read () ?.ok_or_else (|| anyhow::format_err! ("No segment element")) ?;
	anyhow::ensure! (body_id == elem_ids::SEGMENT);
	println! ("Got body element id=0x{body_id:x}, len={body_len}");
	ebml.nest ();
	while let Some ((elem_id, elem_len)) = ebml.read () ? {
		println! ("  Got element id=0x{elem_id:x}, len={elem_len}");
		ebml.skip () ?;
	}
	ebml.unnest () ?;
    Ok (())
}

#[ allow (dead_code) ]
mod elem_ids {

	pub const EBML: u64 = 0x1a45dfa3;
	pub const EBML_VERSION: u64 = 0x4286;
	pub const EBML_READ_VERSION: u64 = 0x42f7;
	pub const EBML_MAX_ID_LENGTH: u64 = 0x42f2;
	pub const EBML_MAX_SIZE_LENGTH: u64 = 0x42f3;
	pub const EBML_DOC_TYPE: u64 = 0x4282;
	pub const EBML_DOC_TYPE_VERSION: u64 = 0x4287;
	pub const EBML_DOC_TYPE_READ_VERSION: u64 = 0x4285;
	pub const EBML_DOC_TYPE_EXTENSION: u64 = 0x4281;
	pub const EBML_DOC_TYPE_EXTENSION_NAME: u64 = 0x4283;
	pub const EBML_DOC_TYPE_EXTENSION_VERSION: u64 = 0x4284;

	pub const CRC_32: u64 = 0xbf;
	pub const VOID: u64 = 0xec;

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

struct EbmlReader <Src> {
	src: TellReader <Src>,
	next_pos: Option <u64>,
	posns: Vec <u64>,
}

impl <Src: Read + Seek> EbmlReader <Src> {

	pub fn new (src: Src) -> Self {
		Self {
			src: TellReader::new (src),
			next_pos: None,
			posns: Vec::new (),
		}
	}

	pub fn read (& mut self) -> io::Result <Option <(u64, u64)>> {
		if self.next_pos.is_some () { panic! (); }
		if let Some (& limit) = self.posns.last () {
			if self.src.pos () == limit { return Ok (None) }
		}
		let elem_id = self.read_elem_id () ?;
		let len = self.read_elem_len () ?.ok_or (io::ErrorKind::InvalidData) ?;
		self.next_pos = Some (self.src.pos () + len);
		if let Some (& limit) = self.posns.last () {
			if limit < self.src.pos () { return Err (io::ErrorKind::InvalidData) ? }
		}
		Ok (Some ((elem_id, len)))
	}

	pub fn skip (& mut self) -> io::Result <()> {
		let next_pos = self.next_pos.take ().unwrap ();
		self.src.set_pos (next_pos) ?;
		Ok (())
	}

	pub fn nest (& mut self) {
		let next_pos = self.next_pos.take ().unwrap ();
		self.posns.push (next_pos);
	}

	pub fn unnest (& mut self) -> io::Result <()> {
		let pos = self.posns.pop ().unwrap ();
		self.src.set_pos (pos) ?;
		Ok (())
	}

	fn read_elem_id (& mut self) -> io::Result <u64> {
		let mut buf = [0; 8];
		self.src.read_exact (& mut buf [0 .. 1]) ?;
		let num_bytes = buf [0].leading_zeros () + 1;
		if num_bytes < 1 || 8 < num_bytes {
			return Err (io::ErrorKind::InvalidData.into ());
		}
		self.src.read_exact (& mut buf [1 .. num_bytes as usize]) ?;
		let mut val = 0_u64;
		for & byte in buf [0 .. num_bytes as usize].iter () {
			val <<= 8;
			val |= byte as u64;
		}
		Ok (val)
	}

	fn read_elem_len (& mut self) -> io::Result <Option <u64>> {
		let mut buf = [0; 8];
		self.src.read_exact (& mut buf [0 .. 1]) ?;
		let num_bytes = buf [0].leading_zeros () + 1;
		if num_bytes < 1 || 8 < num_bytes {
			return Err (io::ErrorKind::InvalidData.into ());
		}
		if num_bytes < 8 {
			buf [0] &= 0xff >> num_bytes;
		} else {
			buf [0] = 0;
		}
		self.src.read_exact (& mut buf [1 .. num_bytes as usize]) ?;
		let mut val = 0_u64;
		for & byte in buf [0 .. num_bytes as usize].iter () {
			val <<= 8;
			val |= byte as u64;
		}
		if val == 0 {
			return Err (io::ErrorKind::InvalidData.into ());
		}
		if val == (1 << num_bytes * 7) - 1 {
			return Ok (None);
		}
		Ok (Some (val))
	}

}

struct TellReader <Src> {
	src: Src,
	pos: u64,
}

impl <Src> TellReader <Src> {

	pub fn new (src: Src) -> Self {
		Self {
			src,
			pos: 0,
		}
	}

	pub fn pos (& self) -> u64 {
		self.pos
	}

	pub fn set_pos (& mut self, pos: u64) -> io::Result <()>
			where Src: Seek {
		if self.pos == pos { return Ok (()) }
		self.src.seek (SeekFrom::Start (pos)) ?;
		self.pos = pos;
		Ok (())
	}

}

impl <Src: Read> Read for TellReader <Src> {

	fn read (& mut self, buf: & mut [u8]) -> io::Result <usize> {
		let len = self.src.read (buf) ?;
		self.pos += len as u64;
		Ok (len)
	}

}
