use crate::ebml;
use crate::imports::*;
use crate::matroska;

pub struct Reader <Src: BufRead + Seek> {
	reader: EbmlReader <Src>,
	ebml: Arc <ebml::head::EbmlElem>,
	seek_head: Arc <matroska::SeekHeadElem>,
	segment_pos: u64,
	segment_info: Option <Arc <matroska::InfoElem>>,
	tracks: Option <Arc <matroska::TracksElem>>,
	tags: Option <Arc <matroska::TagsElem>>,
}

impl <Src: BufRead + Seek> Reader <Src> {

	pub fn new (src: Src) -> anyhow::Result <Self> {

		let mut reader = EbmlReader::new (src) ?;

		// read ebml header

		let Some ((ebml_id, _, _)) = reader.read () ? else {
			any_bail! ("Error reading ebml header");
		};
		anyhow::ensure! (ebml_id == ebml::head::elems::EBML, "Expected EBML, got 0x{ebml_id:x}");
		let ebml = Arc::new (ebml::head::EbmlElem::read (& mut reader) ?);
		if 1 < ebml.read_version {
			any_bail! ("Unsupported EBML read version: {}", ebml.read_version);
		}
		if & ebml.doc_type != "matroska" {
			any_bail! ("Unsupported document type: {} (expected: matroska)", ebml.doc_type);
		}
		if 4 < ebml.doc_type_read_version {
			any_bail! ("Unsupported matroska read version: {}", ebml.doc_type_read_version);
		}

		// read segment

		let Some ((segment_id, _, _)) = reader.read () ? else {
			any_bail! ("Error reading segment");
		};
		let segment_pos = reader.position ();
		anyhow::ensure! (
			segment_id == matroska::elems::SEGMENT,
			"Expected Segment, got 0x{segment_id}");
		reader.nest ();

		// read seek head

		let Some ((seek_head_id, _, _)) = reader.read () ? else {
			any_bail! ("Error reading seek head");
		};
		anyhow::ensure! (
			seek_head_id == matroska::elems::SEEK_HEAD,
			"Expected SeekHead, got 0x{seek_head_id}");
		let seek_head = Arc::new (matroska::SeekHeadElem::read (& mut reader) ?);

		Ok (Self {
			reader,
			ebml,
			seek_head,
			segment_pos,
			segment_info: None,
			tracks: None,
			tags: None,
		})

	}

	#[ expect (dead_code) ]
	pub fn ebml (& self) -> Arc <ebml::head::EbmlElem> {
		Arc::clone (& self.ebml)
	}

	#[ expect (dead_code) ]
	pub fn seek_head (& self) -> Arc <matroska::SeekHeadElem> {
		Arc::clone (& self.seek_head)
	}

	pub fn segment_info (& mut self) -> anyhow::Result <Arc <matroska::InfoElem>> {
		if let Some (segment_info) = self.segment_info.as_ref () {
			return Ok (Arc::clone (segment_info));
		}
		let Some (seek_info) =
			self.seek_head.seeks.iter ()
				.find (|seek| seek.id == matroska::elems::INFO)
		else { any_bail! ("Info not found in seek head") };
		self.reader.jump (self.segment_pos + seek_info.position) ?;
		let Some ((info_id, _, _)) = self.reader.read () ? else {
			any_bail! ("Error reading segment info");
		};
		anyhow::ensure! (
			info_id == matroska::elems::INFO,
			"Expected Info, got 0x{info_id}");
		let segment_info = Arc::new (matroska::InfoElem::read (& mut self.reader) ?);
		self.segment_info = Some (Arc::clone (& segment_info));
		Ok (segment_info)
	}


	pub fn tracks (& mut self) -> anyhow::Result <Arc <matroska::TracksElem>> {
		if let Some (tracks) = self.tracks.as_ref () {
			return Ok (Arc::clone (tracks));
		}
		let Some (seek_tracks) =
			self.seek_head.seeks.iter ()
				.find (|seek| seek.id == matroska::elems::TRACKS)
		else { any_bail! ("Tracks not found in seek head") };
		self.reader.jump (self.segment_pos + seek_tracks.position) ?;
		let Some ((tracks_id, _, _)) = self.reader.read () ? else {
			any_bail! ("Error reading tracks");
		};
		anyhow::ensure! (
			tracks_id == matroska::elems::TRACKS,
			"Expected Tracks, got 0x{tracks_id}");
		let tracks = Arc::new (matroska::TracksElem::read (& mut self.reader) ?);
		self.tracks = Some (Arc::clone (& tracks));
		Ok (tracks)
	}

	pub fn tags (& mut self) -> anyhow::Result <Arc <matroska::TagsElem>> {
		if let Some (tags) = self.tags.as_ref () {
			return Ok (Arc::clone (tags));
		}
		let Some (seek_tags) =
			self.seek_head.seeks.iter ()
				.find (|seek| seek.id == matroska::elems::TAGS)
		else { any_bail! ("Tags not found in seek head") };
		self.reader.jump (self.segment_pos + seek_tags.position) ?;
		let Some ((tags_id, _, _)) = self.reader.read () ? else {
			any_bail! ("Error reading tags");
		};
		anyhow::ensure! (
			tags_id == matroska::elems::TAGS,
			"Expected Tags, got 0x{tags_id}");
		let tags = Arc::new (matroska::TagsElem::read (& mut self.reader) ?);
		self.tags = Some (Arc::clone (& tags));
		Ok (tags)
	}

}
