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
			println! ("{file_display:<max_len$}  directory");
			continue;
		}
		let mut file = BufReader::new (File::open (file_path) ?);
		match detect::FileType::identify_reader (& mut file) {
			Ok (detect::FileType::AppleVideo) => {
				println! ("{file_display:<max_len$}  apple video");
			},
			Ok (detect::FileType::Avi) => {
				println! ("{file_display:<max_len$}  audio video interleve");
			},
			Ok (detect::FileType::IsoMedia) => {
				println! ("{file_display:<max_len$}  iso media");
			},
			Ok (detect::FileType::Matroska) => {
				let info = matroska_info (& mut file)
					.unwrap_or_else (|err| format! ("error: {err}"));
				println! ("{file_display:<max_len$}  matroska {info}");
			},
			Ok (detect::FileType::Mpeg1) => {
				println! ("{file_display:<max_len$}  mpeg base media v1");
			},
			Ok (detect::FileType::Mpeg2) => {
				println! ("{file_display:<max_len$}  mpeg base media v2");
			},
			Ok (detect::FileType::Mp4v1) => {
				println! ("{file_display:<max_len$}  mpeg-4 v1");
			},
			Ok (detect::FileType::Mp4v2) => {
				println! ("{file_display:<max_len$}  mpeg-4 v2");
			},
			Err (detect::IdentifyError::PartiallyRecognised (err)) => {
				println! ("{file_display:<max_len$}  error: {err}");
			},
			Err (detect::IdentifyError::NotRecognised) => {
				println! ("{file_display:<max_len$}  unknown");
			},
			Err (detect::IdentifyError::IoError (err)) => {
				println! ("{file_display:<max_len$}  error: {err}");
			},
		}
	}
    Ok (())
}

fn fmt_size (size: u64) -> String {
	if size < 1024 {
		format! ("{size}B")
	} else if size < 1_048_576 {
		format! ("{:.2}KiB", size as f64 / 1_024.0)
	} else if size < 1_073_741_824 {
		format! ("{:.2}MiB", size as f64 / 1_048_576.0)
	} else if size < 1_099_511_627_776 {
		format! ("{:.2}GiB", size as f64 / 1_073_741_824.0)
	} else {
		format! ("{:.2}TiB", size as f64 / 1_099_511_627_776.0)
	}
}

fn matroska_info (mut file: impl BufRead + Seek) -> anyhow::Result <String> {

	let file_size = file.seek (SeekFrom::End (0)) ?;
	let mut result = fmt_size (file_size);

	let mut reader = matroska::Reader::new (file) ?;
	let info = reader.segment_info () ?;
	let tracks = reader.tracks () ?;

	if let Some (duration_ticks) = info.duration {
		let duration = (info.timestamp_scale as f64 * duration_ticks / 1_000_000_000.0) as u64;
		let _ = write! (
			& mut result,
			", {hour}:{minute:02}:{second:02}",
			hour = duration / 3600,
			minute = duration / 60 % 60,
			second = duration % 60);
	}

	let Some (video_track) = tracks.entries.iter ()
			.find (|track| track.track_type == 1)
		else { any_bail! ("No video track") };
	let Some (video_track_video) = video_track.video.as_ref ()
		else { any_bail! ("Video track details missing") };

	let _ = write! (
		& mut result,
		", {codec} {width}×{height}",
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
