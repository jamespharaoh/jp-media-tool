use crate::ebml;
use crate::imports::*;
use crate::matroska;

#[ derive (Debug, clap::Args) ]
#[ command (about = "Display detailed information about a matroska (mkv) media file" )]
pub struct Args {

	#[ clap (name = "FILE", help = "Files to show information about") ]
	files: Vec <PathBuf>,

	#[ clap (long, help = "Show information in chapters element") ]
	show_chapters: bool,

	#[ clap (long, help = "Show information in cluster elements") ]
	show_clusters: bool,

	#[ clap (long, help = "Show information in cues element") ]
	show_cues: bool,

	#[ clap (long, help = "Show details about EBML header element") ]
	show_head: bool,

	#[ clap (long, help = "Show information in segment info elements") ]
	show_info: bool,

	#[ clap (long, help = "Show information in tags element") ]
	show_tags: bool,

	#[ clap (long, help = "Show information in tracks element") ]
	show_tracks: bool,

	#[ clap (long, help = "Show all information (generates a lot of text)") ]
	show_all: bool,

}

pub fn invoke (args: Args) -> anyhow::Result <()> {

	for file_path in & args.files {
		println! ("{}", file_path.display ());
		let file = BufReader::new (File::open (file_path) ?);
		let mut reader = EbmlReader::new (file) ?;

		let (ebml_id, _, _) = reader.read () ?.ok_or_else (|| any_err! ("No ebml element")) ?;
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

		while let Some ((segment_id, _, _)) = reader.read () ? {
			anyhow::ensure! (segment_id == matroska::elems::SEGMENT);
			reader.nest ();
			while let Some ((elem_id, elem_pos, elem_len)) = reader.read () ? {
				match elem_id {
					matroska::elems::SEEK_HEAD => {
						if args.show_head || args.show_all {
							println! ("Got seek head: start=0x{elem_pos:x}, len={elem_len}");
							let seek_head = matroska::SeekHeadElem::read (& mut reader) ?;
							println! ("{seek_head:#?}");
						} else {
							reader.skip () ?;
						}
					},
					matroska::elems::INFO => {
						if args.show_info || args.show_all {
							println! ("Got segment info: start=0x{elem_pos:x}, len={elem_len}");
							let info = matroska::InfoElem::read (& mut reader) ?;
							println! ("{info:#?}");
						} else {
							reader.skip () ?;
						}
					},
					matroska::elems::TRACKS => {
						if args.show_tracks || args.show_all {
							println! ("Got tracks: start=0x{elem_pos:x}, len={elem_len}");
							let tracks = matroska::TracksElem::read (& mut reader) ?;
							println! ("{tracks:#?}");
						} else {
							reader.skip () ?;
						}
					},
					matroska::elems::CHAPTERS => {
						if args.show_chapters || args.show_all {
							println! ("Got chapters: start=0x{elem_pos:x}, len={elem_len}");
							let chapters = matroska::ChaptersElem::read (& mut reader) ?;
							println! ("{chapters:#?}");
						} else {
							reader.skip () ?;
						}
					},
					matroska::elems::TAGS => {
						if args.show_tags || args.show_all {
							println! ("Got tags: start=0x{elem_pos:x}, len={elem_len}");
							let tags = matroska::TagsElem::read (& mut reader) ?;
							println! ("{tags:#?}");
						} else {
							reader.skip () ?;
						}
					},
					matroska::elems::CLUSTER => {
						if args.show_clusters || args.show_all {
							println! ("Got cluster: start=0x{elem_pos:x}, len={elem_len}");
							let cluster = matroska::ClusterElem::read (& mut reader) ?;
							println! ("{cluster:#?}");
						} else {
							reader.skip () ?;
						}
					},
					matroska::elems::CUES => {
						if args.show_cues || args.show_all {
							println! ("Got cues: start=0x{elem_pos:x}, len={elem_len}");
							let cues = matroska::CuesElem::read (& mut reader) ?;
							println! ("{cues:#?}");
						} else {
							reader.skip () ?;
						}
					},
					ebml::head::elems::CRC32 | ebml::head::elems::VOID => {
						reader.skip () ?;
					},
					_ => {
						println! ("Skipped: id=0x{elem_id:x}, pos=0x{elem_pos:x}, len={elem_len}");
						reader.skip () ?;
					},
				}
			}
			reader.unnest () ?;
		}

	}

    Ok (())

}
