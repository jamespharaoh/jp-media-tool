use crate::ebml;
use crate::imports::*;
use crate::matroska;

/*
LOGIC:
- scan all tracks
- identify which to keep and discard
- construct ffmpeg command
VIDEO:
- just use first video track
AUDIO:
- keep if english, spanish, original
- convert to opus if not already
SUBTITLES:
- keep if english, spanish, original
- discard images (text only)
*/

#[ derive (Debug, clap::Args) ]
pub struct Args {

	#[ clap (name = "FILE") ]
	files: Vec <PathBuf>,

	#[ clap (long) ]
	video_quality: Option <i64>,

}

pub fn invoke (args: Args) -> anyhow::Result <()> {
	for file_path in & args.files {
		if ! invoke_one (& args, file_path) ? { break }
	}
	Ok (())
}

fn invoke_one (args: & Args, file_path: & Path) -> anyhow::Result <bool> {
	println! ("{}", file_path.display ());
	let Some (file_name) = file_path.file_name () else {
		any_bail! ("Specified file has no name");
	};

	let file = BufReader::new (File::open (file_path) ?);
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
	eprintln! ("{seek_tracks:#?}");
	reader.jump (segment_pos + seek_tracks.position) ?;
	let Some ((tracks_id, _, _)) = reader.read () ? else {
		any_bail! ("Error reading tracks");
	};
	anyhow::ensure! (
		tracks_id == matroska::elems::TRACKS,
		"Expected Tracks, got 0x{tracks_id}");
	let tracks = matroska::TracksElem::read (& mut reader) ?;
	eprintln! ("{tracks:#?}");

	let mut command: Vec <OsString> = Vec::new ();
	command.push ("ffmpeg".into ());
	command.push ("-i".into ());
	command.push (file_path.into ());

	// do video

	let video_tracks: Vec <_> =
		tracks.entries.iter ()
			.filter (|track| track.track_type == 1)
			.collect ();
	let Some (_video_track) = video_tracks.get (0) else {
		any_bail! ("No video tracks found");
	};

	if let Some (& video_quality) = args.video_quality.as_ref () {
		command.push ("-map".into ());
		command.push ("0:v:0".into ());
		command.push ("-c:v:0".into ());
		command.push ("libx265".into ());
		command.push ("-preset:v:0".into ());
		command.push ("medium".into ());
		command.push ("-crf:v:0".into ());
		command.push (format! ("{video_quality}").into ());
		command.push ("-pix_fmt:v:0".into ());
		command.push ("yuv420p10le".into ());
		command.push ("-map_metadata:s:v:0".into ());
		command.push ("0:s:v:0".into ());
	} else {
		command.push ("-map".into ());
		command.push ("0:v:0".into ());
		command.push ("-c:v:0".into ());
		command.push ("copy".into ());
		command.push ("-map_metadata:s:v:0".into ());
		command.push ("0:s:v:0".into ());
	}

	// do audio

	let audio_tracks: Vec <_> =
		tracks.entries.iter ()
			.filter (|track| track.track_type == 2)
			.collect ();
	let mut dest_idx = 0;
	for (src_idx, & track) in audio_tracks.iter ().enumerate () {
		let Some (track_audio) = track.audio.as_ref () else {
			any_bail! ("Audio track has no audio settings");
		};
		if ! matches! (track.language.as_str (), "eng" | "esp") && track.flag_original != Some (true) {
			eprintln! ("Skip audio track {src_idx} ({})", track_meta (track));
			continue;
		}
		eprintln! ("Include audio track {src_idx} ({})", track_meta (track));
		if track.codec_id == "A_OPUS" {
			command.push ("-map".into ());
			command.push (format! ("0:a:{src_idx}").into ());
			command.push (format! ("-c:a:{dest_idx}").into ());
			command.push ("copy".into ());
			command.push (format! ("-map_metadata:s:a:{dest_idx}").into ());
			command.push (format! ("0:s:a:{src_idx}").into ());
		} else {
			command.push ("-map".into ());
			command.push (format! ("0:a:{src_idx}").into ());
			command.push (format! ("-c:a:{dest_idx}").into ());
			command.push ("libopus".into ());
			command.push (format! ("-b:a:{dest_idx}").into ());
			command.push (match track_audio.channels {
				1 => "64k",
				2 => "128k",
				6 => "256k",
				8 => "320k",
				_ => any_bail! ("Unable to map {} audio channels", track_audio.channels),
			}.into ());
			command.push (format! ("-map_metadata:s:a:{dest_idx}").into ());
			command.push (format! ("0:s:a:{src_idx}").into ());
		}
		dest_idx += 1;
	}

	// do subtitles

	let subtitle_tracks: Vec <_> =
		tracks.entries.iter ()
			.filter (|track| track.track_type == 17)
			.collect ();
	let mut dest_idx = 0;
	for (src_idx, & track) in subtitle_tracks.iter ().enumerate () {
		if ! track.codec_id.starts_with ("S_TEXT/") {
			eprintln! ("Skip subtitle track {src_idx} ({})", track.codec_id);
			continue;
		}
		if ! matches! (track.language.as_str (), "eng" | "esp") && track.flag_original != Some (true) {
			eprintln! ("Skip subtitle track {src_idx} ({})", track_meta (track));
			continue;
		}
		eprintln! ("Include subtitle track {src_idx} ({})", track_meta (track));
		command.push ("-map".into ());
		command.push (format! ("0:s:{src_idx}").into ());
		command.push (format! ("-c:s:{dest_idx}").into ());
		command.push ("copy".into ());
		command.push (format! ("-map_metadata:s:s:{dest_idx}").into ());
		command.push (format! ("0:s:s:{src_idx}").into ());
		dest_idx += 1;
	}

	// do output

	let mut file_name_out = file_path.file_stem ().unwrap ().to_owned ();
	if args.video_quality.is_some () {
		file_name_out.push ("-x265-opus");
	} else {
		file_name_out.push ("-opus");
	}
	file_name_out.push (".mkv");
	let file_path_out = file_path.with_file_name (file_name_out);
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
			eprintln! ("Encoder process returned status {:?}", code);
		} else {
			eprintln! ("Encoder process terminated abnormally");
		}
		return Ok (false)
	}

    Ok (true)

}

fn track_meta (track: & matroska::tracks::TrackEntryElem) -> String {
	let mut result = String::new ();
	result.push_str (& track.language);
	if ! track.flag_enabled { result.push_str (", disabled"); }
	if ! track.flag_default { result.push_str (", non-default"); }
	if track.flag_forced { result.push_str (", forced"); }
	if track.flag_hearing_impaired.unwrap_or (false) { result.push_str (", hearing impaired"); }
	if track.flag_visual_impaired.unwrap_or (false) { result.push_str (", visual impaired"); }
	if track.flag_text_descriptions.unwrap_or (false) { result.push_str (", text_descriptions"); }
	if track.flag_original.unwrap_or (false) { result.push_str (", original"); }
	if track.flag_commentary.unwrap_or (false) { result.push_str (", commentary"); }
	result
}
