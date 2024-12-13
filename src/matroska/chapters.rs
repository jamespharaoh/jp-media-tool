use crate::imports::*;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ChaptersElem {
	editions: Vec <EditionEntryElem>,
}

impl EbmlValue for ChaptersElem {
	ebml_elem_read! {
		spec = elems::Chapters;
		mul req editions = elems::EditionEntry;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct EditionEntryElem {
	pub uid: Option <u64>,
	pub flag_hidden: bool,
	pub flag_default: bool,
	pub flag_ordered: bool,
	pub displays: Vec <EditionDisplayElem>,
	pub atoms: Vec <ChapterAtomElem>,
}

impl EbmlValue for EditionEntryElem {
	ebml_elem_read! {
		spec = elems::EditionEntry;
		one opt uid = elems::EditionUid;
		one def flag_hidden = elems::EditionFlagHidden, & false;
		one def flag_default = elems::EditionFlagDefault, & false;
		one def flag_ordered = elems::EditionFlagOrdered, & false;
		mul opt displays = elems::EditionDisplay;
		mul req atoms = elems::ChapterAtom;
	}
}

#[ derive (Debug) ]
pub struct EditionDisplayElem {
}

impl EbmlValue for EditionDisplayElem {
	ebml_elem_read! {
		spec = elems::EditionDisplay;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ChapterAtomElem {
	pub uid: u64,
	pub string_uid: Option <String>,
	pub time_start: u64,
	pub time_end: Option <u64>,
	pub flag_hidden: bool,
	pub flag_enabled: bool,
	pub segment_uuid: Option <Vec <u8>>,
	pub skip_type: Option <u64>,
	pub segment_edition_uid: Option <u64>,
	pub physical_equiv: Option <u64>,
	pub track: Option <ChapterTrackElem>,
	pub displays: Vec <ChapterDisplayElem>,
}

impl EbmlValue for ChapterAtomElem {
	ebml_elem_read! {
		spec = elems::ChapterAtom;
		one req uid = elems::ChapterUid;
		one opt string_uid = elems::ChapterStringUid;
		one req time_start = elems::ChapterTimeStart;
		one opt time_end = elems::ChapterTimeEnd;
		one def flag_hidden = elems::ChapterFlagHidden, & false;
		one def flag_enabled = elems::ChapterFlagEnabled, & true;
		one opt segment_uuid = elems::ChapterSegmentUuid;
		one opt skip_type = elems::ChapterSkipType;
		one opt segment_edition_uid = elems::ChapterSegmentEditionUid;
		one opt physical_equiv = elems::ChapterPhysicalEquiv;
		one opt track = elems::ChapterTrack;
		mul opt displays = elems::ChapterDisplay;
	}
}

#[ derive (Debug) ]
pub struct ChapterTrackElem {
}

impl EbmlValue for ChapterTrackElem {
	ebml_elem_read! {
		spec = elems::ChapterTrack;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ChapterDisplayElem {
	pub string: String,
	pub languages: Vec <String>,
	pub languages_bcp47: Vec <String>,
	pub countries: Vec <String>,
}

impl EbmlValue for ChapterDisplayElem {
	ebml_elem_read! {
		spec = elems::ChapterDisplay;
		one req string = elems::ChapString;
		mul def languages = elems::ChapLanguage, & [ "eng" ];
		mul opt languages_bcp47 = elems::ChapLanguageBcp47;
		mul opt countries = elems::ChapCountry;
	}
}

ebml_elem_spec! {
	pub mod elems {
		pub elem Chapters = 0x1043a770, "Chapters", ChaptersElem;
		pub elem EditionEntry = 0x45b9, "EditionEntry", EditionEntryElem;
		pub elem EditionUid = 0x45bc, "EditionUID", u64;
		pub elem EditionFlagHidden = 0x45bd, "EditionFlagHidden", bool;
		pub elem EditionFlagDefault = 0x45db, "EditionFlagDefault", bool;
		pub elem EditionFlagOrdered = 0x45dd, "EditionFlagOrdered", bool;
		pub elem EditionDisplay = 0x4520, "EditionDisplay", EditionDisplayElem;
		pub elem EditionString = 0x4521, "EditionString", String;
		pub elem EditionLanguageIetf = 0x45e4, "EditionLanguageIETF", String;
		pub elem ChapterAtom = 0xb6, "ChapterAtom", ChapterAtomElem;
		pub elem ChapterUid = 0x73c4, "ChapterUID", u64;
		pub elem ChapterStringUid = 0x5654, "ChapterStringUID", String;
		pub elem ChapterTimeStart = 0x91, "ChapterTimeStart", u64;
		pub elem ChapterTimeEnd = 0x92, "ChapterTimeEnd", u64;
		pub elem ChapterFlagHidden = 0x98, "ChapterFlagHidden", bool;
		pub elem ChapterFlagEnabled = 0x4598, "ChapterFlagEnabled", bool;
		pub elem ChapterSegmentUuid = 0x6e67, "ChapterSegmentUuid", Blob;
		pub elem ChapterSkipType = 0x4588, "ChapterSkipType", u64;
		pub elem ChapterSegmentEditionUid = 0x6ebc, "ChapterSegmentEditionUid", u64;
		pub elem ChapterPhysicalEquiv = 0x63c3, "ChapterPhysicalEquiv", u64;
		pub elem ChapterTrack = 0x8f, "ChapterTrack", ChapterTrackElem;
		pub elem ChapterTrackUid = 0x89, "ChapterTrackUID", u64;
		pub elem ChapterDisplay = 0x80, "ChapterDisplay", ChapterDisplayElem;
		pub elem ChapString = 0x85, "ChapString", String;
		pub elem ChapLanguage = 0x437c, "ChapLanguage", String;
		pub elem ChapLanguageBcp47 = 0x437d, "ChapLanguageBCP47", String;
		pub elem ChapCountry = 0x437e, "ChapCountry", String;
		pub elem ChapterProcess = 0x6944, "ChapterProcess", () /*ChapProcessElem*/;
		pub elem ChapterProcessCodecId = 0x6955, "ChapProcessCodecID", u64;
		pub elem ChapterProcessPrivate = 0x450d, "ChapProcessPrivate", Blob;
		pub elem ChapterProcessCommand = 0x6911, "ChapProcessCommand", () /*ChapProcessCommandElem*/;
		pub elem ChapterProcessTime = 0x6922, "ChapProcessTime", u64;
		pub elem ChapterProcessData = 0x6933, "ChapProcessData", Blob;
	}
}
