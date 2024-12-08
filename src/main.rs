use anyhow::format_err as any_err;
use std::fs::File;
use std::io::BufReader;

mod ebml;
mod matroska;
mod reader;

use ebml::EbmlElem;
use reader::EbmlReader;

fn main () -> anyhow::Result <()> {
	let file = BufReader::new (File::open ("test.mkv") ?);
	let mut reader = EbmlReader::new (file) ?;
	let (ebml_id, _) = reader.read () ?.ok_or_else (|| any_err! ("No ebml element")) ?;
	anyhow::ensure! (ebml_id == ebml::elem_ids::EBML, "Expected EBML, got 0x{ebml_id:x}");
	let ebml = EbmlElem::read (& mut reader) ?;
	println! ("{ebml:#?}");
	while let Some ((body_id, body_len)) = reader.read () ? {
		anyhow::ensure! (body_id == matroska::elem_ids::SEGMENT);
		println! ("Got body element id=0x{body_id:x}, len={body_len}");
		reader.nest ();
		while let Some ((elem_id, elem_len)) = reader.read () ? {
			println! ("  Got element id=0x{elem_id:x}, len={elem_len}");
			reader.skip () ?;
		}
		reader.unnest () ?;
	}
    Ok (())
}
