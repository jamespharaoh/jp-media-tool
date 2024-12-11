use crate::imports::*;

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

impl EbmlElement for EbmlElem {
	ebml_elem_read! {
		spec = elems::Ebml;
		one req version = elems::EbmlVersion;
		one req read_version = elems::EbmlReadVersion;
		one req max_id_length = elems::EbmlMaxIdLength;
		one req max_size_length = elems::EbmlMaxSizeLength;
		one req doc_type = elems::EbmlDocType;
		one req doc_type_version = elems::EbmlDocTypeVersion;
		one req doc_type_read_version = elems::EbmlDocTypeReadVersion;
		mul opt doc_type_extensions = elems::EbmlDocTypeExtension;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct EbmlDocTypeExtElem {
	pub name: String,
	pub version: u64,
}

impl EbmlElement for EbmlDocTypeExtElem {
	ebml_elem_read! {
		spec = elems::EbmlDocType;
		one req name = elems::EbmlDocTypeExtName;
		one req version = elems::EbmlDocTypeExtVersion;
	}
}

ebml_elem_spec! {
	pub mod elems {

		pub elem Ebml = 0x1a45dfa3, "EBML", EbmlElem;
		pub elem EbmlVersion = 0x4286, "EBMLVersion", u64;
		pub elem EbmlReadVersion = 0x42f7, "EBMLReadVersion", u64;
		pub elem EbmlMaxIdLength = 0x42f2, "EBMLMaxIdLength", u64;
		pub elem EbmlMaxSizeLength = 0x42f3, "EBMLMaxSizeLength", u64;
		pub elem EbmlDocType = 0x4282, "DocType", String;
		pub elem EbmlDocTypeVersion = 0x4287, "DocTypeVersion", u64;
		pub elem EbmlDocTypeReadVersion = 0x4285, "DocTypeReadVersion", u64;
		pub elem EbmlDocTypeExtension = 0x4281, "DocTypeExtension", EbmlDocTypeExtElem;
		pub elem EbmlDocTypeExtName = 0x4283, "DocTypeExtensionName", String;
		pub elem EbmlDocTypeExtVersion = 0x4284, "DocTypeExtensionVersion", u64;

		pub elem Crc32 = 0xbf, "CRC-32", Blob;
		pub elem Void = 0xec, "Void", Blob;

	}
}
