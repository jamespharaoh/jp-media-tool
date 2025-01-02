use crate::imports::*;

#[ derive (Debug, clap::Args) ]
#[ command (about = "Convert various file formats to matroska (mkv)" )]
pub struct Args {

	#[ clap (name = "FILE", help = "Files to convert") ]
	files: Vec <PathBuf>,

}

pub fn invoke (args: Args) -> anyhow::Result <()> {
	for file_path in & args.files {
		if ! invoke_one (& args, file_path) ? { break }
	}
	Ok (())
}

fn invoke_one (args: & Args, file_path: & Path) -> anyhow::Result <bool> {
	println! ("{}", file_path.display ());
	let mut file = File::open (file_path) ?;
	let mut buf = vec! [0; 4096];
	let bytes_read = file.read (& mut buf) ?;
	let buf = & buf [ .. bytes_read];
	if 12 <= buf.len () && & buf [0 .. 4] == b"RIFF" {
		if & buf [8 .. 12] == b"AVI " {
			println! ("Detected AVI file format");
			return invoke_ffmpeg (args, file_path, true);
		}
		any_bail! ("Unknown RIFF file type: {:02x} {:02x} {:02x} {:02x}", buf [8], buf [9], buf [10], buf [11]);
	}
	if 20 <= buf.len () && & buf [0 .. 3] == b"\0\0\0" && & buf [4 .. 8] == b"ftyp" {
		if & buf [8 .. 12] == b"isom" {
			println! ("Detected ISO base media file format");
			return invoke_ffmpeg (args, file_path, false);
		}
		if & buf [8 .. 12] == b"mp41" {
			println! ("Detected MP4 version 1 file format");
			return invoke_ffmpeg (args, file_path, false);
		}
		if & buf [8 .. 12] == b"mp42" {
			println! ("Detected MP4 version 2 file format");
			return invoke_ffmpeg (args, file_path, false);
		}
		any_bail! ("Unknown ISOM file type: {:02x} {:02x} {:02x} {:02x}", buf [8], buf [9], buf [10], buf [11]);
	}
	if 5 <= buf.len () && & buf [0 .. 3] == [ 0x00, 0x00, 0x01, 0xba ] {
		if buf [4] & 0xc0 == 0x40 {
			println! ("Detected MPEG program stream file format");
			return invoke_ffmpeg (args, file_path, true);
		}
		any_bail! ("Unknown MPEG program stream file type");
	}
	any_bail! ("Unknown file type");
}

fn invoke_ffmpeg (_args: & Args, file_path: & Path, avi: bool) -> anyhow::Result <bool> {
	let Some (file_name) = file_path.file_name () else {
		any_bail! ("No filename in path");
	};
	let strip_extension = match file_path.extension ().map (OsStr::as_encoded_bytes) {
		Some (b"avi" | b"AVI") => true,
		Some (b"mp4" | b"MP4") => true,
		_ => false,
	};
	let mut dest_name =
		if strip_extension { file_path.file_stem ().unwrap ().to_owned () }
		else { file_name.to_owned () };
	dest_name.push (".mkv");
	let dest_path = file_path.with_file_name (dest_name);
	let mut command: Vec <OsString> = Vec::new ();
	command.push ("-hide_banner".into ());
	if avi {
		command.push ("-fflags".into ());
		command.push ("+genpts".into ());
	}
	command.push ("-i".into ());
	command.push (file_path.into ());
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
