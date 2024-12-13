use crate::imports::*;

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct ClusterElem {
	pub timestamp: u64,
	pub position: Option <u64>,
	pub prev_size: Option <u64>,
	pub simple_blocks: Vec <BlobRef>,
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
}

impl EbmlValue for BlockGroupElem {
	ebml_elem_read! {
		spec = elems::BlockGroup;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct BlockAdditionsElem {
}

impl EbmlValue for BlockAdditionsElem {
	ebml_elem_read! {
		spec = elems::BlockAdditions;
	}
}

#[ allow (dead_code) ]
#[ derive (Debug) ]
pub struct BlockMoreElem {
}

impl EbmlValue for BlockMoreElem {
	ebml_elem_read! {
		spec = elems::BlockMore;
	}
}

ebml_elem_spec! {
	pub mod elems {
		pub elem Cluster = 0x1f43b675, "Cluster", ClusterElem;
		pub elem Timestamp = 0xe7, "Timestamp", u64;
		pub elem Position = 0xa7, "Position", u64;
		pub elem PrevSize = 0xab, "PrevSize", u64;
		pub elem SimpleBlock = 0xa3, "SimpleBlock", BlobRef;
		pub elem BlockGroup = 0xa0, "BlockGroup", BlockGroupElem;
		pub elem Block = 0xa1, "Block", BlobRef;
		pub elem BlockAdditions = 0x75a1, "BlockAdditions", BlockAdditionsElem;
		pub elem BlockMore = 0xa6, "BlockMore", BlockMoreElem;
		pub elem BlockAdditional = 0xa5, "BlockAdditional", BlobRef;
		pub elem BlockAddId = 0xee, "BlockAddID", u64;
		pub elem BlockDuration = 0x9b, "BlockDuration", u64;
		pub elem ReferencePriority = 0xfa, "ReferencePriority", u64;
		pub elem ReferenceBlock = 0xfb, "ReferenceBlock", i64;
		pub elem CodecState = 0xa4, "CodecState", Blob;
		pub elem DiscardPadding = 0x75a2, "DiscardPadding", i64;
	}
}
