use crate::imports::*;

#[ derive (Clone, Copy, Debug) ]
pub enum FileType {
	Avi,
	IsoMedia,
	Matroska,
	Mp4v1,
	Mp4v2,
	Mpeg1,
	Mpeg2
}

impl FileType {

	pub fn identify_path (file_path: & Path) -> Result <FileType, IdentifyError> {
		Self::identify_reader (File::open (file_path) ?)
	}

	pub fn identify_reader (mut reader: impl Read) -> Result <FileType, IdentifyError> {
		let mut buf = vec! [0; 4096];
		let bytes_read = reader.read (& mut buf) ?;
		let buf = & buf [ .. bytes_read];
		Self::identify_slice (buf)
	}

	pub fn identify_slice (buf: & [u8]) -> Result <FileType, IdentifyError> {
		if 4 <= buf.len () && & buf [0 .. 4] == [ 0x1a, 0x45, 0xdf, 0xa3 ] {
			return Ok (FileType::Matroska);
		}
		if 12 <= buf.len () && & buf [0 .. 4] == b"RIFF" {
			if & buf [8 .. 12] == b"AVI " { return Ok (FileType::Avi) }
			return Err (IdentifyError::partial (format! (
				"Unknown RIFF file type: {:02x} {:02x} {:02x} {:02x}",
				buf [8], buf [9], buf [10], buf [11])));
		}
		if 20 <= buf.len () && & buf [0 .. 3] == b"\0\0\0" && & buf [4 .. 8] == b"ftyp" {
			if & buf [8 .. 12] == b"isom" { return Ok (FileType::IsoMedia) }
			if & buf [8 .. 12] == b"mp41" { return Ok (FileType::Mp4v1) }
			if & buf [8 .. 12] == b"mp42" { return Ok (FileType::Mp4v2) }
			return Err (IdentifyError::partial (format! (
				"Unknown ISOM file type: {:02x} {:02x} {:02x} {:02x}",
				buf [8], buf [9], buf [10], buf [11])));
		}
		if 5 <= buf.len () && & buf [0 .. 4] == [ 0x00, 0x00, 0x01, 0xba ] {
			if buf [4] & 0xf0 == 0x20 { return Ok (FileType::Mpeg1) }
			if buf [4] & 0xc0 == 0x40 { return Ok (FileType::Mpeg2) }
			return Err (IdentifyError::partial ("Unknown MPEG program stream file type"));
		}
		return Err (IdentifyError::NotRecognised);
	}

	pub fn needs_timestamp (self) -> bool {
		match self {
			Self::Avi => true,
			Self::IsoMedia => false,
			Self::Matroska => false,
			Self::Mp4v1 => false,
			Self::Mp4v2 => false,
			Self::Mpeg1 => true,
			Self::Mpeg2 => true,
		}
	}

}

#[ derive (Debug, thiserror::Error) ]
pub enum IdentifyError {
	#[ error ("{0}") ]
	PartiallyRecognised (String),
	#[ error ("Unknown file type") ]
	NotRecognised,
	#[ error ("Error reading file") ]
	IoError (#[from] io::Error),
}

impl IdentifyError {
	fn partial (message: impl Into <String>) -> Self {
		Self::PartiallyRecognised (message.into ())
	}
}
