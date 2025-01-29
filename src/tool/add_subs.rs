use crate::ffmpeg;
use crate::imports::*;
use crate::matroska;

#[ derive (Debug, clap::Args) ]
#[ command (about = "Reencode video as x265 (optionally) and audio as opus" )]
pub struct Args {

	#[ clap (name = "SOURCE", help = "File to add subtitles to") ]
	source_path: PathBuf,

	#[ clap (name = "SUBS", help = "Subtitles to add") ]
	subs_path: PathBuf,

	#[ clap (long, help = "Subtitle track language tag") ]
	lang: String,

	#[ clap (long, help = "Subtitle track title") ]
	title: Option <String>,

	#[ clap (long, help = "Mark subtitles as forced") ]
	forced: bool,

	#[ clap (long, help = "Mark subtitles as default") ]
	default: bool,

	#[ clap (long, help = "Mark subtitles as hearing-impaired") ]
	hearing_impaired: bool,

	#[ clap (long, help = "Mark subtitles as visual-impaired") ]
	visual_impaired: bool,

	#[ clap (long, help = "Mark subtitles as text descriptions") ]
	descriptions: bool,

	#[ clap (long, help = "Mark subtitles as original") ]
	original: bool,

	#[ clap (long, help = "Mark subtitles as commentary") ]
	commentary: bool,

}

pub fn invoke (args: Args) -> anyhow::Result <()> {

	// check file name

	let Some (source_name) = args.source_path.file_name () else {
		any_bail! ("Specified file has no name: {}", args.source_path.display ());
	};

	let mut dest_name = args.source_path.file_stem ().unwrap ().to_owned ();
	dest_name.push (format! ("-subs-{}", args.lang));

	// open source file

	let file = BufReader::new (File::open (& args.source_path) ?);
	let mut reader = matroska::Reader::new (file) ?;

	let info = reader.segment_info () ?;
	let duration_micros = info.duration.map (|duration| (info.timestamp_scale as f64 * duration / 1000.0) as u64);

	let tracks = reader.tracks () ?;
	let mut command: Vec <OsString> = Vec::new ();
	command.push ("-i".into ());
	command.push ({
		let mut val = OsString::from ("file:");
		val.push (& args.source_path);
		val
	});
	command.push ("-i".into ());
	command.push ({
		let mut val = OsString::from ("file:");
		val.push (args.subs_path);
		val
	});

	// do video

	let video_tracks: Vec <_> =
		tracks.entries.iter ()
			.filter (|track| track.track_type == 1)
			.collect ();
	for (video_idx, _video_track) in video_tracks.iter ().enumerate () {
		command.push ("-map".into ());
		command.push (format! ("0:v:{video_idx}").into ());
		command.push (format! ("-c:v:{video_idx}").into ());
		command.push ("copy".into ());
	}

	// do audio

	let audio_tracks: Vec <_> =
		tracks.entries.iter ()
			.filter (|track| track.track_type == 2)
			.collect ();
	for (audio_idx, _audio_track) in audio_tracks.iter ().enumerate () {
		command.push ("-map".into ());
		command.push (format! ("0:a:{audio_idx}").into ());
		command.push (format! ("-c:a:{audio_idx}").into ());
		command.push ("copy".into ());
	}

	// do subtitles

	let subs_tracks: Vec <_> =
		tracks.entries.iter ()
			.filter (|track| track.track_type == 17)
			.collect ();
	for (subs_idx, _subs_track) in subs_tracks.iter ().enumerate () {
		command.push ("-map".into ());
		command.push (format! ("0:s:{subs_idx}").into ());
		command.push (format! ("-c:s:{subs_idx}").into ());
		command.push ("copy".into ());
	}

	// new subtitles

	let new_subs_idx = subs_tracks.len ();
	command.push ("-map".into ());
	command.push ("1:s:0".into ());
	command.push (format! ("-c:s:{new_subs_idx}").into ());
	command.push ("copy".into ());
	command.push (format! ("-metadata:s:s:{new_subs_idx}").into ());
	command.push (format! ("language={}", args.lang).into ());
	if let Some (title) = args.title.as_ref () {
		command.push (format! ("-metadata:s:s:{new_subs_idx}").into ());
		command.push (format! ("title={title}").into ());
	}
	let mut dispositions: Vec <OsString> = Vec::new ();
	if args.default { dispositions.push ("default".into ()); }
	if args.forced { dispositions.push ("forced".into ()); }
	if args.hearing_impaired { dispositions.push ("hearing_impaired".into ()); }
	if args.visual_impaired { dispositions.push ("visual_impaired".into ()); }
	if args.descriptions { dispositions.push ("descriptions".into ()); }
	if args.original { dispositions.push ("original".into ()); }
	if args.commentary { dispositions.push ("commentary".into ()); }
	let mut dispositions_str = OsString::new ();
	for disposition in dispositions {
		if ! dispositions_str.is_empty () { dispositions_str.push ("+"); }
		dispositions_str.push (disposition);
	}
	command.push (format! ("-disposition:s:{new_subs_idx}").into ());
	if ! dispositions_str.is_empty () {
		command.push (dispositions_str);
	} else {
		command.push ("0".into ());
	}

	// do conversion

	command.push ("-f".into ());
	command.push ("matroska".into ());

	dest_name.push (".mkv");
	let dest_path = args.source_path.with_file_name (dest_name);
	if dest_path.try_exists () ? {
		any_bail! ("File already exists: {}", dest_path.display ());
	}
	command.push ({
		let mut val = OsString::from ("file:");
		val.push (dest_path);
		val
	});

	let source_display = source_name.to_string_lossy ();
	ffmpeg::convert_progress (& source_display, duration_micros, command) ?;

    Ok (())

}
