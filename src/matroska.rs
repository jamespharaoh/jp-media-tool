use crate::imports::*;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct SeekHeadElem {
	pub seeks: Vec <SeekElem>,
}

impl EbmlElement for SeekHeadElem {
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

impl EbmlElement for SeekElem {
	ebml_elem_read! {
		spec = elems::Seek;
		one req id = elems::SeekId;
		one req position = elems::SeekPosition;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct SegmentInfoElem {
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

impl EbmlElement for SegmentInfoElem {
	ebml_elem_read! {
		spec = elems::SegmentInfo;
		one opt uuid = elems::SegmentUuid;
		one opt filename = elems::SegmentFilename;
		one opt prev_uuid = elems::SegmentPrevUuid;
		one opt prev_filename = elems::SegmentPrevFilename;
		one opt next_uuid = elems::SegmentNextUuid;
		one opt next_filename = elems::SegmentNextFilename;
		mul opt families = elems::SegmentFamily;
		mul opt chapter_translates = elems::SegmentChapterTranslate;
		one req timestamp_scale = elems::SegmentTimestampScale;
		one opt duration = elems::SegmentDuration;
		one opt date_utc = elems::SegmentDateUtc;
		one opt title = elems::SegmentTitle;
		one req muxing_app = elems::SegmentMuxingApp;
		one req writing_app = elems::SegmentWritingApp;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ChapterTranslateElem {
	pub id: Vec <u8>,
	pub codec: u64,
	pub edition_uids: Vec <u64>,
}

impl EbmlElement for ChapterTranslateElem {
	ebml_elem_read! {
		spec = elems::SegmentChapterTranslate;
		one req id = elems::SegmentChapterTranslateId;
		one req codec = elems::SegmentChapterTranslateCodec;
		mul opt edition_uids = elems::SegmentChapterTranslateEditionUid;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct TracksElem {
	pub tracks: Vec <TrackEntryElem>,
}

impl EbmlElement for TracksElem {
	ebml_elem_read! {
		spec = elems::Tracks;
		mul req tracks = elems::TrackEntry;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct TrackEntryElem {
	pub number: u64,
	pub uid: u64,
	pub track_type: u64,
	pub flag_enabled: bool,
	pub flag_default: bool,
	pub flag_forced: bool,
	pub flag_hearing_impaired: Option <bool>,
	pub flag_visual_impaired: Option <bool>,
	pub flag_text_descriptions: Option <bool>,
	pub flag_original: Option <bool>,
	pub flag_commentary: Option <bool>,
	pub flag_lacing: bool,
	pub default_duration: Option <u64>,
	pub default_decoded_field_duration: Option <u64>,
	pub max_block_addition_id: u64,
	pub block_addition_mappings: Vec <BlockAdditionMappingElem>,
	pub name: Option <String>,
	pub language: String,
	pub language_bcp47: Option <String>,
	pub codec_id: String,
	pub codec_private: Option <Vec <u8>>,
	pub codec_name: Option <String>,
	pub codec_delay: u64,
	pub seek_pre_roll: u64,
	pub translates: Vec <TrackTranslateElem>,
	pub video: Option <VideoElem>,
	pub audio: Option <AudioElem>,
	pub operations: Vec <TrackOperationElem>,
	pub encodings: Option <ContentEncodingsElem>,
}

impl EbmlElement for TrackEntryElem {
	ebml_elem_read! {
		spec = elems::TrackEntry;
		one req number = elems::TrackNumber;
		one req uid = elems::TrackUid;
		one req track_type = elems::TrackType;
		one def flag_enabled = elems::TrackFlagEnabled, & true;
		one def flag_default = elems::TrackFlagDefault, & true;
		one def flag_forced = elems::TrackFlagForced, & false;
		one opt flag_hearing_impaired = elems::TrackFlagHearingImpaired;
		one opt flag_visual_impaired = elems::TrackFlagVisualImpaired;
		one opt flag_text_descriptions = elems::TrackFlagTextDescriptions;
		one opt flag_original = elems::TrackFlagOriginal;
		one opt flag_commentary = elems::TrackFlagCommentary;
		one def flag_lacing = elems::TrackFlagLacing, & true;
		one opt default_duration = elems::TrackDefaultDuration;
		one opt default_decoded_field_duration = elems::TrackDefaultDecodedFieldDuration;
		one def max_block_addition_id = elems::TrackMaxBlockAdditionId, & 0;
		mul opt block_addition_mappings = elems::TrackBlockAdditionMapping;
		one opt name = elems::TrackName;
		one def language = elems::TrackLanguage, "eng";
		one opt language_bcp47 = elems::TrackLanguageBcp47;
		one req codec_id = elems::TrackCodecId;
		one opt codec_private = elems::TrackCodecPrivate;
		one opt codec_name = elems::TrackCodecName;
		one def codec_delay = elems::TrackCodecDelay, & 0;
		one def seek_pre_roll = elems::TrackSeekPreRoll, & 0;
		mul opt translates = elems::TrackTranslate;
		one opt video = elems::TrackVideo;
		one opt audio = elems::TrackAudio;
		mul opt operations = elems::TrackOperation;
		one opt encodings = elems::TrackContentEncodings;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct BlockAdditionMappingElem {
	// TODO
}

impl EbmlElement for BlockAdditionMappingElem {
	ebml_elem_read! {
		spec = elems::TrackBlockAdditionMapping;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct TrackTranslateElem {
	// TODO
}

impl EbmlElement for TrackTranslateElem {
	ebml_elem_read! {
		spec = elems::TrackTranslate;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct VideoElem {
	pub flag_interlaced: u64,
	pub field_order: u64,
	pub stereo_mode: u64,
	pub alpha_mode: u64,
	pub pixel_width: u64,
	pub pixel_height: u64,
	pub pixel_crop_bottom: u64,
	pub pixel_crop_top: u64,
	pub pixel_crop_left: u64,
	pub pixel_crop_right: u64,
	pub display_width: Option <u64>,
	pub display_height: Option <u64>,
	pub display_unit: u64,
	pub uncompressed_four_cc: Option <Vec <u8>>,
	pub colour: Option <ColourElem>,
}

impl EbmlElement for VideoElem {
	ebml_elem_read! {
		spec = elems::TrackVideo;
		one def flag_interlaced = elems::TrackFlagInterlaced, & 0;
		one def field_order = elems::TrackFieldOrder, & 2;
		one def stereo_mode = elems::TrackStereoMode, & 0;
		one def alpha_mode = elems::TrackAlphaMode, & 0;
		one req pixel_width = elems::TrackPixelWidth;
		one req pixel_height = elems::TrackPixelHeight;
		one def pixel_crop_bottom = elems::TrackPixelCropBottom, & 0;
		one def pixel_crop_top = elems::TrackPixelCropTop, & 0;
		one def pixel_crop_left = elems::TrackPixelCropLeft, & 0;
		one def pixel_crop_right = elems::TrackPixelCropRight, & 0;
		one opt display_width = elems::TrackDisplayWidth;
		one opt display_height = elems::TrackDisplayHeight;
		one def display_unit =elems::TrackDisplayUnit, & 0;
		one opt uncompressed_four_cc = elems::TrackUncompressedFourCc;
		one opt colour =elems::TrackColour;
	}
}

#[ derive (Debug) ]
pub struct ColourElem {
	// TODO
}

impl EbmlElement for ColourElem {
	ebml_elem_read! {
		spec = elems::TrackColour;
	}
}

#[ derive (Debug) ]
pub struct AudioElem {
	// TODO
}

impl EbmlElement for AudioElem {
	ebml_elem_read! {
		spec = elems::TrackAudio;
	}
}

#[ derive (Debug) ]
pub struct TrackOperationElem {
	// TODO
}

impl EbmlElement for TrackOperationElem {
	ebml_elem_read! {
		spec = elems::TrackOperation;
	}
}

#[ derive (Debug) ]
pub struct ContentEncodingsElem {
	// TODO
}

impl EbmlElement for ContentEncodingsElem {
	ebml_elem_read! {
		spec = elems::TrackContentEncodings;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ChaptersElem {
	editions: Vec <EditionEntryElem>,
}

impl EbmlElement for ChaptersElem {
	ebml_elem_read! {
		spec = elems::Chapters;
		mul req editions = elems::ChapterEditionEntry;
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

impl EbmlElement for EditionEntryElem {
	ebml_elem_read! {
		spec = elems::ChapterEditionEntry;
		one opt uid = elems::ChapterEditionUid;
		one def flag_hidden = elems::ChapterEditionFlagHidden, & false;
		one def flag_default = elems::ChapterEditionFlagDefault, & false;
		one def flag_ordered = elems::ChapterEditionFlagOrdered, & false;
		mul opt displays = elems::ChapterEditionDisplay;
		mul req atoms = elems::ChapterAtom;
	}
}

#[ derive (Debug) ]
pub struct EditionDisplayElem {
}

impl EbmlElement for EditionDisplayElem {
	ebml_elem_read! {
		spec = elems::ChapterEditionDisplay;
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

impl EbmlElement for ChapterAtomElem {
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

impl EbmlElement for ChapterTrackElem {
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

impl EbmlElement for ChapterDisplayElem {
	ebml_elem_read! {
		spec = elems::ChapterDisplay;
		one req string = elems::ChapterString;
		mul def languages = elems::ChapterLanguage, & [ "eng" ];
		mul opt languages_bcp47 = elems::ChapterLanguageBcp47;
		mul opt countries = elems::ChapterCountry;
	}
}

ebml_elem_spec! {

	pub mod elems {

		pub elem Cluster = 0x1f43b675, "Cluster", ();
		pub elem ClusterTimestamp = 0xe7, "Timestamp", u64;
		pub elem ClusterPosition = 0xa7, "Position", u64;
		pub elem ClusterPrevSize = 0xab, "PrevSize", u64;
		pub elem ClusterSimpleBlock = 0xa3, "SimpleBlock", u64;
		pub elem ClusterBlockGroup = 0xa0, "BlockGroup", ();
		pub elem ClusterBlock = 0xa1, "Block", Blob;
		pub elem ClusterBlockAdditions = 0x75a1, "BlockAdditions", ();
		pub elem ClusterBlockMore = 0xa6, "BlockMore", ();
		pub elem ClusterBlockAdditional = 0xa5, "BlockAdditional", ();
		pub elem ClusterBlockAddId = 0xee, "BlockAddID", ();
		pub elem ClusterBlockDuration = 0x9b, "BlockDuration", ();
		pub elem ClusterReferencePriority = 0xfa, "ReferencePriority", ();
		pub elem ClusterReferenceBlock = 0xfb, "ReferenceBlock", ();
		pub elem ClusterCodecState = 0xa4, "CodecState", ();
		pub elem ClusterDiscardPadding = 0x75a2, "DiscardPadding", ();

		pub elem Tracks = 0x1654ae6b, "Tracks", TracksElem;
		pub elem TrackEntry = 0xae, "TrackEntry", TrackEntryElem;
		pub elem TrackNumber = 0xd7, "TrackNumber", u64;
		pub elem TrackUid = 0x73c5, "TrackUID", u64;
		pub elem TrackType = 0x83, "TrackType", u64;
		pub elem TrackFlagEnabled = 0xb9, "FlagEnabled", bool;
		pub elem TrackFlagDefault = 0x88, "FlagDefault", bool;
		pub elem TrackFlagForced = 0x55aa, "FlagForced", bool;
		pub elem TrackFlagHearingImpaired = 0x55ab, "FlagHearingImpaired", bool;
		pub elem TrackFlagVisualImpaired = 0x55ac, "FlagVisualImpaired", bool;
		pub elem TrackFlagTextDescriptions = 0x55ad, "FlagTextDescriptions", bool;
		pub elem TrackFlagOriginal = 0x55ae, "FlagOriginal", bool;
		pub elem TrackFlagCommentary = 0x55af, "FlagCommentary", bool;
		pub elem TrackFlagLacing = 0x9c, "FlagLacing", bool;
		pub elem TrackDefaultDuration = 0x23e383, "DefaultDuration", u64;
		pub elem TrackDefaultDecodedFieldDuration = 0x234e7a, "DefaultDecodedFieldDuration", u64;
		pub elem TrackMaxBlockAdditionId = 0x55ee, "MaxBlockAdditionID", u64;
		pub elem TrackBlockAdditionMapping = 0x41e4, "BlockAdditionMapping", BlockAdditionMappingElem;
		pub elem TrackBlockAddIdValue = 0x41f0, "BlockAddIDValue", u64;
		pub elem TrackBlockAddIdName = 0x41f4, "BlockAddIDName", String;
		pub elem TrackBlockAddIdType = 0x41e7, "BlockAddIDType", u64;
		pub elem TrackBlockAddIdExtraData = 0x41ed, "BlockAddIDExtraData", Blob;
		pub elem TrackName = 0x536e, "Name", String;
		pub elem TrackLanguage = 0x22b59c, "Language", String;
		pub elem TrackLanguageBcp47 = 0x22b59d, "LanguageBCP47", String;
		pub elem TrackCodecId = 0x86, "CodecID", String;
		pub elem TrackCodecPrivate = 0x63a2, "CodecPrivate", Blob;
		pub elem TrackCodecName = 0x258688, "CodecName", String;
		pub elem TrackCodecDelay = 0x56aa, "CodecDelay", u64;
		pub elem TrackSeekPreRoll = 0x56bb, "SeekPreRoll", u64;
		pub elem TrackTranslate = 0x6624, "TrackTranslate", TrackTranslateElem;
		pub elem TrackTranslateTrackId = 0x66a5, "TrackTranslateTrackID", Blob;
		pub elem TrackTranslateCodec = 0x66bf, "TrackTranslateCodec", u64;
		pub elem TrackTranslateEditionUid = 0x66fc, "TrackTranslateEditionUID", u64;
		pub elem TrackVideo = 0xe0, "Video", VideoElem;
		pub elem TrackFlagInterlaced = 0x9a, "FlagInterlaced", u64;
		pub elem TrackFieldOrder = 0x9d, "FieldOrder", u64;
		pub elem TrackStereoMode = 0x53b8, "StereoMode", u64;
		pub elem TrackAlphaMode = 0x53c0, "AlphaMode", u64;
		pub elem TrackPixelWidth = 0xb0, "PixelWidth", u64;
		pub elem TrackPixelHeight = 0xba, "PixelHeight", u64;
		pub elem TrackPixelCropBottom = 0x54aa, "PixelCropBottom", u64;
		pub elem TrackPixelCropTop = 0x54bb, "PixelCropTop", u64;
		pub elem TrackPixelCropLeft = 0x54cc, "PixelCropLeft", u64;
		pub elem TrackPixelCropRight = 0x54dd, "PixelCropRight", u64;
		pub elem TrackDisplayWidth = 0x54b0, "DisplayWidth", u64;
		pub elem TrackDisplayHeight = 0x54ba, "DisplayHeight", u64;
		pub elem TrackDisplayUnit = 0x54b2, "DisplayUnit", u64;
		pub elem TrackUncompressedFourCc = 0x2eb524, "UncompressedFourCC", Blob;
		pub elem TrackColour = 0x55b0, "Colour", ColourElem;
		pub elem TrackMatrixCoefficients = 0x55b1, "MatrixCoefficients", u64;
		pub elem TrackBitsPerChannel = 0x55b2, "BitsPerChannel", u64;
		pub elem TrackChromaSubsamplingHorz = 0x55b3, "ChromaSubsamplingHorz", u64;
		pub elem TrackChromaSubsamplingVert = 0x55b4, "ChromaSubsamplingVert", u64;
		pub elem TrackCbSubsamplingHorz = 0x55b5, "CbSubsamplingHorz", u64;
		pub elem TrackCbSubsamplingVert = 0x55b6, "CbSubsamplingVert", u64;
		pub elem TrackChromaSitingHorz = 0x55b7, "ChromaSitingHorz", u64;
		pub elem TrackChromaSitingVert = 0x55b8, "ChromaSitingVert", u64;
		pub elem TrackRange = 0x55b9, "Range", u64;
		pub elem TrackTransferCharacteristics = 0x55ba, "TransferCharacteristics", u64;
		pub elem TrackPrimaries = 0x55bb, "Primaries", u64;
		pub elem TrackMaxCll = 0x55bc, "MaxCLL", u64;
		pub elem TrackMaxFall = 0x55bd, "MaxFALL", u64;
		pub elem TrackMasteringMetadata = 0x55d0, "MasteringMetadata", () /*MasteringMetadataElem*/;
		pub elem TrackPrimaryRChromaticityX = 0x55d1, "PrimaryRChromaticityX", f64;
		pub elem TrackPrimaryRChromaticityY = 0x55d2, "PrimaryRChromaticityY", f64;
		pub elem TrackPrimaryGChromaticityX = 0x55d3, "PrimaryGChromaticityX", f64;
		pub elem TrackPrimaryGChromaticityY = 0x55d4, "PrimaryGChromaticityY", f64;
		pub elem TrackPrimaryBChromaticityX = 0x55d5, "PrimaryBChromaticityX", f64;
		pub elem TrackPrimaryBChromaticityY = 0x55d6, "PrimaryBChromaticityY", f64;
		pub elem TrackWhitePointChromaticityX = 0x55d7, "WhitePointChromaticityX", f64;
		pub elem TrackWhitePointChromaticityY = 0x55d8, "WhitePointChromaticityY", f64;
		pub elem TrackLuminanceMax = 0x55d9, "LuminanceMax", f64;
		pub elem TrackLuminanceMin = 0x55da, "LuminanceMin", f64;
		pub elem TrackProjection = 0x7670, "Projection", () /*ProjectionElem*/;
		pub elem TrackProjectionType = 0x7671, "ProjectionType", u64;
		pub elem TrackProjectionPrivate = 0x7672, "ProjectionPrivate", Blob;
		pub elem TrackProjectionPoseYaw = 0x7673, "ProjectionPoseYaw", f64;
		pub elem TrackProjectionPosePitch = 0x7674, "ProjectionPosePitch", f64;
		pub elem TrackProjectionPoseRoll = 0x7675, "ProjectionPoseRoll", f64;
		pub elem TrackAudio = 0xe1, "Audio", AudioElem;
		pub elem TrackAudioSamplingFrequency = 0xb5, "SamplingFrequency", f64;
		pub elem TrackAudioOutputSamplingFrequency = 0x78b5, "OutputSamplingFrequency", f64;
		pub elem TrackAudioChannels = 0x9f, "Channels", u64;
		pub elem TrackAudioBitDepth = 0x6264, "BitDepth", u64;
		pub elem TrackAudioEmphasis = 0x52f1, "Emphasis", u64;
		pub elem TrackOperation = 0xe2, "TrackOperation", TrackOperationElem;
		pub elem TrackCombinePlanes = 0xe3, "TrackCombinePlanes", () /*TrackCombinePlanesElem*/;
		pub elem TrackPlane = 0xe4, "TrackPlane", () /*TrackPlaneElem*/;
		pub elem TrackPlaneUid = 0xe5, "TrackPlaneUID", u64;
		pub elem TrackPlaneType = 0xe6, "TrackPlaneType", u64;
		pub elem TrackJoinBlocks = 0xe9, "TrackJoinBlocks", () /*TrackJoinBlocksElem*/;
		pub elem TrackJoinUid = 0xed, "TrackJoinUID", u64;
		pub elem TrackContentEncodings = 0x6d80, "ContentEncodings", ContentEncodingsElem;
		pub elem TrackContentEncoding = 0x6240, "ContentEncoding", () /*ContentEncodingElem*/;
		pub elem TrackContentEncodingOrder = 0x5031, "ContentEncodingOrder", u64;
		pub elem TrackContentEncodingScope = 0x5032, "ContentEncodingScope", u64;
		pub elem TrackContentEncodingType = 0x5033, "ContentEncodingType", u64;
		pub elem TrackContentCompression = 0x5034, "ContentCompression", u64;
		pub elem TrackContentCompAlgo = 0x4254, "ContentCompAlgo", u64;
		pub elem TrackContentCompSettings = 0x4255, "ContentCompSettings", Blob;
		pub elem TrackContentEncryption = 0x5035, "ContentEncryption", () /*ContentEncryptionElem*/;
		pub elem TrackContentEncAlgo = 0x47e1, "ContentEncAlgo", u64;
		pub elem TrackContentEncKeyId = 0x47e2, "ContentEncKeyID", Blob;
		pub elem TrackContentEncAesSettings = 0x47e7, "ContentEncAESSettings", () /*ContentEncAesSettingsElem*/;
		pub elem TrackAesSettingsCipherMode = 0x47e8, "AESSettingsCipherMode", u64;

		pub elem SeekHead = 0x114d9b74, "SeekHead", SeekHeadElem;
		pub elem Seek = 0x4dbb, "Seek", SeekElem;
		pub elem SeekId = 0x53ab, "SeekID", u64;
		pub elem SeekPosition = 0x53ac, "SeekPosition", u64;

		pub elem Segment = 0x18538067, "Segment", ();
		pub elem SegmentInfo = 0x1549a966, "Info", SegmentInfoElem;
		pub elem SegmentUuid = 0x73a4, "SegmentUUID", Blob;
		pub elem SegmentFilename = 0x7384, "SegmentFilename", String;
		pub elem SegmentPrevUuid = 0x3cb923, "PrevUUID", Blob;
		pub elem SegmentPrevFilename = 0x3c83ab, "PrevFilename", String;
		pub elem SegmentNextUuid = 0x3eb923, "NextUUID", Blob;
		pub elem SegmentNextFilename = 0x3e83bb, "NextFilename", String;
		pub elem SegmentFamily = 0x4444, "SegmentFamily", Blob;
		pub elem SegmentChapterTranslate = 0x6924, "ChapterTranslate", ChapterTranslateElem;
		pub elem SegmentChapterTranslateId = 0x69a5, "ChapterTranslateID", Blob;
		pub elem SegmentChapterTranslateCodec = 0x69bf, "ChapterTranslateCodec", u64;
		pub elem SegmentChapterTranslateEditionUid = 0x69fc, "ChapterTranslateEditionUID", u64;
		pub elem SegmentTimestampScale = 0x2ad7b1, "TimestampScale", u64;
		pub elem SegmentDuration = 0x4489, "Duration", f64;
		pub elem SegmentDateUtc = 0x4461, "DateUTC", u64;
		pub elem SegmentTitle = 0x7ba9, "Title", String;
		pub elem SegmentMuxingApp = 0x4d80, "MuxingApp", String;
		pub elem SegmentWritingApp = 0x5741, "WritingApp", String;

		pub elem Chapters = 0x1043a770, "Chapters", ChaptersElem;
		pub elem ChapterEditionEntry = 0x45b9, "EditionEntry", EditionEntryElem;
		pub elem ChapterEditionUid = 0x45bc, "EditionUID", u64;
		pub elem ChapterEditionFlagHidden = 0x45bd, "EditionFlagHidden", bool;
		pub elem ChapterEditionFlagDefault = 0x45db, "EditionFlagDefault", bool;
		pub elem ChapterEditionFlagOrdered = 0x45dd, "EditionFlagOrdered", bool;
		pub elem ChapterEditionDisplay = 0x4520, "EditionDisplay", EditionDisplayElem;
		pub elem ChapterEditionString = 0x4521, "EditionString", String;
		pub elem ChapterEditionLanguageIetf = 0x45e4, "EditionLanguageIETF", String;
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
		pub elem ChapterString = 0x85, "ChapString", String;
		pub elem ChapterLanguage = 0x437c, "ChapLanguage", String;
		pub elem ChapterLanguageBcp47 = 0x437d, "ChapLanguageBCP47", String;
		pub elem ChapterCountry = 0x437e, "ChapCountry", String;
		pub elem ChapterProcess = 0x6944, "ChapterProcess", () /*ChapProcessElem*/;
		pub elem ChapterProcessCodecId = 0x6955, "ChapProcessCodecID", u64;
		pub elem ChapterProcessPrivate = 0x450d, "ChapProcessPrivate", Blob;
		pub elem ChapterProcessCommand = 0x6911, "ChapProcessCommand", () /*ChapProcessCommandElem*/;
		pub elem ChapterProcessTime = 0x6922, "ChapProcessTime", u64;
		pub elem ChapterProcessData = 0x6933, "ChapProcessData", Blob;

	}

}
