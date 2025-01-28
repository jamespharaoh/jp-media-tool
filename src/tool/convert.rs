use crate::ffmpeg;
use crate::imports::*;

#[ derive (Debug, clap::Args) ]
#[ command (about = "Convert various file formats to matroska (mkv)" )]
pub struct Args {

	#[ clap (name = "FILE", help = "Files to convert") ]
	files: Vec <PathBuf>,

	#[ clap (long, help = "Skip all subtitle tracks") ]
	skip_subs: bool,

}

pub fn invoke (args: Args) -> anyhow::Result <()> {
	for file_path in & args.files {
		invoke_one (& args, file_path) ?;
	}
	Ok (())
}

fn invoke_one (args: & Args, file_path: & Path) -> anyhow::Result <()> {
	let Some (file_name) = file_path.file_name () else {
		any_bail! ("No filename in path");
	};
	let file_display = file_path.to_string_lossy ();
	let file_type = detect (file_path)
		.with_context (|| any_err! ("Error identifying file: {file_display}")) ?;
	let probe = ffmpeg::probe (file_path) ?;
	let strip_extension = match file_path.extension ().map (OsStr::as_encoded_bytes) {
		Some (b"avi" | b"AVI") => true,
		Some (b"m4v" | b"M4V") => true,
		Some (b"mkv" | b"MKV") => true,
		Some (b"mp4" | b"MP4") => true,
		Some (b"vob" | b"VOB") => true,
		_ => false,
	};
	let mut dest_name =
		if strip_extension { file_path.file_stem ().unwrap ().to_owned () }
		else { file_name.to_owned () };
	dest_name.push (".mkv");
	if dest_name == file_path {
		dest_name = file_path.file_stem ().unwrap ().to_owned ();
		dest_name.push ("-convert.mkv");
	}
	let dest_path = file_path.with_file_name (dest_name);
	if dest_path.try_exists () ? {
		any_bail! ("File already exists: {}", dest_path.display ());
	}
	let mut command: Vec <OsString> = Vec::new ();
	if file_type.needs_timestamp () {
		command.push ("-fflags".into ());
		command.push ("+genpts".into ());
	}
	command.push ("-i".into ());
	command.push (file_path.into ());
	command.push ("-map".into ());
	command.push ("0:v:0".into ());
	let num_audio =
		probe.streams.iter ()
			.filter (|stream| stream.stream_type == ffmpeg::StreamType::Audio)
			.count ();
	for audio_idx in 0 .. num_audio {
		command.push ("-map".into ());
		command.push (format! ("0:a:{audio_idx}").into ());
	}
	if ! args.skip_subs {
		for (subs_idx, subs_stream) in
				probe.streams.iter ()
					.filter (|stream| stream.stream_type == ffmpeg::StreamType::Subtitle)
					.enumerate () {
			match & * subs_stream.codec_name {
				"mov_text" => continue,
				"subrip" => (),
				codec => any_bail! ("Unknown subtitle codec: {codec}"),
			}
			command.push ("-map".into ());
			command.push (format! ("0:s:{subs_idx}").into ());
		}
	}
	command.push ("-codec".into ());
	command.push ("copy".into ());
	command.push ("-format".into ());
	command.push ("matroska".into ());
	command.push (dest_path.into ());
	let file_display = file_name.to_string_lossy ();
	ffmpeg::convert_progress (& file_display, Some ((probe.duration * 1_000_000.0) as u64), command) ?;
	Ok (())
}

fn detect (file_path: & Path) -> anyhow::Result <FileType> {
	let mut file = File::open (file_path) ?;
	let mut buf = vec! [0; 4096];
	let bytes_read = file.read (& mut buf) ?;
	let buf = & buf [ .. bytes_read];
	if 4 <= buf.len () && & buf [0 .. 4] == [ 0x1a, 0x45, 0xdf, 0xa3 ] {
		return Ok (FileType::Matroska);
	}
	if 12 <= buf.len () && & buf [0 .. 4] == b"RIFF" {
		if & buf [8 .. 12] == b"AVI " { return Ok (FileType::Avi) }
		any_bail! ("Unknown RIFF file type: {:02x} {:02x} {:02x} {:02x}", buf [8], buf [9], buf [10], buf [11]);
	}
	if 20 <= buf.len () && & buf [0 .. 3] == b"\0\0\0" && & buf [4 .. 8] == b"ftyp" {
		if & buf [8 .. 12] == b"isom" { return Ok (FileType::IsoMedia) }
		if & buf [8 .. 12] == b"mp41" { return Ok (FileType::Mp4v1) }
		if & buf [8 .. 12] == b"mp42" { return Ok (FileType::Mp4v2) }
		any_bail! ("Unknown ISOM file type: {:02x} {:02x} {:02x} {:02x}", buf [8], buf [9], buf [10], buf [11]);
	}
	if 5 <= buf.len () && & buf [0 .. 4] == [ 0x00, 0x00, 0x01, 0xba ] {
		if buf [4] & 0xc0 == 0x40 { return Ok (FileType::Mpeg) }
		any_bail! ("Unknown MPEG program stream file type");
	}
	any_bail! ("Unknown file type");
}

#[ derive (Clone, Copy) ]
enum FileType {
	Avi,
	IsoMedia,
	Matroska,
	Mp4v1,
	Mp4v2,
	Mpeg,
}

impl FileType {
	fn needs_timestamp (self) -> bool {
		match self {
			Self::Avi => true,
			Self::IsoMedia => false,
			Self::Matroska => false,
			Self::Mp4v1 => false,
			Self::Mp4v2 => false,
			Self::Mpeg => true,
		}
	}
}
