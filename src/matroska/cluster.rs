use crate::imports::*;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ClusterElem {
	pub timestamp: u64,
	pub position: Option <u64>,
	pub prev_size: Option <u64>,
	pub simple_blocks: Vec <BlockData>,
	pub block_groups: Vec <BlockGroupElem>,
}

impl EbmlValue for ClusterElem {
	ebml_elem_read! {
		spec = elems::Cluster;
		one req timestamp = elems::Timestamp;
		one opt position = elems::Position;
		one opt prev_size = elems::PrevSize;
		mul opt simple_blocks = elems::SimpleBlock;
		mul opt block_groups = elems::BlockGroup;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct BlockGroupElem {
	pub blocks: Vec <BlockData>,
	pub block_additions: Option <BlockAdditionsElem>,
	pub block_duration: Option <u64>,
	pub reference_priority: u64,
	pub reference_blocks: Vec <i64>,
	pub codec_state: Option <Blob>,
	pub discard_padding: Option <i64>,
}

impl EbmlValue for BlockGroupElem {
	ebml_elem_read! {
		spec = elems::BlockGroup;
		mul req blocks = elems::Block;
		one opt block_additions = elems::BlockAdditions;
		one opt block_duration = elems::BlockDuration;
		one def reference_priority = elems::ReferencePriority, & 0;
		mul opt reference_blocks = elems::ReferenceBlock;
		one opt codec_state = elems::CodecState;
		one opt discard_padding = elems::DiscardPadding;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct BlockAdditionsElem {
	pub mores: Vec <BlockMoreElem>,
}

impl EbmlValue for BlockAdditionsElem {
	ebml_elem_read! {
		spec = elems::BlockAdditions;
		mul req mores = elems::BlockMore;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct BlockMoreElem {
	pub additional: Blob,
	pub add_id: u64,
}

impl EbmlValue for BlockMoreElem {
	ebml_elem_read! {
		spec = elems::BlockMore;
		one req additional = elems::BlockAdditional;
		one def add_id = elems::BlockAddId, & 1;
	}
}

#[ allow (dead_code) ]
pub struct BlockData {
	pub track_number: u64,
	pub timestamp: i16,
	pub flags: u8,
	pub data: Vec <u8>,
}

#[ allow (dead_code) ]
impl BlockData {
	pub fn keyframe (& self) -> bool { self.flags & 0x80 != 0 }
	pub fn invisible (& self) -> bool { self.flags & 0x40 != 0 }
	pub fn lacing (& self) -> u8 { self.flags & 0x60 >> 1 as u8 }
	pub fn discardable (& self) -> bool { self.flags & 0x01 != 0 }
}

impl EbmlValue for BlockData {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <Self> {
		let data = reader.data () ?;
		let mut data = & data [ .. ];
		let Some (track_number) = read_unsigned (& mut data) else {
			any_bail! ("Error reading track number");
		};
		if data.len () < 2 { any_bail! ("Error reading timestamp") }
		let timestamp = i16::from_be_bytes ([ data [0], data [1] ]);
		data = & data [2 .. ];
		if data.len () < 1 { any_bail! ("Error reading flags") }
		let flags = data [0];
		data = & data [1 .. ];
		Ok (Self {
			track_number,
			timestamp,
			flags,
			data: data.to_vec (),
		})
	}
}

impl Debug for BlockData {
	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		fmtr.debug_struct ("BlockData")
			.field ("track_number", & self.track_number)
			.field ("timestamp", & self.timestamp)
			.field ("flags", & self.flags)
			.field ("data", & format! ("... ({} bytes)", self.data.len ()))
			.finish ()
	}
}

fn read_unsigned (src: & mut & [u8]) -> Option <u64> {
	if src.len () < 1 { return None }
	let num_bytes = src [0].leading_zeros () + 1;
	if 8 < num_bytes { return None }
	let mut val = if num_bytes < 8 { src [0] & 0xff >> num_bytes } else { 0 } as u64;
	for & byte in src [1 .. num_bytes as usize].iter () {
		val <<= 8;
		val |= byte as u64;
	}
	* src = & src [num_bytes as usize .. ];
	Some (val)
}

ebml_elem_spec! {
	pub mod elems {
		pub elem Cluster = 0x1f43b675, "Cluster", ClusterElem;
		pub elem Timestamp = 0xe7, "Timestamp", u64;
		pub elem Position = 0xa7, "Position", u64;
		pub elem PrevSize = 0xab, "PrevSize", u64;
		pub elem SimpleBlock = 0xa3, "SimpleBlock", BlockData;
		pub elem BlockGroup = 0xa0, "BlockGroup", BlockGroupElem;
		pub elem Block = 0xa1, "Block", BlockData;
		pub elem BlockAdditions = 0x75a1, "BlockAdditions", BlockAdditionsElem;
		pub elem BlockMore = 0xa6, "BlockMore", BlockMoreElem;
		pub elem BlockAdditional = 0xa5, "BlockAdditional", Blob;
		pub elem BlockAddId = 0xee, "BlockAddID", u64;
		pub elem BlockDuration = 0x9b, "BlockDuration", u64;
		pub elem ReferencePriority = 0xfa, "ReferencePriority", u64;
		pub elem ReferenceBlock = 0xfb, "ReferenceBlock", i64;
		pub elem CodecState = 0xa4, "CodecState", Blob;
		pub elem DiscardPadding = 0x75a2, "DiscardPadding", i64;
	}
}
