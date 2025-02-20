pub mod attachments;
pub mod chapters;
pub mod cluster;
pub mod cues;
pub mod reader;
pub mod segment;
pub mod tags;
pub mod tracks;

pub use chapters::ChaptersElem;
pub use cluster::ClusterElem;
pub use cues::CuesElem;
pub use reader::Reader;
pub use segment::InfoElem;
pub use segment::SeekHeadElem;
pub use tags::TagsElem;
pub use tracks::TracksElem;

pub mod elems {
	use super::*;
	pub use chapters::elems::CHAPTERS;
	pub use cluster::elems::CLUSTER;
	pub use cues::elems::CUES;
	pub use segment::elems::INFO;
	pub use segment::elems::SEEK_HEAD;
	pub use segment::elems::SEGMENT;
	pub use tags::elems::TAGS;
	pub use tracks::elems::TRACKS;
}
