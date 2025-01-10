use crate::imports::*;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct TracksElem {
	pub entries: Vec <TrackEntryElem>,
}

impl EbmlValue for TracksElem {
	ebml_elem_read! {
		spec = elems::Tracks;
		mul req entries = elems::TrackEntry;
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
	pub content_encodings: Option <ContentEncodingsElem>,
}

impl EbmlValue for TrackEntryElem {
	ebml_elem_read! {
		spec = elems::TrackEntry;
		one req number = elems::TrackNumber;
		one req uid = elems::TrackUid;
		one req track_type = elems::TrackType;
		one def flag_enabled = elems::FlagEnabled, & true;
		one def flag_default = elems::FlagDefault, & true;
		one def flag_forced = elems::FlagForced, & false;
		one opt flag_hearing_impaired = elems::FlagHearingImpaired;
		one opt flag_visual_impaired = elems::FlagVisualImpaired;
		one opt flag_text_descriptions = elems::FlagTextDescriptions;
		one opt flag_original = elems::FlagOriginal;
		one opt flag_commentary = elems::FlagCommentary;
		one def flag_lacing = elems::FlagLacing, & true;
		one opt default_duration = elems::DefaultDuration;
		one opt default_decoded_field_duration = elems::DefaultDecodedFieldDuration;
		one def max_block_addition_id = elems::MaxBlockAdditionId, & 0;
		mul opt block_addition_mappings = elems::BlockAdditionMapping;
		one opt name = elems::Name;
		one def language = elems::Language, "eng";
		one opt language_bcp47 = elems::LanguageBcp47;
		one req codec_id = elems::CodecId;
		one opt codec_private = elems::CodecPrivate;
		one opt codec_name = elems::CodecName;
		one def codec_delay = elems::CodecDelay, & 0;
		one def seek_pre_roll = elems::SeekPreRoll, & 0;
		mul opt translates = elems::TrackTranslate;
		one opt video = elems::Video;
		one opt audio = elems::Audio;
		mul opt operations = elems::TrackOperation;
		one opt content_encodings = elems::ContentEncodings;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct BlockAdditionMappingElem {
	pub id_value: Option <u64>,
	pub id_name: Option <String>,
	pub id_type: u64,
	pub id_extra_data: Option <Blob>,
}

impl EbmlValue for BlockAdditionMappingElem {
	ebml_elem_read! {
		spec = elems::BlockAdditionMapping;
		one opt id_value = elems::BlockAddIdValue;
		one opt id_name = elems::BlockAddIdName;
		one def id_type = elems::BlockAddIdType, & 0;
		one opt id_extra_data = elems::BlockAddIdExtraData;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct TrackTranslateElem {
	pub track_id: Blob,
	pub codec: u64,
	pub edition_uids: Vec <u64>,
}

impl EbmlValue for TrackTranslateElem {
	ebml_elem_read! {
		spec = elems::TrackTranslate;
		one req track_id = elems::TrackTranslateTrackId;
		one req codec = elems::TrackTranslateCodec;
		mul opt edition_uids = elems::TrackTranslateEditionUid;
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

impl EbmlValue for VideoElem {
	ebml_elem_read! {
		spec = elems::Video;
		one def flag_interlaced = elems::FlagInterlaced, & 0;
		one def field_order = elems::FieldOrder, & 2;
		one def stereo_mode = elems::StereoMode, & 0;
		one def alpha_mode = elems::AlphaMode, & 0;
		one req pixel_width = elems::PixelWidth;
		one req pixel_height = elems::PixelHeight;
		one def pixel_crop_bottom = elems::PixelCropBottom, & 0;
		one def pixel_crop_top = elems::PixelCropTop, & 0;
		one def pixel_crop_left = elems::PixelCropLeft, & 0;
		one def pixel_crop_right = elems::PixelCropRight, & 0;
		one opt display_width = elems::DisplayWidth;
		one opt display_height = elems::DisplayHeight;
		one def display_unit =elems::DisplayUnit, & 0;
		one opt uncompressed_four_cc = elems::UncompressedFourCc;
		one opt colour = elems::Colour;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ColourElem {
	pub matrix_coefficients: u64,
	pub bits_per_channel: u64,
	pub chroma_subsampling_horz: Option <u64>,
	pub chroma_subsampling_vert: Option <u64>,
	pub cb_subsampling_horz: Option <u64>,
	pub cb_subsampling_vert: Option <u64>,
	pub chroma_siting_horz: u64,
	pub chroma_siting_vert: u64,
	pub range: u64,
	pub transfer_characteristics: u64,
	pub primaries: u64,
	pub max_cll: Option <u64>,
	pub max_fall: Option <u64>,
	pub mastering_metadata: Option <MasteringMetadataElem>,
}

impl EbmlValue for ColourElem {
	ebml_elem_read! {
		spec = elems::Colour;
		one def matrix_coefficients = elems::MatrixCoefficients, & 2;
		one def bits_per_channel = elems::BitsPerChannel, & 0;
		one opt chroma_subsampling_horz = elems::ChromaSubsamplingHorz;
		one opt chroma_subsampling_vert = elems::ChromaSubsamplingVert;
		one opt cb_subsampling_horz = elems::CbSubsamplingHorz;
		one opt cb_subsampling_vert = elems::CbSubsamplingVert;
		one def chroma_siting_horz = elems::ChromaSitingHorz, & 0;
		one def chroma_siting_vert = elems::ChromaSitingVert, & 0;
		one def range = elems::Range, & 0;
		one def transfer_characteristics = elems::TransferCharacteristics, & 2;
		one def primaries = elems::Primaries, & 2;
		one opt max_cll = elems::MaxCll;
		one opt max_fall = elems::MaxFall;
		one opt mastering_metadata = elems::MasteringMetadata;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct MasteringMetadataElem {
	pub primary_r_chromaticity_x: Option <f64>,
	pub primary_r_chromaticity_y: Option <f64>,
	pub primary_g_chromaticity_x: Option <f64>,
	pub primary_g_chromaticity_y: Option <f64>,
	pub primary_b_chromaticity_x: Option <f64>,
	pub primary_b_chromaticity_y: Option <f64>,
	pub white_point_chromaticity_x: Option <f64>,
	pub white_point_chromaticity_y: Option <f64>,
	pub luminance_max: Option <f64>,
	pub luminance_min: Option <f64>,
}

impl EbmlValue for MasteringMetadataElem {
	ebml_elem_read! {
		spec = elems::MasteringMetadata;
		one opt primary_r_chromaticity_x = elems::PrimaryRChromaticityX;
		one opt primary_r_chromaticity_y = elems::PrimaryRChromaticityY;
		one opt primary_g_chromaticity_x = elems::PrimaryGChromaticityX;
		one opt primary_g_chromaticity_y = elems::PrimaryGChromaticityY;
		one opt primary_b_chromaticity_x = elems::PrimaryBChromaticityX;
		one opt primary_b_chromaticity_y = elems::PrimaryBChromaticityY;
		one opt white_point_chromaticity_x = elems::WhitePointChromaticityX;
		one opt white_point_chromaticity_y = elems::WhitePointChromaticityY;
		one opt luminance_max = elems::LuminanceMax;
		one opt luminance_min = elems::LuminanceMin;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct AudioElem {
	pub sampling_frequency: f64,
	pub output_sampling_frequency: Option <f64>,
	pub channels: u64,
	pub bit_depth: Option <u64>,
	pub emphasis: u64,
}

impl EbmlValue for AudioElem {
	ebml_elem_read! {
		spec = elems::Audio;
		one def sampling_frequency = elems::SamplingFrequency, & 8000.0;
		one opt output_sampling_frequency = elems::OutputSamplingFrequency;
		one def channels = elems::Channels, & 1;
		one opt bit_depth = elems::BitDepth;
		one def emphasis = elems::Emphasis, & 0;
	}
}

#[ derive (Debug) ]
pub struct TrackOperationElem {
	// TODO
}

impl EbmlValue for TrackOperationElem {
	ebml_elem_read! {
		spec = elems::TrackOperation;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ContentEncodingsElem {
	pub encodings: Vec <ContentEncodingElem>,
}

impl EbmlValue for ContentEncodingsElem {
	ebml_elem_read! {
		spec = elems::ContentEncodings;
		mul req encodings = elems::ContentEncoding;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ContentEncodingElem {
	pub order: u64,
	pub scope: u64,
	pub type_: u64,
	pub compression: Option <ContentCompressionElem>,
	pub encryption: Option <ContentEncryptionElem>,
}

impl EbmlValue for ContentEncodingElem {
	ebml_elem_read! {
		spec = elems::ContentEncoding;
		one def order = elems::ContentEncodingOrder, & 0;
		one def scope = elems::ContentEncodingScope, & 1;
		one def type_ = elems::ContentEncodingType, & 0;
		one opt compression = elems::ContentCompression;
		one opt encryption = elems::ContentEncryption;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ContentCompressionElem {
	pub algo: u64,
	pub settings: Option <Blob>,
}

impl EbmlValue for ContentCompressionElem {
	ebml_elem_read! {
		spec = elems::ContentCompression;
		one def algo = elems::ContentCompAlgo, & 0;
		one opt settings = elems::ContentCompSettings;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ContentEncryptionElem {
	pub algo: u64,
	pub key_id: Option <Blob>,
	pub aes_settings: Option <ContentEncAesSettingsElem>,
}

impl EbmlValue for ContentEncryptionElem {
	ebml_elem_read! {
		spec = elems::ContentEncryption;
		one def algo = elems::ContentEncAlgo, & 0;
		one opt key_id = elems::ContentEncKeyId;
		one opt aes_settings = elems::ContentEncAesSettings;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ContentEncAesSettingsElem {
	pub cipher_mode: u64,
}

impl EbmlValue for ContentEncAesSettingsElem {
	ebml_elem_read! {
		spec = elems::ContentEncAesSettings;
		one req cipher_mode = elems::AesSettingsCipherMode;
	}
}

ebml_elem_spec! {
	pub mod elems {
		pub elem Tracks = 0x1654ae6b, "Tracks", TracksElem;
		pub elem TrackEntry = 0xae, "TrackEntry", TrackEntryElem;
		pub elem TrackNumber = 0xd7, "TrackNumber", u64;
		pub elem TrackUid = 0x73c5, "TrackUID", u64;
		pub elem TrackType = 0x83, "TrackType", u64;
		pub elem FlagEnabled = 0xb9, "FlagEnabled", bool;
		pub elem FlagDefault = 0x88, "FlagDefault", bool;
		pub elem FlagForced = 0x55aa, "FlagForced", bool;
		pub elem FlagHearingImpaired = 0x55ab, "FlagHearingImpaired", bool;
		pub elem FlagVisualImpaired = 0x55ac, "FlagVisualImpaired", bool;
		pub elem FlagTextDescriptions = 0x55ad, "FlagTextDescriptions", bool;
		pub elem FlagOriginal = 0x55ae, "FlagOriginal", bool;
		pub elem FlagCommentary = 0x55af, "FlagCommentary", bool;
		pub elem FlagLacing = 0x9c, "FlagLacing", bool;
		pub elem DefaultDuration = 0x23e383, "DefaultDuration", u64;
		pub elem DefaultDecodedFieldDuration = 0x234e7a, "DefaultDecodedFieldDuration", u64;
		pub elem MaxBlockAdditionId = 0x55ee, "MaxBlockAdditionID", u64;
		pub elem BlockAdditionMapping = 0x41e4, "BlockAdditionMapping", BlockAdditionMappingElem;
		pub elem BlockAddIdValue = 0x41f0, "BlockAddIDValue", u64;
		pub elem BlockAddIdName = 0x41f4, "BlockAddIDName", String;
		pub elem BlockAddIdType = 0x41e7, "BlockAddIDType", u64;
		pub elem BlockAddIdExtraData = 0x41ed, "BlockAddIDExtraData", Blob;
		pub elem Name = 0x536e, "Name", String;
		pub elem Language = 0x22b59c, "Language", String;
		pub elem LanguageBcp47 = 0x22b59d, "LanguageBCP47", String;
		pub elem CodecId = 0x86, "CodecID", String;
		pub elem CodecPrivate = 0x63a2, "CodecPrivate", Blob;
		pub elem CodecName = 0x258688, "CodecName", String;
		pub elem CodecDelay = 0x56aa, "CodecDelay", u64;
		pub elem SeekPreRoll = 0x56bb, "SeekPreRoll", u64;
		pub elem TrackTranslate = 0x6624, "TrackTranslate", TrackTranslateElem;
		pub elem TrackTranslateTrackId = 0x66a5, "TrackTranslateTrackID", Blob;
		pub elem TrackTranslateCodec = 0x66bf, "TrackTranslateCodec", u64;
		pub elem TrackTranslateEditionUid = 0x66fc, "TrackTranslateEditionUID", u64;
		pub elem Video = 0xe0, "Video", VideoElem;
		pub elem FlagInterlaced = 0x9a, "FlagInterlaced", u64;
		pub elem FieldOrder = 0x9d, "FieldOrder", u64;
		pub elem StereoMode = 0x53b8, "StereoMode", u64;
		pub elem AlphaMode = 0x53c0, "AlphaMode", u64;
		pub elem PixelWidth = 0xb0, "PixelWidth", u64;
		pub elem PixelHeight = 0xba, "PixelHeight", u64;
		pub elem PixelCropBottom = 0x54aa, "PixelCropBottom", u64;
		pub elem PixelCropTop = 0x54bb, "PixelCropTop", u64;
		pub elem PixelCropLeft = 0x54cc, "PixelCropLeft", u64;
		pub elem PixelCropRight = 0x54dd, "PixelCropRight", u64;
		pub elem DisplayWidth = 0x54b0, "DisplayWidth", u64;
		pub elem DisplayHeight = 0x54ba, "DisplayHeight", u64;
		pub elem DisplayUnit = 0x54b2, "DisplayUnit", u64;
		pub elem UncompressedFourCc = 0x2eb524, "UncompressedFourCC", Blob;
		pub elem Colour = 0x55b0, "Colour", ColourElem;
		pub elem MatrixCoefficients = 0x55b1, "MatrixCoefficients", u64;
		pub elem BitsPerChannel = 0x55b2, "BitsPerChannel", u64;
		pub elem ChromaSubsamplingHorz = 0x55b3, "ChromaSubsamplingHorz", u64;
		pub elem ChromaSubsamplingVert = 0x55b4, "ChromaSubsamplingVert", u64;
		pub elem CbSubsamplingHorz = 0x55b5, "CbSubsamplingHorz", u64;
		pub elem CbSubsamplingVert = 0x55b6, "CbSubsamplingVert", u64;
		pub elem ChromaSitingHorz = 0x55b7, "ChromaSitingHorz", u64;
		pub elem ChromaSitingVert = 0x55b8, "ChromaSitingVert", u64;
		pub elem Range = 0x55b9, "Range", u64;
		pub elem TransferCharacteristics = 0x55ba, "TransferCharacteristics", u64;
		pub elem Primaries = 0x55bb, "Primaries", u64;
		pub elem MaxCll = 0x55bc, "MaxCLL", u64;
		pub elem MaxFall = 0x55bd, "MaxFALL", u64;
		pub elem MasteringMetadata = 0x55d0, "MasteringMetadata", MasteringMetadataElem;
		pub elem PrimaryRChromaticityX = 0x55d1, "PrimaryRChromaticityX", f64;
		pub elem PrimaryRChromaticityY = 0x55d2, "PrimaryRChromaticityY", f64;
		pub elem PrimaryGChromaticityX = 0x55d3, "PrimaryGChromaticityX", f64;
		pub elem PrimaryGChromaticityY = 0x55d4, "PrimaryGChromaticityY", f64;
		pub elem PrimaryBChromaticityX = 0x55d5, "PrimaryBChromaticityX", f64;
		pub elem PrimaryBChromaticityY = 0x55d6, "PrimaryBChromaticityY", f64;
		pub elem WhitePointChromaticityX = 0x55d7, "WhitePointChromaticityX", f64;
		pub elem WhitePointChromaticityY = 0x55d8, "WhitePointChromaticityY", f64;
		pub elem LuminanceMax = 0x55d9, "LuminanceMax", f64;
		pub elem LuminanceMin = 0x55da, "LuminanceMin", f64;
		pub elem Projection = 0x7670, "Projection", () /*ProjectionElem*/;
		pub elem ProjectionType = 0x7671, "ProjectionType", u64;
		pub elem ProjectionPrivate = 0x7672, "ProjectionPrivate", Blob;
		pub elem ProjectionPoseYaw = 0x7673, "ProjectionPoseYaw", f64;
		pub elem ProjectionPosePitch = 0x7674, "ProjectionPosePitch", f64;
		pub elem ProjectionPoseRoll = 0x7675, "ProjectionPoseRoll", f64;
		pub elem Audio = 0xe1, "Audio", AudioElem;
		pub elem SamplingFrequency = 0xb5, "SamplingFrequency", f64;
		pub elem OutputSamplingFrequency = 0x78b5, "OutputSamplingFrequency", f64;
		pub elem Channels = 0x9f, "Channels", u64;
		pub elem BitDepth = 0x6264, "BitDepth", u64;
		pub elem Emphasis = 0x52f1, "Emphasis", u64;
		pub elem TrackOperation = 0xe2, "TrackOperation", TrackOperationElem;
		pub elem TrackCombinePlanes = 0xe3, "TrackCombinePlanes", () /*TrackCombinePlanesElem*/;
		pub elem TrackPlane = 0xe4, "TrackPlane", () /*TrackPlaneElem*/;
		pub elem TrackPlaneUid = 0xe5, "TrackPlaneUID", u64;
		pub elem TrackPlaneType = 0xe6, "TrackPlaneType", u64;
		pub elem TrackJoinBlocks = 0xe9, "TrackJoinBlocks", () /*TrackJoinBlocksElem*/;
		pub elem TrackJoinUid = 0xed, "TrackJoinUID", u64;
		pub elem ContentEncodings = 0x6d80, "ContentEncodings", ContentEncodingsElem;
		pub elem ContentEncoding = 0x6240, "ContentEncoding", ContentEncodingElem;
		pub elem ContentEncodingOrder = 0x5031, "ContentEncodingOrder", u64;
		pub elem ContentEncodingScope = 0x5032, "ContentEncodingScope", u64;
		pub elem ContentEncodingType = 0x5033, "ContentEncodingType", u64;
		pub elem ContentCompression = 0x5034, "ContentCompression", ContentCompressionElem;
		pub elem ContentCompAlgo = 0x4254, "ContentCompAlgo", u64;
		pub elem ContentCompSettings = 0x4255, "ContentCompSettings", Blob;
		pub elem ContentEncryption = 0x5035, "ContentEncryption", ContentEncryptionElem;
		pub elem ContentEncAlgo = 0x47e1, "ContentEncAlgo", u64;
		pub elem ContentEncKeyId = 0x47e2, "ContentEncKeyID", Blob;
		pub elem ContentEncAesSettings = 0x47e7, "ContentEncAESSettings", ContentEncAesSettingsElem;
		pub elem AesSettingsCipherMode = 0x47e8, "AESSettingsCipherMode", u64;
	}
}
