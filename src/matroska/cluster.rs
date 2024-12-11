use crate::imports::*;

ebml_elem_spec! {
	pub mod elems {
		pub elem Cluster = 0x1f43b675, "Cluster", ();
		pub elem Timestamp = 0xe7, "Timestamp", u64;
		pub elem Position = 0xa7, "Position", u64;
		pub elem PrevSize = 0xab, "PrevSize", u64;
		pub elem SimpleBlock = 0xa3, "SimpleBlock", u64;
		pub elem BlockGroup = 0xa0, "BlockGroup", ();
		pub elem Block = 0xa1, "Block", Blob;
		pub elem BlockAdditions = 0x75a1, "BlockAdditions", ();
		pub elem BlockMore = 0xa6, "BlockMore", ();
		pub elem BlockAdditional = 0xa5, "BlockAdditional", ();
		pub elem BlockAddId = 0xee, "BlockAddID", ();
		pub elem BlockDuration = 0x9b, "BlockDuration", ();
		pub elem ReferencePriority = 0xfa, "ReferencePriority", ();
		pub elem ReferenceBlock = 0xfb, "ReferenceBlock", ();
		pub elem CodecState = 0xa4, "CodecState", ();
		pub elem DiscardPadding = 0x75a2, "DiscardPadding", ();
	}
}
