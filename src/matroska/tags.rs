use crate::imports::*;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct TagsElem {
	pub tags: Vec <TagElem>,
}

impl EbmlElement for TagsElem {
	ebml_elem_read! {
		spec = elems::Tags;
		mul opt tags = elems::Tag;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct TagElem {
	pub targets: TargetsElem,
	pub simple_tags: Vec <SimpleTagElem>,
}

impl EbmlElement for TagElem {
	ebml_elem_read! {
		spec = elems::Tag;
		one req targets = elems::Targets;
		mul req simple_tags = elems::SimpleTag;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct TargetsElem {
	pub type_value: u64,
	pub target_type: Option <String>,
	pub track_uids: Vec <u64>,
	pub edition_uids: Vec <u64>,
	pub chapter_uids: Vec <u64>,
	pub attachment_uids: Vec <u64>,
}

impl EbmlElement for TargetsElem {
	ebml_elem_read! {
		spec = elems::Targets;
		one def type_value = elems::TargetTypeValue, & 50;
		one opt target_type = elems::TargetType;
		mul opt track_uids = elems::TagTrackUid;
		mul opt edition_uids = elems::TagEditionUid;
		mul opt chapter_uids = elems::TagChapterUid;
		mul opt attachment_uids = elems::TagAttachmentUid;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct SimpleTagElem {
	pub name: String,
	pub language: String,
	pub language_bcp47: Option <String>,
	pub default: bool,
	pub string: Option <String>,
	pub binary: Option <Blob>,
}

impl EbmlElement for SimpleTagElem {
	ebml_elem_read! {
		spec = elems::SimpleTag;
		one req name = elems::TagName;
		one def language = elems::TagLanguage, "und";
		one opt language_bcp47 = elems::TagLanguageBcp47;
		one def default = elems::TagDefault, & true;
		one opt string = elems::TagString;
		one opt binary = elems::TagBinary;
	}
}

ebml_elem_spec! {
	pub mod elems {
		pub elem Tags = 0x1254c367, "Tags", TagsElem;
		pub elem Tag = 0x7373, "Tag", TagElem;
		pub elem Targets = 0x63c0, "Targets", TargetsElem;
		pub elem TargetTypeValue = 0x68ca, "TargetTypeValue", u64;
		pub elem TargetType = 0x63ca, "TargetType", String;
		pub elem TagTrackUid = 0x63c5, "TagTrackUID", u64;
		pub elem TagEditionUid = 0x63c9, "TagEditionUID", u64;
		pub elem TagChapterUid = 0x63c4, "TagChapterUID", u64;
		pub elem TagAttachmentUid = 0x63c6, "TagAttachmentUID", u64;
		pub elem SimpleTag = 0x67c8, "SimpleTag", SimpleTagElem;
		pub elem TagName = 0x45a3, "TagName", String;
		pub elem TagLanguage = 0x447a, "TagLanguage", String;
		pub elem TagLanguageBcp47 = 0x447b, "TagLanguageBCP47", String;
		pub elem TagDefault = 0x4484, "TagDefault", bool;
		pub elem TagString = 0x4487, "TagString", String;
		pub elem TagBinary = 0x4485, "TagBinary", Blob;
	}
}
