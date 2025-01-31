use crate::detect;
use crate::imports::*;
use crate::matroska;

#[ derive (Debug, clap::Args) ]
#[ command (about = "Display summary information about a list of media files" )]
pub struct Args {

	#[ clap (name = "FILE", help = "Files to show information about") ]
	files: Vec <PathBuf>,

}

pub fn invoke (args: Args) -> anyhow::Result <()> {
	let max_len =
		args.files.iter ()
			.map (|file_path| file_path.to_string_lossy ().chars ().count ())
			.max ()
			.unwrap_or (0);
	for file_path in & args.files {
		let file_display = file_path.display ();
		if file_path.is_dir () {
			println! ("{file_display:<max_len$} directory");
			continue;
		}
		match detect::FileType::identify (& file_path) {
			Ok (detect::FileType::Avi) => {
				println! ("{file_display:<max_len$} audio video interleve");
			},
			Ok (detect::FileType::IsoMedia) => {
				println! ("{file_display:<max_len$} iso media");
			},
			Ok (detect::FileType::Matroska) => {
				let info = matroska_info (file_path)
					.unwrap_or_else (|err| format! ("error: {err}"));
				println! ("{file_display:<max_len$} matroska {info}");
			},
			Ok (detect::FileType::Mpeg1) => {
				println! ("{file_display:<max_len$} mpeg base media v1");
			},
			Ok (detect::FileType::Mpeg2) => {
				println! ("{file_display:<max_len$} mpeg base media v2");
			},
			Ok (detect::FileType::Mp4v1) => {
				println! ("{file_display:<max_len$} mpeg-4 v1");
			},
			Ok (detect::FileType::Mp4v2) => {
				println! ("{file_display:<max_len$} mpeg-4 v2");
			},
			Err (detect::IdentifyError::PartiallyRecognised (err)) => {
				println! ("{file_display:<max_len$} error: {err}");
			},
			Err (detect::IdentifyError::NotRecognised) => {
				println! ("{file_display:<max_len$} unknown");
			},
			Err (detect::IdentifyError::IoError (err)) => {
				println! ("{file_display:<max_len$} error: {err}");
			},
		}
	}
    Ok (())
}

fn matroska_info (path: & Path) -> anyhow::Result <String> {

	let file = BufReader::new (File::open (path) ?);
	let mut reader = matroska::Reader::new (file) ?;
	let tracks = reader.tracks () ?;

	let Some (video_track) = tracks.entries.iter ()
			.find (|track| track.track_type == 1)
		else { any_bail! ("No video track") };
	let Some (video_track_video) = video_track.video.as_ref ()
		else { any_bail! ("Video track details missing") };

	let mut result = format! (
		"{codec} {width}×{height}",
		codec = matroska_codec_name (& video_track.codec_id),
		width = video_track_video.pixel_width,
		height = video_track_video.pixel_height);

	for audio_track in tracks.entries.iter ()
			.filter (|track| track.track_type == 2) {
		result.push_str (& format! (
			", {codec}",
			codec = matroska_codec_name (& audio_track.codec_id)));
	}

	let num_subs = tracks.entries.iter ()
		.filter (|track| track.track_type == 17)
		.count ();
	if 1 == num_subs {
		result.push_str (", subs");
	} else if 1 < num_subs {
		result.push_str (& format! (", {num_subs} subs"));
	}

	Ok (result)

}

fn matroska_codec_name (codec_id: & str) -> & str {
	match codec_id {
		"A_AAC" => "aac",
		"A_AC3" => "ac3",
		"A_DTS" => "dts",
		"A_EAC3" => "eac3",
		"A_FLAC" => "flac",
		"A_MPEG/L3" => "mp3",
		"A_OPUS" => "opus",
		"A_TRUEHD" => "truehd",
		"V_AV1" => "av1",
		"V_MPEG2" => "mpeg2",
		"V_MPEG4/ISO/ASP" => "mpeg4/asp",
		"V_MPEG4/ISO/AVC" => "mpeg4/avc",
		"V_MPEGH/ISO/HEVC" => "mpeg4/hevc",
		_ => codec_id,
	}
}
