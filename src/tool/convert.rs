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
		if ! invoke_one (& args, file_path) ? { break }
	}
	Ok (())
}

fn invoke_one (args: & Args, file_path: & Path) -> anyhow::Result <bool> {
	println! ("{}", file_path.display ());
	let probe = invoke_ffprobe (file_path) ?;
	let mut file = File::open (file_path) ?;
	let mut buf = vec! [0; 4096];
	let bytes_read = file.read (& mut buf) ?;
	let buf = & buf [ .. bytes_read];
	if 4 <= buf.len () && & buf [0 .. 4] == [ 0x1a, 0x45, 0xdf, 0xa3 ] {
		// TODO check EBML document type
		println! ("Detected Matroska file format");
		return invoke_ffmpeg (args, file_path, & probe, false);
	}
	if 12 <= buf.len () && & buf [0 .. 4] == b"RIFF" {
		if & buf [8 .. 12] == b"AVI " {
			println! ("Detected AVI file format");
			return invoke_ffmpeg (args, file_path, & probe, true);
		}
		any_bail! ("Unknown RIFF file type: {:02x} {:02x} {:02x} {:02x}", buf [8], buf [9], buf [10], buf [11]);
	}
	if 20 <= buf.len () && & buf [0 .. 3] == b"\0\0\0" && & buf [4 .. 8] == b"ftyp" {
		if & buf [8 .. 12] == b"isom" {
			println! ("Detected ISO base media file format");
			return invoke_ffmpeg (args, file_path, & probe, false);
		}
		if & buf [8 .. 12] == b"mp41" {
			println! ("Detected MP4 version 1 file format");
			return invoke_ffmpeg (args, file_path, & probe, false);
		}
		if & buf [8 .. 12] == b"mp42" {
			println! ("Detected MP4 version 2 file format");
			return invoke_ffmpeg (args, file_path, & probe, false);
		}
		any_bail! ("Unknown ISOM file type: {:02x} {:02x} {:02x} {:02x}", buf [8], buf [9], buf [10], buf [11]);
	}
	if 5 <= buf.len () && & buf [0 .. 4] == [ 0x00, 0x00, 0x01, 0xba ] {
		if buf [4] & 0xc0 == 0x40 {
			println! ("Detected MPEG program stream file format");
			return invoke_ffmpeg (args, file_path, & probe, true);
		}
		any_bail! ("Unknown MPEG program stream file type");
	}
	any_bail! ("Unknown file type");
}

fn invoke_ffmpeg (args: & Args, file_path: & Path, probe: & [Stream], avi: bool) -> anyhow::Result <bool> {
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
	command.push ("-hide_banner".into ());
	if avi {
		command.push ("-fflags".into ());
		command.push ("+genpts".into ());
	}
	command.push ("-i".into ());
	command.push (file_path.into ());
	command.push ("-map".into ());
	command.push ("0:v:0".into ());
	let num_audio =
		probe.iter ()
			.filter (|stream| stream.stream_type == StreamType::Audio)
			.count ();
	for audio_idx in 0 .. num_audio {
		command.push ("-map".into ());
		command.push (format! ("0:a:{audio_idx}").into ());
	}
	if ! args.skip_subs {
		let num_subs =
			probe.iter ()
				.filter (|stream| stream.stream_type == StreamType::Subtitle)
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
	let mut proc =
		process::Command::new ("ffmpeg")
			.args (command)
			.stdin (process::Stdio::null ())
			.stdout (process::Stdio::inherit ())
			.stderr (process::Stdio::inherit ())
			.spawn ()
			.unwrap ();
	let result = proc.wait ().unwrap ();
	if ! result.success () {
		if let Some (code) = result.code () {
			eprintln! ("Encoder process returned status {:?}", code);
		} else {
			eprintln! ("Encoder process terminated abnormally");
		}
		return Ok (false)
	}
    Ok (true)
}

fn invoke_ffprobe (file_path: & Path) -> anyhow::Result <Vec <Stream>> {
	let mut command: Vec <OsString> = Vec::new ();
	command.push ("-hide_banner".into ());
	command.push ("-print_format".into ());
	command.push ("json".into ());
	command.push ("-show_streams".into ());
	command.push (file_path.into ());
	let output =
		process::Command::new ("ffprobe")
			.args (command)
			.output () ?;
	if ! output.status.success () {
		io::stderr ().write_all (& output.stderr) ?;
		any_bail! ("Error invoking ffprobe: {}", output.status);
	}
	let data: FfData = serde_json::from_slice (& output.stdout) ?;
	data.streams.iter ()
		.map (|stream| Ok (Stream {
			stream_type: match & * stream.codec_type {
				"video" => StreamType::Video,
				"audio" => StreamType::Audio,
				"subtitle" => StreamType::Subtitle,
				codec_type => any_bail! ("Invalid codec type: {codec_type}"),
			},
		}))
		.collect ()
}

#[ derive (Debug) ]
struct Stream {
	stream_type: StreamType,
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum StreamType {
	Video,
	Audio,
	Subtitle,
}

#[ derive (Debug, Deserialize) ]
struct FfData {
	streams: Vec <FfStream>,
}

#[ derive (Debug, Deserialize) ]
struct FfStream {
	codec_type: String,
}
