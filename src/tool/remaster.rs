use crate::ffmpeg;
use crate::imports::*;
use crate::matroska;

#[ derive (Debug, clap::Args) ]
#[ command (about = "Reencode video as x265 (optionally) and audio as opus" )]
pub struct Args {

	#[ clap (name = "FILE", help = "Files to remaster") ]
	files: Vec <PathBuf>,

	#[ clap (long, default_value_t, help = "Preset for x265 encoder") ]
	video_preset: VideoPreset,

	#[ clap (long, value_parser = clap::value_parser! (i32).range (0 ..= 51),
		help = "Video quality setting (CRF) for x265 encoder (lower is better)") ]
	video_quality: Option <i32>,

	#[ clap (long, value_parser = clap::value_parser! (i32).range (1 ..= 10),
		help = "Apply denoise filter, value controls luma and chroma spatial parameter") ]
	video_denoise: Option <i32>,

	#[ clap (long, help = "Apply deinterlace filter" ) ]
	video_deinterlace: bool,

	#[ clap (long, help = "Apply deshake filter" ) ]
	video_deshake: bool,

	#[ clap (long, help = "Show more detailed information" ) ]
	verbose: bool,

}

#[derive (Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum) ]
enum VideoPreset {
	Ultrafast,
	Superfast,
	Veryfast,
	Faster,
	Fast,
	#[ default ]
	Medium,
	Slow,
	Slower,
	Veryslow,
	Placebo,
}

impl fmt::Display for VideoPreset {
	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		let val = match * self {
			Self::Ultrafast => "ultrafast",
			Self::Superfast => "superfast",
			Self::Veryfast => "veryfast",
			Self::Faster => "faster",
			Self::Fast => "fast",
			Self::Medium => "medium",
			Self::Slow => "slow",
			Self::Slower => "slower",
			Self::Veryslow => "veryslow",
			Self::Placebo => "placebo",
		};
		fmtr.write_str (val)
	}
}

pub fn invoke (args: Args) -> anyhow::Result <()> {
	for file_path in & args.files {
		if ! invoke_one (& args, file_path) ? { break }
	}
	Ok (())
}

fn invoke_one (args: & Args, file_path: & Path) -> anyhow::Result <bool> {

	let Some (file_name) = file_path.file_name () else {
		any_bail! ("Specified file has no name: {}", file_path.display ());
	};
	if file_name.as_encoded_bytes ().windows (10)
				.any (|sub| sub == b"-remaster-" || sub == b"-remaster.")
			&& file_name.as_encoded_bytes ().ends_with (b".mkv") {
		return Ok (true);
	}
	if args.verbose { eprintln! ("Analysing source file: {}", file_path.display ()); }

	// read source file

	let file = BufReader::new (File::open (file_path) ?);
	let mut reader = matroska::Reader::new (file) ?;
	let info = reader.segment_info () ?;
	let duration_micros = info.duration.map (|duration| (info.timestamp_scale as f64 * duration / 1000.0) as u64);
	let tracks = reader.tracks () ?;

	// start building command

	let mut command: Vec <OsString> = Vec::new ();
	command.push ("-i".into ());
	command.push (file_path.into ());

	let mut dest_name = file_path.file_stem ().unwrap ().to_owned ();
	dest_name.push ("-remaster");

	// do video

	let video_tracks: Vec <_> =
		tracks.entries.iter ()
			.filter (|track| track.track_type == 1)
			.collect ();
	let Some (_video_track) = video_tracks.get (0) else {
		any_bail! ("No video tracks found");
	};

	if let Some (& video_quality) = args.video_quality.as_ref () {
		let video_preset = & args.video_preset;
		dest_name.push (format! ("-x265-{video_preset}-{video_quality}"));
		command.push ("-map".into ());
		command.push ("0:v:0".into ());
		command.push ("-c:v:0".into ());
		command.push ("libx265".into ());
		command.push ("-preset:v:0".into ());
		command.push (format! ("{video_preset}").into ());
		command.push ("-crf:v:0".into ());
		command.push (format! ("{video_quality}").into ());
		command.push ("-pix_fmt:v:0".into ());
		command.push ("yuv420p10le".into ());
		command.push ("-map_metadata:s:v:0".into ());
		command.push ("0:s:v:0".into ());
		let mut video_filters: Vec <OsString> = Vec::new ();
		if args.video_deinterlace {
			dest_name.push ("-deinterlace");
			video_filters.push ("yadif=0".into ());
		}
		if let Some (& video_denoise) = args.video_denoise.as_ref () {
			dest_name.push (format! ("-denoise-{video_denoise}"));
			video_filters.push (format! ("hqdn3d={video_denoise}:{video_denoise}:6:6").into ());
		}
		if args.video_deshake {
			dest_name.push ("-deshake");
			video_filters.push ("deshake".into ());
		}
		if ! video_filters.is_empty () {
			command.push ("-filter:v:0".into ());
			let mut video_filters_param = OsString::new ();
			for video_filter in video_filters {
				if ! video_filters_param.is_empty () {
					video_filters_param.push (", ");
				}
				video_filters_param.push (video_filter);
			}
			command.push (video_filters_param);
		}
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
	let mut audio_mappings = Vec::new ();

	for (src_idx, & track) in audio_tracks.iter ().enumerate () {
		if ! matches! (track.language.as_str (), "eng" | "esp")
				&& track.flag_original != Some (true) {
			if args.verbose { eprintln! ("Skip audio track {src_idx} ({})", track_meta (track)); }
			continue;
		}
		if args.verbose { eprintln! ("Include audio track {src_idx} ({})", track_meta (track)); }
		audio_mappings.push (src_idx);
	}

	if audio_mappings.is_empty () {
		if args.verbose { eprintln! ("Adding first audio track as none were selected"); }
		audio_mappings.push (0);
	}

	for (dest_idx, & src_idx) in audio_mappings.iter ().enumerate () {
		let track = & audio_tracks [src_idx];
		let Some (track_audio) = track.audio.as_ref () else {
			any_bail! ("Audio track {src_idx} has no audio settings");
		};
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
				4 => "224k",
				5 => "256k",
				6 => "256k",
				8 => "320k",
				_ => any_bail! ("Unable to map {} audio channels", track_audio.channels),
			}.into ());
			if track_audio.channels == 6 {
				command.push (format! ("-filter:a:{dest_idx}").into ());
				command.push ("channelmap=channel_layout=5.1".into ());
			}
			command.push (format! ("-map_metadata:s:a:{dest_idx}").into ());
			command.push (format! ("0:s:a:{src_idx}").into ());
		}
	}

	// do subtitles

	let subtitle_tracks: Vec <_> =
		tracks.entries.iter ()
			.filter (|track| track.track_type == 17)
			.collect ();
	let mut dest_idx = 0;
	for (src_idx, & track) in subtitle_tracks.iter ().enumerate () {
		if ! track.codec_id.starts_with ("S_TEXT/") {
			if args.verbose { eprintln! ("Skip subtitle track {src_idx} ({})", track.codec_id); }
			continue;
		}
		if args.verbose { eprintln! ("Include subtitle track {src_idx} ({})", track_meta (track)); }
		command.push ("-map".into ());
		command.push (format! ("0:s:{src_idx}").into ());
		command.push (format! ("-c:s:{dest_idx}").into ());
		command.push ("copy".into ());
		command.push (format! ("-map_metadata:s:s:{dest_idx}").into ());
		command.push (format! ("0:s:s:{src_idx}").into ());
		dest_idx += 1;
	}

	// do conversion

	dest_name.push ("-opus.mkv");
	let dest_path = file_path.with_file_name (dest_name);
	if dest_path.try_exists () ? {
		any_bail! ("File already exists: {}", dest_path.display ());
	}
	command.push (dest_path.into ());

	let file_display = file_name.to_string_lossy ();
	ffmpeg::convert_progress (& file_display, duration_micros, command) ?;

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
