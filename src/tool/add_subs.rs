use crate::ebml;
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

	let Some (_source_name) = args.source_path.file_name () else {
		any_bail! ("Specified file has no name: {}", args.source_path.display ());
	};
	println! ("{}", args.source_path.display ());

	let file = BufReader::new (File::open (& args.source_path) ?);
	let mut reader = EbmlReader::new (file) ?;

	// read ebml header

	let Some ((ebml_id, _, _)) = reader.read () ? else {
		any_bail! ("Error reading ebml header");
	};
	anyhow::ensure! (ebml_id == ebml::head::elems::EBML, "Expected EBML, got 0x{ebml_id:x}");
	let ebml = ebml::head::EbmlElem::read (& mut reader) ?;
	if 1 < ebml.read_version {
		any_bail! ("Unsupported EBML read version: {}", ebml.read_version);
	}
	if & ebml.doc_type != "matroska" {
		any_bail! ("Unsupported document type: {} (expected: matroska)", ebml.doc_type);
	}
	if 4 < ebml.doc_type_read_version {
		any_bail! ("Unsupported matroska read version: {}", ebml.doc_type_read_version);
	}

	// read segment

	let Some ((segment_id, _, _)) = reader.read () ? else {
		any_bail! ("Error reading segment");
	};
	let segment_pos = reader.position ();
	anyhow::ensure! (
		segment_id == matroska::elems::SEGMENT,
		"Expected Segment, got 0x{segment_id}");
	reader.nest ();

	// read seek head

	let Some ((seek_head_id, _, _)) = reader.read () ? else {
		any_bail! ("Error reading seek head");
	};
	anyhow::ensure! (
		seek_head_id == matroska::elems::SEEK_HEAD,
		"Expected SeekHead, got 0x{seek_head_id}");
	let seek_head = matroska::SeekHeadElem::read (& mut reader) ?;

	// read tracks

	let Some (seek_tracks) =
		seek_head.seeks.iter ()
			.find (|seek| seek.id == matroska::elems::TRACKS)
	else { any_bail! ("Tracks not found in seek head") };
	reader.jump (segment_pos + seek_tracks.position) ?;
	let Some ((tracks_id, _, _)) = reader.read () ? else {
		any_bail! ("Error reading tracks");
	};
	anyhow::ensure! (
		tracks_id == matroska::elems::TRACKS,
		"Expected Tracks, got 0x{tracks_id}");
	let tracks = matroska::TracksElem::read (& mut reader) ?;

	let mut command: Vec <OsString> = Vec::new ();
	command.push ("ffmpeg".into ());
	command.push ("-hide_banner".into ());
	command.push ("-i".into ());
	command.push ((& args.source_path).into ());
	command.push ("-i".into ());
	command.push (args.subs_path.into ());

	let mut file_name_out = args.source_path.file_stem ().unwrap ().to_owned ();
	file_name_out.push (format! ("-subs-{}", args.lang));

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

	file_name_out.push (".mkv");
	let file_path_out = args.source_path.with_file_name (file_name_out);
	command.push (file_path_out.into ());

	let mut proc =
		process::Command::new (command [0].clone ())
			.args (& command [1 .. ])
			.stdin (process::Stdio::null ())
			.stdout (process::Stdio::inherit ())
			.stderr (process::Stdio::inherit ())
			.spawn ()
			.unwrap ();
	let result = proc.wait ().unwrap ();
	if ! result.success () {
		if let Some (code) = result.code () {
			any_bail! ("Encoder process returned status {:?}", code);
		} else {
			any_bail! ("Encoder process terminated abnormally");
		}
	}

    Ok (())

}
