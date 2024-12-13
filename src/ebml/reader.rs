use crate::imports::*;

pub type Blob = Vec <u8>;

#[ derive (Debug) ]
pub struct BlobRef {
	pub start: u64,
	pub end: u64,
}

pub trait EbmlRead {

	fn read (& mut self) -> io::Result <Option <(u64, u64, u64)>>;
	fn data (& mut self) -> io::Result <Vec <u8>>;
	fn data_ref (& mut self) -> io::Result <BlobRef>;
	fn skip (& mut self) -> io::Result <()>;
	fn nest (& mut self);
	fn unnest (& mut self) -> io::Result <()>;

	fn unsigned (& mut self) -> io::Result <u64> {
		let data = self.data () ?;
		if 8 < data.len () {
			return Err (io::ErrorKind::InvalidData.into ()) }
		let mut bytes = [0; 8];
		let skip = 8 - data.len ();
		for (src, dst) in iter::zip (data, & mut bytes [skip .. ]) { * dst = src }
		Ok (u64::from_be_bytes (bytes))
	}

	fn boolean (& mut self) -> io::Result <bool> {
		match self.unsigned () ? {
			0 => Ok (false),
			1 => Ok (true),
			_ => Err (io::ErrorKind::InvalidData.into ()),
		}
	}

	fn float (& mut self) -> io::Result <f64> {
		let data = self.data () ?;
		match data.len () {
			4 => Ok (f32::from_be_bytes ([ data [0], data [1], data [2], data [3] ]) as f64),
			8 => Ok (f64::from_be_bytes (data [..].try_into ().unwrap ())),
			_ => Err (io::ErrorKind::InvalidData.into ()),
		}
	}

	fn binary (& mut self) -> io::Result <Vec <u8>> {
		let mut data = self.data () ?;
		while ! data.is_empty () && data [data.len () - 1] == 0 {
			data.pop ().unwrap ();
		}
		Ok (data)
	}

	fn string (& mut self) -> io::Result <String> {
		let mut data = self.data () ?;
		while ! data.is_empty () && data [data.len () - 1] == 0 {
			data.pop ().unwrap ();
		}
		let val =
			String::from_utf8 (data)
				.map_err (|err| io::Error::new (io::ErrorKind::InvalidData, err)) ?;
		Ok (val)
	}

}

pub struct EbmlReader <Src> {
	src: Src,
	pos: u64,
	next_pos: Option <u64>,
	posns: Vec <u64>,
}

impl <Src: BufRead + Seek> EbmlReader <Src> {

	pub fn new (mut src: Src) -> io::Result <Self> {
		src.seek (SeekFrom::Start (0)) ?;
		Ok (Self {
			src: src,
			pos: 0,
			next_pos: None,
			posns: Vec::new (),
		})
	}

	fn read_elem_id (& mut self) -> io::Result <u64> {
		let mut buf = [0; 8];
		self.read_bytes (& mut buf [0 .. 1]) ?;
		if buf [0] == 0x00 {
			return Err (io::Error::new (
				io::ErrorKind::InvalidData,
				"Invalid first byte of element id: 0x00"));
		}
		let num_bytes = buf [0].leading_zeros () + 1;
		debug_assert! (1 <= num_bytes && num_bytes < 9);
		self.read_bytes (& mut buf [1 .. num_bytes as usize]) ?;
		let mut val = 0_u64;
		for & byte in buf [0 .. num_bytes as usize].iter () {
			val <<= 8;
			val |= byte as u64;
		}
		if val == 0 {
			return Err (io::Error::new (
				io::ErrorKind::InvalidData,
				"Invalid element id: 0x00"));
		}
		Ok (val)
	}

	fn read_elem_len (& mut self) -> io::Result <Option <u64>> {
		let mut buf = [0; 8];
		self.read_bytes (& mut buf [0 .. 1]) ?;
		let num_bytes = buf [0].leading_zeros () + 1;
		if num_bytes < 1 || 8 < num_bytes {
			return Err (io::ErrorKind::InvalidData.into ());
		}
		if num_bytes < 8 {
			buf [0] &= 0xff >> num_bytes;
		} else {
			buf [0] = 0;
		}
		self.read_bytes (& mut buf [1 .. num_bytes as usize]) ?;
		let mut val = 0_u64;
		for & byte in buf [0 .. num_bytes as usize].iter () {
			val <<= 8;
			val |= byte as u64;
		}
		if val == (1 << num_bytes * 7) - 1 {
			return Ok (None);
		}
		Ok (Some (val))
	}

	fn set_pos (& mut self, pos: u64) -> io::Result <()> {
		if self.pos == pos { return Ok (()) }
		self.src.seek (SeekFrom::Start (pos)) ?;
		self.pos = pos;
		Ok (())
	}

	fn read_bytes (& mut self, mut buf: & mut [u8]) -> io::Result <()> {
		while ! buf.is_empty () {
			let len = self.src.read (buf) ?;
			if len == 0 { return Err (io::ErrorKind::UnexpectedEof.into ()) }
			self.pos += len as u64;
			buf = & mut buf [len .. ];
		}
		Ok (())
	}

}

impl <Src: BufRead + Seek> EbmlRead for EbmlReader <Src> {

	fn read (& mut self) -> io::Result <Option <(u64, u64, u64)>> {
		if self.next_pos.is_some () { panic! (); }
		if let Some (& limit) = self.posns.last () {
			if self.pos == limit { return Ok (None) }
		}
		if self.src.fill_buf () ?.is_empty () { return Ok (None) }
		let elem_pos = self.pos;
		let elem_id = self.read_elem_id () ?;
		let elem_len = self.read_elem_len () ?.ok_or (io::ErrorKind::InvalidData) ?;
		self.next_pos = Some (self.pos + elem_len);
		if let Some (& limit) = self.posns.last () {
			if limit < self.pos {
				return Err (io::Error::new (
					io::ErrorKind::InvalidData,
					"Read past end of container"));
			}
		}
		Ok (Some ((elem_id, elem_pos, elem_len)))
	}

	fn data (& mut self) -> io::Result <Vec <u8>> {
		let mut buf = vec! [0; (self.next_pos.take ().unwrap () - self.pos) as usize];
		self.read_bytes (& mut buf) ?;
		Ok (buf)
	}

	fn data_ref (& mut self) -> io::Result <BlobRef> {
		let start = self.pos;
		let end = self.next_pos.take ().unwrap ();
		self.set_pos (end) ?;
		Ok (BlobRef { start, end })
	}

	fn skip (& mut self) -> io::Result <()> {
		let next_pos = self.next_pos.take ().unwrap ();
		self.set_pos (next_pos) ?;
		Ok (())
	}

	fn nest (& mut self) {
		let next_pos = self.next_pos.take ().unwrap ();
		self.posns.push (next_pos);
	}

	fn unnest (& mut self) -> io::Result <()> {
		let pos = self.posns.pop ().unwrap ();
		self.set_pos (pos) ?;
		Ok (())
	}

}

pub trait EbmlValue: Sized {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <Self>;
}

impl EbmlValue for bool {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		Ok (reader.boolean () ?)
	}
}

impl EbmlValue for u64 {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <u64> {
		Ok (reader.unsigned () ?)
	}
}

impl EbmlValue for f64 {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <f64> {
		Ok (reader.float () ?)
	}
}

impl EbmlValue for Blob {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <Blob> {
		Ok (reader.binary () ?)
	}
}

impl EbmlValue for String {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <String> {
		Ok (reader.string () ?)
	}
}

impl EbmlValue for BlobRef {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <Self> {
		Ok (reader.data_ref () ?)
	}
}
