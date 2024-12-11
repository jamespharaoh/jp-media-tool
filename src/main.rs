mod ebml;
mod element;
mod imports;
mod matroska;
mod reader;

use crate::imports::*;

fn main () -> anyhow::Result <()> {
	let file = BufReader::new (File::open ("test.mkv") ?);
	let mut reader = EbmlReader::new (file) ?;
	let (ebml_id, ebml_pos, ebml_len) = reader.read () ?.ok_or_else (|| any_err! ("No ebml element")) ?;
	anyhow::ensure! (ebml_id == ebml::elems::EBML, "Expected EBML, got 0x{ebml_id:x}");
	println! ("Got ebml: start=0x{ebml_pos:x}, len={ebml_len}");
	let ebml = ebml::EbmlElem::read (& mut reader) ?;
	println! ("{ebml:#?}");
	if 1 < ebml.read_version {
		println! ("WARNING: Unsupported EBML read version: {}", ebml.read_version);
	}
	if & ebml.doc_type != "matroska" {
		any_bail! ("Unsupported document type: {} (expected: matroska)", ebml.doc_type);
	}
	if 4 < ebml.doc_type_read_version {
		println! ("WARNING: Unsupported matroska read version: {}", ebml.doc_type_read_version);
	}
	while let Some ((segment_id, segment_pos, segment_len)) = reader.read () ? {
		anyhow::ensure! (segment_id == matroska::elems::SEGMENT);
		println! ("Got segment: start=0x{segment_pos:x}, len={segment_len}");
		reader.nest ();
		while let Some ((elem_id, elem_pos, elem_len)) = reader.read () ? {
			match elem_id {
				matroska::elems::SEEK_HEAD => {
					println! ("Got seek head: start=0x{elem_pos:x}, len={elem_len}");
					let seek_head = matroska::SeekHeadElem::read (& mut reader) ?;
					println! ("{seek_head:#?}");
				},
				matroska::elems::INFO => {
					println! ("Got segment info: start=0x{elem_pos:x}, len={elem_len}");
					let info = matroska::InfoElem::read (& mut reader) ?;
					println! ("{info:#?}");
				},
				matroska::elems::TRACKS => {
					println! ("Got tracks: start=0x{elem_pos:x}, len={elem_len}");
					let tracks = matroska::TracksElem::read (& mut reader) ?;
					println! ("{tracks:#?}");
				},
				matroska::elems::CHAPTERS => {
					println! ("Got chapters: start=0x{elem_pos:x}, len={elem_len}");
					let chapters = matroska::ChaptersElem::read (& mut reader) ?;
					println! ("{chapters:#?}");
				},
				matroska::elems::TAGS => {
					println! ("Got tags: start=0x{elem_pos:x}, len={elem_len}");
					let tags = matroska::TagsElem::read (& mut reader) ?;
					println! ("{tags:#?}");
				},
				matroska::elems::CUES => {
					println! ("Got cues: start=0x{elem_pos:x}, len={elem_len}");
					let cues = matroska::CuesElem::read (& mut reader) ?;
					println! ("{cues:#?}");
				},
				_ => {
					println! ("Skipped: id=0x{elem_id:x}, pos=0x{elem_pos:x}, len={elem_len}");
					reader.skip () ?;
				},
			}
		}
		reader.unnest () ?;
	}
    Ok (())
}
