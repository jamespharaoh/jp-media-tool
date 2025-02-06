use crate::ffmpeg;
use crate::imports::*;
use crate::matroska;

#[ derive (Debug, clap::Args) ]
#[ command (about = "Interactively edit metadata and tracks for matroska file" )]
pub struct Args {

	#[ clap (name = "FILE", help = "File to edit") ]
	file: PathBuf,

}

pub fn invoke (args: Args) -> anyhow::Result <()> {

	let (temp, duration_micros) = write_temp (& args) ?;

	let nano_status =
		process::Command::new ("nano")
			.arg (temp.path ())
			.status () ?;
	if nano_status.code () != Some (0) {
		any_bail! ("Editor did not exit cleanly, aborting");
	}

	perform_edits (& args, temp, duration_micros) ?;

	Ok (())

}

fn write_temp (args: & Args) -> anyhow::Result <(tempfile::NamedTempFile, Option <u64>)> {

	let file = BufReader::new (File::open (& args.file) ?);
	let mut reader = matroska::Reader::new (file) ?;
	let file_info = reader.segment_info () ?;
	let duration_micros = file_info.duration
		.map (|duration| (file_info.timestamp_scale as f64 * duration / 1000.0) as u64);
	let file_tracks = reader.tracks () ?;
	let file_tags = reader.tags () ?;

	let mut temp_value = serde_yaml::Mapping::new ();
	temp_value.insert ("title".into (),
		file_info.title.as_ref ().map (String::as_str).unwrap_or_default ().into ());
	let mut temp_tags = serde_yaml::Mapping::new ();
	for tag in & file_tags.tags {
		if ! tag.targets.track_uids.is_empty ()
				|| ! tag.targets.edition_uids.is_empty ()
				|| ! tag.targets.chapter_uids.is_empty ()
				|| ! tag.targets.attachment_uids.is_empty () {
			continue;
		}
		for simple_tag in & tag.simple_tags {
			let Some (string) = simple_tag.string.as_ref () else { continue };
			temp_tags.insert (simple_tag.name.as_str ().into (), string.as_str ().into ());
		}
	}
	temp_value.insert ("tags".into (), temp_tags.into ());

	let mut tracks = serde_yaml::Mapping::new ();
	for (track_type_label, track_type) in [ ("video", 1), ("audio", 2), ("subs", 17) ] {
		for (track_idx, file_track) in file_tracks.entries.iter ()
				.filter (|track| track.track_type == track_type)
				.enumerate () {
			let mut track = serde_yaml::Mapping::new ();
			track.insert ("title".into (),
				file_track.name.as_ref ().map (String::as_str).unwrap_or_default ().into ());
			track.insert ("codec".into (), file_track.codec_id.as_str ().into ());
			track.insert ("language".into (), file_track.language.as_str ().into ());
			let mut tags = serde_yaml::Mapping::new ();
			for tag in & file_tags.tags {
				if ! tag.targets.track_uids.contains (& file_track.uid) { continue }
				for simple_tag in & tag.simple_tags {
					let Some (string) = simple_tag.string.as_ref () else { continue };
					tags.insert (simple_tag.name.as_str ().into (), string.as_str ().into ());
				}
			}
			track.insert ("tags".into (), tags.into ());
			tracks.insert (format! ("{track_type_label}-{track_idx}").into (), track.into ());
		}
	}
	temp_value.insert ("tracks".into (), tracks.into ());

	let temp_value = serde_yaml::Value::from (temp_value);
	let mut temp = tempfile::NamedTempFile::with_prefix ("jp-media-tool-edit-") ?;
	serde_yaml::to_writer (BufWriter::new (& mut temp), & temp_value) ?;
	temp.flush () ?;

	Ok ((temp, duration_micros))

}

fn perform_edits (
	args: & Args,
	mut temp: tempfile::NamedTempFile,
	duration_micros: Option <u64>,
) -> anyhow::Result <()> {

	let format_err = || any_err! ("Format error");

	temp.seek (SeekFrom::Start (0)) ?;
	let temp_value: serde_yaml::Value = serde_yaml::from_reader (BufReader::new (& mut temp)) ?;
	let temp_value = temp_value.as_mapping ().ok_or_else (format_err) ?;

	let mut command: Vec <OsString> = Vec::new ();
	command.push ("-i".into ());
	command.push ({
		let mut val = OsString::from ("file:");
		val.push (& args.file);
		val
	});
	command.push ("-map_metadata".into ());
	command.push ("-1".into ());
	let temp_title = temp_value.get ("title").ok_or_else (format_err) ?;
	let temp_title = temp_title.as_str ().ok_or_else (format_err) ?;
	command.push ("-metadata".into ());
	command.push (format! ("title={temp_title}").into ());
	if let Some (temp_tags) = temp_value.get ("tags") {
		let temp_tags = temp_tags.as_mapping ().ok_or_else (format_err) ?;
		for (tag_name, tag_value) in temp_tags {
			let tag_name = tag_name.as_str ().ok_or_else (format_err) ?;
			let tag_value = tag_value.as_str ().ok_or_else (format_err) ?;
			command.push ("-metadata".into ());
			command.push (format! ("{tag_name}={tag_value}").into ());
		}
	}

	let temp_tracks = temp_value.get ("tracks").ok_or_else (format_err) ?;
	let temp_tracks = temp_tracks.as_mapping ().ok_or_else (format_err) ?;
	let mut num_video = 0;
	let mut num_audio = 0;
	let mut num_subs = 0;
	for (track_id, temp_track) in temp_tracks {
		let track_id = track_id.as_str ().ok_or_else (format_err) ?;
		let track_id_out;
		if let Some (id) = track_id.strip_prefix ("video-") {
			command.push ("-map".into ());
			command.push (format! ("0:v:{id}").into ());
			track_id_out = format! ("v:{num_video}");
			num_video += 1;
		} else if let Some (id) = track_id.strip_prefix ("audio-") {
			command.push ("-map".into ());
			command.push (format! ("0:a:{id}").into ());
			track_id_out = format! ("a:{num_audio}");
			num_audio += 1;
		} else if let Some (id) = track_id.strip_prefix ("audio-") {
			command.push ("-map".into ());
			command.push (format! ("0:s:{id}").into ());
			track_id_out = format! ("s:{num_subs}");
			num_subs += 1;
		} else { return Err (format_err ()) }
		command.push (format! ("-codec:{track_id_out}").into ());
		command.push ("copy".into ());
		let temp_lang = temp_track.get ("language").ok_or_else (format_err) ?;
		let temp_lang = temp_lang.as_str ().ok_or_else (format_err) ?;
		if ! temp_lang.is_empty () {
			command.push (format! ("-metadata:s:{track_id_out}").into ());
			command.push (format! ("language={temp_lang}").into ());
		}
		if let Some (temp_tags) = temp_track.get ("tags") {
			let temp_tags = temp_tags.as_mapping ().ok_or_else (format_err) ?;
			for (tag_name, tag_value) in temp_tags {
				let tag_name = tag_name.as_str ().ok_or_else (format_err) ?;
				let tag_value = tag_value.as_str ().ok_or_else (format_err) ?;
				command.push (format! ("-metadata:s:{track_id_out}").into ());
				command.push (format! ("{tag_name}={tag_value}").into ());
			}
		}
	}

	let dest_file = {
		let mut val = args.file.file_stem ().unwrap ().to_owned ();
		val.push ("-edit.mkv");
		PathBuf::from (val)
	};
	if fs::exists (& dest_file) ? {
		any_bail! ("Destination file exists: {}", dest_file.display ());
	}

	command.push ("-f".into ());
	command.push ("matroska".into ());
	command.push ({
		let mut val = OsString::from ("file:");
		val.push (& dest_file);
		val
	});
	let file_display = args.file.to_string_lossy ();
eprintln! ("{command:#?}");
	ffmpeg::convert_progress (& file_display, duration_micros, command) ?;

	Ok (())

}
