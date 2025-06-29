use crate::detect;
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
	let file_type = detect::FileType::identify_path (file_path)
		.with_context (|| any_err! ("Error identifying file: {file_display}")) ?;
	eprintln! ("{} (probing...)", file_path.display ());
	let probe = ffmpeg::probe (file_path) ?;
	eprint! ("\x1b[A\x1b[J");
	let strip_extension = match file_path.extension ().map (OsStr::as_encoded_bytes) {
		Some (b"avi" | b"AVI") => true,
		Some (b"m4v" | b"M4V") => true,
		Some (b"mkv" | b"MKV") => true,
		Some (b"mpg" | b"MPG") => true,
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
	command.push ({
		let mut val = OsString::from ("file:");
		val.push (file_path);
		val
	});
	command.push ("-map".into ());
	command.push ("0:v:0".into ());
	command.push ("-codec:v:0".into ());
	command.push ("copy".into ());
	let num_audio =
		probe.streams.iter ()
			.filter (|stream| stream.stream_type == ffmpeg::StreamType::Audio)
			.count ();
	for audio_idx in 0 .. num_audio {
		command.push ("-map".into ());
		command.push (format! ("0:a:{audio_idx}").into ());
		command.push (format! ("-codec:a:{audio_idx}").into ());
		command.push ("copy".into ());
	}
	if ! args.skip_subs {
		let mut new_subs_idx = 0;
		for (subs_idx, subs_stream) in
				probe.streams.iter ()
					.filter (|stream| stream.stream_type == ffmpeg::StreamType::Subtitle)
					.enumerate () {
			let Some (codec) = (match subs_stream.codec_name.as_ref ().map (String::as_str) {
				Some ("ass") => Some ("copy"),
				Some ("dvd_subtitle") => any_bail! ("Can't convert DVD subtitles to text, consider --skip-subs"),
				Some ("mov_text") => Some ("srt"),
				Some ("subrip") => Some ("copy"),
				Some (codec) => any_bail! ("Unknown subtitle codec: {codec}"),
				None => any_bail! ("No codec for subtitle track"),
			}) else { continue };
			command.push ("-map".into ());
			command.push (format! ("0:s:{subs_idx}").into ());
			command.push (format! ("-codec:s:{new_subs_idx}").into ());
			command.push (codec.into ());
			new_subs_idx += 1;
		}
	}
	command.push ("-format".into ());
	command.push ("matroska".into ());
	command.push ({
		let mut val = OsString::from ("file:");
		val.push (dest_path);
		val
	});
	let file_display = file_name.to_string_lossy ();
	ffmpeg::convert_progress (& file_display, Some ((probe.duration * 1_000_000.0) as u64), command) ?;
	Ok (())
}
