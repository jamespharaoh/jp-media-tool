use crate::imports::*;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct CuesElem {
	pub points: Vec <CuePointElem>,
}

impl EbmlElement for CuesElem {
	ebml_elem_read! {
		spec = elems::Cues;
		mul req points = elems::CuePoint;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct CuePointElem {
	pub time: u64,
	pub track_positions: Vec <CueTrackPositionsElem>,
}

impl EbmlElement for CuePointElem {
	ebml_elem_read! {
		spec = elems::CuePoint;
		one req time = elems::CueTime;
		mul req track_positions = elems::CueTrackPositions;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct CueTrackPositionsElem {
	pub track: u64,
	pub cluster_position: u64,
	pub relative_position: Option <u64>,
	pub duration: Option <u64>,
	pub block_number: Option <u64>,
	pub codec_state: u64,
	pub references: Vec <CueReferenceElem>,
}

impl EbmlElement for CueTrackPositionsElem {
	ebml_elem_read! {
		spec = elems::CueTrackPositions;
		one req track = elems::CueTrack;
		one req cluster_position = elems::CueClusterPosition;
		one opt relative_position = elems::CueRelativePosition;
		one opt duration = elems::CueDuration;
		one opt block_number = elems::CueBlockNumber;
		one def codec_state = elems::CueCodecState, & 0;
		mul opt references = elems::CueReference;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct CueReferenceElem {
	pub ref_time: u64,
}

impl EbmlElement for CueReferenceElem {
	ebml_elem_read! {
		spec = elems::CueReference;
		one req ref_time = elems::CueRefTime;
	}
}

ebml_elem_spec! {
	pub mod elems {
		pub elem Cues = 0x1c53bb6b, "Cues", CuesElem;
		pub elem CuePoint = 0xbb, "CuePoint", CuePointElem;
		pub elem CueTime = 0xb3, "CueTime", u64;
		pub elem CueTrackPositions = 0xb7, "CueTrackPositions", CueTrackPositionsElem;
		pub elem CueTrack = 0xf7, "CueTrack", u64;
		pub elem CueClusterPosition = 0xf1, "CueClusterPosition", u64;
		pub elem CueRelativePosition = 0xf0, "CueRelativePosition", u64;
		pub elem CueDuration = 0xb2, "CueDuration", u64;
		pub elem CueBlockNumber = 0xb378, "CueBlockNumber", u64;
		pub elem CueCodecState = 0xea, "CueCodecState", u64;
		pub elem CueReference = 0xdb, "CueReference", CueReferenceElem;
		pub elem CueRefTime = 0x96, "CueRefTime", u64;
	}
}
