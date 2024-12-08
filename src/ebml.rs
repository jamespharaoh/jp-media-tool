use anyhow::format_err as any_err;
use std::io::BufRead;
use std::io::Seek;

use crate::EbmlReader;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct EbmlElem {
	pub version: u64,
	pub read_version: u64,
	pub max_id_length: u64,
	pub max_size_length: u64,
	pub doc_type: String,
	pub doc_type_version: u64,
	pub doc_type_read_version: u64,
	pub doc_type_extensions: Vec <EbmlDocTypeExtElem>,
}

impl EbmlElem {

	pub fn read <Src: BufRead + Seek> (reader: & mut EbmlReader <Src>) -> anyhow::Result <Self> {
		let mut version = None;
		let mut read_version = None;
		let mut max_id_length = None;
		let mut max_size_length = None;
		let mut doc_type = None;
		let mut doc_type_version = None;
		let mut doc_type_read_version = None;
		let mut doc_type_extensions = Vec::new ();
		reader.nest ();
		while let Some ((elem_id, _)) = reader.read () ? {
			match elem_id {
				elem_ids::EBML_VERSION => version = Some (reader.unsigned () ?),
				elem_ids::EBML_READ_VERSION => read_version = Some (reader.unsigned () ?),
				elem_ids::EBML_MAX_ID_LENGTH => max_id_length = Some (reader.unsigned () ?),
				elem_ids::EBML_MAX_SIZE_LENGTH => max_size_length = Some (reader.unsigned () ?),
				elem_ids::EBML_DOC_TYPE => doc_type = Some (reader.string () ?),
				elem_ids::EBML_DOC_TYPE_VERSION => doc_type_version = Some (reader.unsigned () ?),
				elem_ids::EBML_DOC_TYPE_READ_VERSION => doc_type_read_version = Some (reader.unsigned () ?),
				elem_ids::EBML_DOC_TYPE_EXTENSION => {
					doc_type_extensions.push (EbmlDocTypeExtElem::read (reader) ?);
				},
				_ => reader.skip () ?,
			}
		}
		reader.unnest () ?;
		Ok (EbmlElem {
			version: version.ok_or_else (|| any_err! ("Missing EBMLVersion")) ?,
			read_version: read_version.ok_or_else (|| any_err! ("Missing EBMLReadVersion")) ?,
			max_id_length: max_id_length.ok_or_else (|| any_err! ("Missing EBMLMaxIDLength")) ?,
			max_size_length: max_size_length.ok_or_else (|| any_err! ("Missing EBMLMaxSizeLength")) ?,
			doc_type: doc_type.ok_or_else (|| any_err! ("Missing DocType")) ?,
			doc_type_version: doc_type_version.ok_or_else (|| any_err! ("Mising DocTypeVersion")) ?,
			doc_type_read_version: doc_type_read_version.ok_or_else (|| any_err! ("Missing DocTypeReadVersion")) ?,
			doc_type_extensions,
		})
	}

}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct EbmlDocTypeExtElem {
	pub name: String,
	pub version: u64,
}

impl EbmlDocTypeExtElem {

	pub fn read <Src: BufRead + Seek> (reader: & mut EbmlReader <Src>) -> anyhow::Result <Self> {
		let mut name = None;
		let mut version = None;
		reader.nest ();
		while let Some ((elem_id, _)) = reader.read () ? {
			match elem_id {
				elem_ids::EBML_DOC_TYPE_EXTENSION_NAME => name = Some (reader.string () ?),
				elem_ids::EBML_DOC_TYPE_EXTENSION_VERSION => version = Some (reader.unsigned () ?),
				_ => reader.skip () ?,
			}
		}
		reader.unnest () ?;
		Ok (EbmlDocTypeExtElem {
			name: name.ok_or_else (|| any_err! ("Missing DocTypeExtensionName")) ?,
			version: version.ok_or_else (|| any_err! ("Missing DocTypeExtensionVersion")) ?,
		})
	}

}

#[ allow (dead_code) ]
pub mod elem_ids {

	pub const EBML: u64 = 0x1a45dfa3;
	pub const EBML_VERSION: u64 = 0x4286;
	pub const EBML_READ_VERSION: u64 = 0x42f7;
	pub const EBML_MAX_ID_LENGTH: u64 = 0x42f2;
	pub const EBML_MAX_SIZE_LENGTH: u64 = 0x42f3;
	pub const EBML_DOC_TYPE: u64 = 0x4282;
	pub const EBML_DOC_TYPE_VERSION: u64 = 0x4287;
	pub const EBML_DOC_TYPE_READ_VERSION: u64 = 0x4285;
	pub const EBML_DOC_TYPE_EXTENSION: u64 = 0x4281;
	pub const EBML_DOC_TYPE_EXTENSION_NAME: u64 = 0x4283;
	pub const EBML_DOC_TYPE_EXTENSION_VERSION: u64 = 0x4284;

	pub const CRC_32: u64 = 0xbf;
	pub const VOID: u64 = 0xec;

}
