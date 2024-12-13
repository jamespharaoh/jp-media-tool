use crate::imports::*;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct SeekHeadElem {
	pub seeks: Vec <SeekElem>,
}

impl EbmlValue for SeekHeadElem {
	ebml_elem_read! {
		spec = elems::SeekHead;
		mul req seeks = elems::Seek;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct SeekElem {
	pub id: u64,
	pub position: u64,
}

impl EbmlValue for SeekElem {
	ebml_elem_read! {
		spec = elems::Seek;
		one req id = elems::SeekId;
		one req position = elems::SeekPosition;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct InfoElem {
	pub uuid: Option <Blob>,
	pub filename: Option <String>,
	pub prev_uuid: Option <Blob>,
	pub prev_filename: Option <String>,
	pub next_uuid: Option <Blob>,
	pub next_filename: Option <String>,
	pub families: Vec <Blob>,
	pub chapter_translates: Vec <ChapterTranslateElem>,
	pub timestamp_scale: u64,
	pub duration: Option <f64>,
	pub date_utc: Option <u64>,
	pub title: Option <String>,
	pub muxing_app: String,
	pub writing_app: String,
}

impl EbmlValue for InfoElem {
	ebml_elem_read! {
		spec = elems::Info;
		one opt uuid = elems::SegmentUuid;
		one opt filename = elems::SegmentFilename;
		one opt prev_uuid = elems::PrevUuid;
		one opt prev_filename = elems::PrevFilename;
		one opt next_uuid = elems::NextUuid;
		one opt next_filename = elems::NextFilename;
		mul opt families = elems::SegmentFamily;
		mul opt chapter_translates = elems::ChapterTranslate;
		one req timestamp_scale = elems::TimestampScale;
		one opt duration = elems::Duration;
		one opt date_utc = elems::DateUtc;
		one opt title = elems::Title;
		one req muxing_app = elems::MuxingApp;
		one req writing_app = elems::WritingApp;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ChapterTranslateElem {
	pub id: Vec <u8>,
	pub codec: u64,
	pub edition_uids: Vec <u64>,
}

impl EbmlValue for ChapterTranslateElem {
	ebml_elem_read! {
		spec = elems::ChapterTranslate;
		one req id = elems::ChapterTranslateId;
		one req codec = elems::ChapterTranslateCodec;
		mul opt edition_uids = elems::ChapterTranslateEditionUid;
	}
}

ebml_elem_spec! {
	pub mod elems {
		pub elem Segment = 0x18538067, "Segment", ();
		pub elem Info = 0x1549a966, "Info", InfoElem;
		pub elem SegmentUuid = 0x73a4, "SegmentUUID", Blob;
		pub elem SegmentFilename = 0x7384, "SegmentFilename", String;
		pub elem PrevUuid = 0x3cb923, "PrevUUID", Blob;
		pub elem PrevFilename = 0x3c83ab, "PrevFilename", String;
		pub elem NextUuid = 0x3eb923, "NextUUID", Blob;
		pub elem NextFilename = 0x3e83bb, "NextFilename", String;
		pub elem SegmentFamily = 0x4444, "SegmentFamily", Blob;
		pub elem ChapterTranslate = 0x6924, "ChapterTranslate", ChapterTranslateElem;
		pub elem ChapterTranslateId = 0x69a5, "ChapterTranslateID", Blob;
		pub elem ChapterTranslateCodec = 0x69bf, "ChapterTranslateCodec", u64;
		pub elem ChapterTranslateEditionUid = 0x69fc, "ChapterTranslateEditionUID", u64;
		pub elem TimestampScale = 0x2ad7b1, "TimestampScale", u64;
		pub elem Duration = 0x4489, "Duration", f64;
		pub elem DateUtc = 0x4461, "DateUTC", u64;
		pub elem Title = 0x7ba9, "Title", String;
		pub elem MuxingApp = 0x4d80, "MuxingApp", String;
		pub elem WritingApp = 0x5741, "WritingApp", String;
		pub elem SeekHead = 0x114d9b74, "SeekHead", SeekHeadElem;
		pub elem Seek = 0x4dbb, "Seek", SeekElem;
		pub elem SeekId = 0x53ab, "SeekID", u64;
		pub elem SeekPosition = 0x53ac, "SeekPosition", u64;
	}
}
