use std::io;
use std::io::BufRead;
use std::io::Seek;
use std::io::SeekFrom;
use std::iter;

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

	pub fn read (& mut self) -> io::Result <Option <(u64, u64)>> {
		if self.next_pos.is_some () { panic! (); }
		if let Some (& limit) = self.posns.last () {
			if self.pos == limit { return Ok (None) }
		}
		if self.src.fill_buf () ?.is_empty () { return Ok (None) }
		let elem_id = self.read_elem_id () ?;
		let len = self.read_elem_len () ?.ok_or (io::ErrorKind::InvalidData) ?;
		self.next_pos = Some (self.pos + len);
		if let Some (& limit) = self.posns.last () {
			if limit < self.pos { return Err (io::ErrorKind::InvalidData) ? }
		}
		Ok (Some ((elem_id, len)))
	}

	pub fn data (& mut self) -> io::Result <Vec <u8>> {
		let mut buf = vec! [0; (self.next_pos.take ().unwrap () - self.pos) as usize];
		self.read_bytes (& mut buf) ?;
		Ok (buf)
	}

	pub fn skip (& mut self) -> io::Result <()> {
		let next_pos = self.next_pos.take ().unwrap ();
		self.set_pos (next_pos) ?;
		Ok (())
	}

	pub fn nest (& mut self) {
		let next_pos = self.next_pos.take ().unwrap ();
		self.posns.push (next_pos);
	}

	pub fn unnest (& mut self) -> io::Result <()> {
		let pos = self.posns.pop ().unwrap ();
		self.set_pos (pos) ?;
		Ok (())
	}

	pub fn unsigned (& mut self) -> io::Result <u64> {
		let data = self.data () ?;
		if 8 < data.len () { return Err (io::ErrorKind::InvalidData.into ()) }
		let mut bytes = [0; 8];
		let skip = 8 - data.len ();
		for (src, dst) in iter::zip (data, & mut bytes [skip .. ]) { * dst = src }
		Ok (u64::from_be_bytes (bytes))
	}

	pub fn string (& mut self) -> io::Result <String> {
		let mut data = self.data () ?;
		while ! data.is_empty () && data [data.len () - 1] == 0 {
			data.pop ().unwrap ();
		}
		let val =
			String::from_utf8 (data)
				.map_err (|err| io::Error::new (io::ErrorKind::InvalidData, err)) ?;
		Ok (val)
	}

	fn read_elem_id (& mut self) -> io::Result <u64> {
		let mut buf = [0; 8];
		self.read_bytes (& mut buf [0 .. 1]) ?;
		let num_bytes = buf [0].leading_zeros () + 1;
		if num_bytes < 1 || 8 < num_bytes {
			return Err (io::ErrorKind::InvalidData.into ());
		}
		self.read_bytes (& mut buf [1 .. num_bytes as usize]) ?;
		let mut val = 0_u64;
		for & byte in buf [0 .. num_bytes as usize].iter () {
			val <<= 8;
			val |= byte as u64;
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
		if val == 0 {
			return Err (io::ErrorKind::InvalidData.into ());
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