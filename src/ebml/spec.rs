use crate::imports::*;

pub trait ElementSpec {
	fn id (& self) -> u64;
	fn name (& self) -> & 'static str;
}

#[ derive (Clone, Copy) ]
pub struct ElementSpecImpl <Val> {
	pub id: u64,
	pub name: & 'static str,
	_phantom: PhantomData <Val>,
}

impl <Val> ElementSpecImpl <Val> {
	pub const fn new (id: u64, name: & 'static str) -> Self {
		Self { id, name, _phantom: PhantomData }
	}
}

impl <Val> ElementSpec for ElementSpecImpl <Val> {
	fn id (& self) -> u64 { self.id }
	fn name (& self) -> & 'static str { self.name }
}

pub trait FieldReader {
	type Val;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool>;
	fn get (self) -> anyhow::Result <Self::Val>;
}

pub struct FieldReaderOneOpt <Val> {
	pub id: u64,
	pub name: & 'static str,
	pub value: Option <Val>,
}

impl <Val: EbmlValue> FieldReaderOneOpt <Val> {
	fn new (spec: & impl ElementSpec) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			value: None,
		}
	}
}

impl <Val: EbmlValue> FieldReader for FieldReaderOneOpt <Val> {
	type Val = Option <Val>;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		any_ensure! (self.value.is_none (), "Repeated {}", self.name);
		self.value = Some (Val::read (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Option <Val>> {
		Ok (self.value)
	}
}

pub struct FieldReaderOneReq <Val: EbmlValue> {
	pub id: u64,
	pub name: & 'static str,
	pub value: Option <Val>,
}

impl <Val: EbmlValue> FieldReaderOneReq <Val> {
	fn new (spec: & impl ElementSpec) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			value: None,
		}
	}
}

impl <Val: EbmlValue> FieldReader for FieldReaderOneReq <Val> {
	type Val = Val;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		any_ensure! (self.value.is_none (), "Repeated {}", self.name);
		self.value = Some (Val::read (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Val> {
		self.value.ok_or_else (|| any_err! ("Missing {}", self.name))
	}
}

pub struct FieldReaderOneDef <Val: EbmlValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> {
	pub id: u64,
	pub name: & 'static str,
	pub value: Option <Val>,
	pub default: & 'static Def,
}

impl <Val: EbmlValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> FieldReaderOneDef <Val, Def> {
	fn new (spec: & impl ElementSpec, default: & 'static Def) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			value: None,
			default: default,
		}
	}
}

impl <
	Val: EbmlValue,
	Def: ToOwned <Owned = Val> + ?Sized + 'static,
> FieldReader for FieldReaderOneDef <Val, Def> {
	type Val = Val;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		any_ensure! (self.value.is_none (), "Repeated {}", self.name);
		self.value = Some (Val::read (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Val> {
		Ok (self.value.unwrap_or_else (|| self.default.to_owned ()))
	}
}

pub struct FieldReaderMulOpt <Val: EbmlValue> {
	pub id: u64,
	#[ allow (dead_code) ]
	pub name: & 'static str,
	pub values: Vec <Val>,
}

impl <Val: EbmlValue> FieldReaderMulOpt <Val> {
	fn new (spec: & impl ElementSpec) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			values: Vec::new (),
		}
	}
}

impl <Val: EbmlValue> FieldReader for FieldReaderMulOpt <Val> {
	type Val = Vec <Val>;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		self.values.push (Val::read (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Vec <Val>> {
		Ok (self.values)
	}
}

pub struct FieldReaderMulReq <Val: EbmlValue> {
	pub id: u64,
	pub name: & 'static str,
	pub values: Vec <Val>,
}

impl <Val: EbmlValue> FieldReaderMulReq <Val> {
	fn new (spec: & impl ElementSpec) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			values: Vec::new (),
		}
	}
}

pub struct FieldReaderMulDef <Val: EbmlValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> {
	pub id: u64,
	#[ allow (dead_code) ]
	pub name: & 'static str,
	pub values: Vec <Val>,
	pub default: & 'static [& 'static Def],
}

impl <Val: EbmlValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> FieldReaderMulDef <Val, Def> {
	fn new (spec: & impl ElementSpec, default: & 'static [& 'static Def]) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			values: Vec::new (),
			default: default,
		}
	}
}

impl <
	Val: EbmlValue,
	Def: ToOwned <Owned = Val> + ?Sized + 'static,
> FieldReader for FieldReaderMulDef <Val, Def> {
	type Val = Vec <Val>;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		self.values.push (Val::read (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Vec <Val>> {
		if self.values.is_empty () {
			Ok (self.default.iter ().map (|& def| def.to_owned ()).collect ())
		} else {
			Ok (self.values)
		}
	}
}

impl <Val: EbmlValue> FieldReader for FieldReaderMulReq <Val> {
	type Val = Vec <Val>;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		self.values.push (Val::read (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Vec <Val>> {
		any_ensure! (! self.values.is_empty (), "Missing {}", self.name);
		Ok (self.values)
	}
}

pub trait FieldReaderFactory <Val: EbmlValue> {

	fn field_reader_one_opt (& self) -> FieldReaderOneOpt <Val>;

	fn field_reader_one_req (& self) -> FieldReaderOneReq <Val>;

	fn field_reader_one_def <Def: ToOwned <Owned = Val> + ?Sized + 'static> (
		& self,
		def: & 'static Def,
	) -> FieldReaderOneDef <Val, Def>;

	fn field_reader_mul_opt (& self) -> FieldReaderMulOpt <Val>;

	fn field_reader_mul_req (& self) -> FieldReaderMulReq <Val>;

	fn field_reader_mul_def <Def: ToOwned <Owned = Val> + ?Sized + 'static> (
		& self,
		def: & 'static [& 'static Def],
	) -> FieldReaderMulDef <Val, Def>;

}

impl <Val: EbmlValue> FieldReaderFactory <Val> for ElementSpecImpl <Val> {

	fn field_reader_one_opt (& self) -> FieldReaderOneOpt <Val> {
		FieldReaderOneOpt::new (self)
	}

	fn field_reader_one_req (& self) -> FieldReaderOneReq <Val> {
		FieldReaderOneReq::new (self)
	}

	fn field_reader_one_def <Def: ToOwned <Owned = Val> + ?Sized + 'static> (
		& self,
		def: & 'static Def,
	) -> FieldReaderOneDef <Val, Def> {
		FieldReaderOneDef::new (self, def)
	}

	fn field_reader_mul_opt (& self) -> FieldReaderMulOpt <Val> {
		FieldReaderMulOpt::new (self)
	}

	fn field_reader_mul_req (& self) -> FieldReaderMulReq <Val> {
		FieldReaderMulReq::new (self)
	}

	fn field_reader_mul_def <Def: ToOwned <Owned = Val> + ?Sized + 'static> (
		& self,
		def: & 'static [& 'static Def],
	) -> FieldReaderMulDef <Val, Def> {
		FieldReaderMulDef::new (self, def)
	}

}

#[ macro_export ]
macro_rules! ebml_elem_spec {
	(
		$mod_vis:vis mod $mod_name:ident {
			$(
				$elem_vis:vis elem $elem_name:ident =
					$elem_id:expr, $elem_display_name:expr, $elem_type:ty;
			)*
		}
	) => {
		#[ allow (dead_code) ]
		#[ allow (non_upper_case_globals) ]
		$mod_vis mod $mod_name {
			use super::*;
			$(
				$elem_vis const $elem_name: $crate::ebml::spec::ElementSpecImpl <$elem_type> =
					$crate::ebml::spec::ElementSpecImpl::new ($elem_id, $elem_display_name);
				paste! {
					$elem_vis const [<$elem_name:snake:upper>]: u64 = $elem_id;
				}
			)*
		}
	};
}

#[ macro_export ]
macro_rules! ebml_elem_read {
	(
		spec = $parent_spec:expr;
		$( $num:tt $req:tt $name:ident = $spec:expr $( , $def:expr )?; )*
	) => {
		#[ inline (never) ]
		fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <Self> {
			$( ebml_elem_read! (@decl $num $req $name = $spec $( , $def )? ); )*
			reader.nest ();
			while let Some ((elem_id, _, _)) = reader.read () ? {
				$( if $name.read (elem_id, reader) ? { continue } )*
				if elem_id != 0xbf && elem_id != 0xec {
					eprintln! ("Ignoring {} child element 0x{elem_id:x}", $parent_spec.name);
				}
				reader.skip () ?;
			}
			reader.unnest () ?;
			Ok (Self {
				$( $name: $name.get () ?, )*
			})
		}
	};
	( @decl one req $name:ident = $spec:expr ) => {
		let mut $name = $spec.field_reader_one_req ();
	};
	( @decl one opt $name:ident = $spec:expr ) => {
		let mut $name = $spec.field_reader_one_opt ();
	};
	( @decl one def $name:ident = $spec:expr, $default:expr ) => {
		let mut $name = $spec.field_reader_one_def ($default);
	};
	( @decl mul req $name:ident = $spec:expr ) => {
		let mut $name = $spec.field_reader_mul_req ();
	};
	( @decl mul opt $name:ident = $spec:expr ) => {
		let mut $name = $spec.field_reader_mul_opt ();
	};
	( @decl mul def $name:ident = $spec:expr, $default:expr ) => {
		let mut $name = $spec.field_reader_mul_def ($default);
	};
}
