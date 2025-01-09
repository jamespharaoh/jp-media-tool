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
	let file_display = file_path.to_string_lossy ();
	(|| {
		let probe = ffmpeg::probe (file_path) ?;
		let mut file = File::open (file_path) ?;
		let mut buf = vec! [0; 4096];
		let bytes_read = file.read (& mut buf) ?;
		let buf = & buf [ .. bytes_read];
		if 4 <= buf.len () && & buf [0 .. 4] == [ 0x1a, 0x45, 0xdf, 0xa3 ] {
			// detected ebml (presumably matroska) TODO check properly
			return invoke_one_real (args, file_path, & probe, false);
		}
		if 12 <= buf.len () && & buf [0 .. 4] == b"RIFF" {
			// detected riff
			if & buf [8 .. 12] == b"AVI " {
				// detected avi
				return invoke_one_real (args, file_path, & probe, true);
			}
			any_bail! ("Unknown RIFF file type: {:02x} {:02x} {:02x} {:02x}", buf [8], buf [9], buf [10], buf [11]);
		}
		if 20 <= buf.len () && & buf [0 .. 3] == b"\0\0\0" && & buf [4 .. 8] == b"ftyp" {
			if & buf [8 .. 12] == b"isom" {
				// detected iso media
				return invoke_one_real (args, file_path, & probe, false);
			}
			if & buf [8 .. 12] == b"mp41" {
				// detected mp4 version 1
				return invoke_one_real (args, file_path, & probe, false);
			}
			if & buf [8 .. 12] == b"mp42" {
				// detected mp4 version 2
				return invoke_one_real (args, file_path, & probe, false);
			}
			any_bail! ("Unknown ISOM file type: {:02x} {:02x} {:02x} {:02x}", buf [8], buf [9], buf [10], buf [11]);
		}
		if 5 <= buf.len () && & buf [0 .. 4] == [ 0x00, 0x00, 0x01, 0xba ] {
			if buf [4] & 0xc0 == 0x40 {
				// detected mpeg program stream
				return invoke_one_real (args, file_path, & probe, true);
			}
			any_bail! ("Unknown MPEG program stream file type");
		}
		any_bail! ("Unknown file type");
	}) ().with_context (|| any_err! ("Error identifying file: {file_display}"))
}

fn invoke_one_real (
	args: & Args,
	file_path: & Path,
	probe: & ffmpeg::Info,
	avi: bool,
) -> anyhow::Result <()> {
	let Some (file_name) = file_path.file_name () else {
		any_bail! ("No filename in path");
	};
	let strip_extension = match file_path.extension ().map (OsStr::as_encoded_bytes) {
		Some (b"avi" | b"AVI") => true,
		Some (b"m4v" | b"M4V") => true,
		Some (b"mkv" | b"MKV") => true,
		Some (b"mp4" | b"MP4") => true,
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
	let mut command: Vec <OsString> = Vec::new ();
	if avi {
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
		let num_subs =
			probe.streams.iter ()
				.filter (|stream| stream.stream_type == ffmpeg::StreamType::Subtitle)
				.count ();
		for subs_idx in 0 .. num_subs {
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
