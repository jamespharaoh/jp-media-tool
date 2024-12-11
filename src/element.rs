use crate::imports::*;

pub trait EbmlElement: Sized {
	fn read (reader: & mut dyn EbmlRead) -> anyhow::Result <Self>;
}

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

pub trait ElementReader {
	type Val;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool>;
	fn get (self) -> anyhow::Result <Self::Val>;
}

pub struct ElementReaderOneOpt <Val> {
	pub id: u64,
	pub name: & 'static str,
	pub value: Option <Val>,
}

impl <Val: ElementValue> ElementReaderOneOpt <Val> {
	fn new (spec: & impl ElementSpec) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			value: None,
		}
	}
}

impl <Val: ElementValue> ElementReader for ElementReaderOneOpt <Val> {
	type Val = Option <Val>;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		any_ensure! (self.value.is_none (), "Repeated {}", self.name);
		self.value = Some (Val::get (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Option <Val>> {
		Ok (self.value)
	}
}

pub struct ElementReaderOneReq <Val: ElementValue> {
	pub id: u64,
	pub name: & 'static str,
	pub value: Option <Val>,
}

impl <Val: ElementValue> ElementReaderOneReq <Val> {
	fn new (spec: & impl ElementSpec) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			value: None,
		}
	}
}

impl <Val: ElementValue> ElementReader for ElementReaderOneReq <Val> {
	type Val = Val;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		any_ensure! (self.value.is_none (), "Repeated {}", self.name);
		self.value = Some (Val::get (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Val> {
		self.value.ok_or_else (|| any_err! ("Missing {}", self.name))
	}
}

pub struct ElementReaderOneDef <Val: ElementValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> {
	pub id: u64,
	pub name: & 'static str,
	pub value: Option <Val>,
	pub default: & 'static Def,
}

impl <Val: ElementValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> ElementReaderOneDef <Val, Def> {
	fn new (spec: & impl ElementSpec, default: & 'static Def) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			value: None,
			default: default,
		}
	}
}

impl <Val: ElementValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> ElementReader for ElementReaderOneDef <Val, Def> {
	type Val = Val;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		any_ensure! (self.value.is_none (), "Repeated {}", self.name);
		self.value = Some (Val::get (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Val> {
		Ok (self.value.unwrap_or_else (|| self.default.to_owned ()))
	}
}

pub struct ElementReaderMulOpt <Val: ElementValue> {
	pub id: u64,
	#[ allow (dead_code) ]
	pub name: & 'static str,
	pub values: Vec <Val>,
}

impl <Val: ElementValue> ElementReaderMulOpt <Val> {
	fn new (spec: & impl ElementSpec) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			values: Vec::new (),
		}
	}
}

impl <Val: ElementValue> ElementReader for ElementReaderMulOpt <Val> {
	type Val = Vec <Val>;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		self.values.push (Val::get (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Vec <Val>> {
		Ok (self.values)
	}
}

pub struct ElementReaderMulReq <Val: ElementValue> {
	pub id: u64,
	pub name: & 'static str,
	pub values: Vec <Val>,
}

impl <Val: ElementValue> ElementReaderMulReq <Val> {
	fn new (spec: & impl ElementSpec) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			values: Vec::new (),
		}
	}
}

pub struct ElementReaderMulDef <Val: ElementValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> {
	pub id: u64,
	#[ allow (dead_code) ]
	pub name: & 'static str,
	pub values: Vec <Val>,
	pub default: & 'static [& 'static Def],
}

impl <Val: ElementValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> ElementReaderMulDef <Val, Def> {
	fn new (spec: & impl ElementSpec, default: & 'static [& 'static Def]) -> Self {
		Self {
			id: spec.id (),
			name: spec.name (),
			values: Vec::new (),
			default: default,
		}
	}
}

impl <Val: ElementValue, Def: ToOwned <Owned = Val> + ?Sized + 'static> ElementReader for ElementReaderMulDef <Val, Def> {
	type Val = Vec <Val>;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		self.values.push (Val::get (reader) ?);
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

impl <Val: ElementValue> ElementReader for ElementReaderMulReq <Val> {
	type Val = Vec <Val>;
	fn read (& mut self, ebml_id: u64, reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		if ebml_id != self.id { return Ok (false) }
		self.values.push (Val::get (reader) ?);
		Ok (true)
	}
	fn get (self) -> anyhow::Result <Vec <Val>> {
		any_ensure! (! self.values.is_empty (), "Missing {}", self.name);
		Ok (self.values)
	}
}

pub trait ElementReaderFactory <Val: ElementValue> {

	fn reader_one_opt (& self) -> ElementReaderOneOpt <Val>;

	fn reader_one_req (& self) -> ElementReaderOneReq <Val>;

	fn reader_one_def <Def: ToOwned <Owned = Val> + ?Sized + 'static> (
		& self,
		def: & 'static Def,
	) -> ElementReaderOneDef <Val, Def>;

	fn reader_mul_opt (& self) -> ElementReaderMulOpt <Val>;

	fn reader_mul_req (& self) -> ElementReaderMulReq <Val>;

	fn reader_mul_def <Def: ToOwned <Owned = Val> + ?Sized + 'static> (
		& self,
		def: & 'static [& 'static Def],
	) -> ElementReaderMulDef <Val, Def>;

}

impl <Val: ElementValue> ElementReaderFactory <Val> for ElementSpecImpl <Val> {

	fn reader_one_opt (& self) -> ElementReaderOneOpt <Val> {
		ElementReaderOneOpt::new (self)
	}

	fn reader_one_req (& self) -> ElementReaderOneReq <Val> {
		ElementReaderOneReq::new (self)
	}

	fn reader_one_def <Def: ToOwned <Owned = Val> + ?Sized + 'static> (
		& self,
		def: & 'static Def,
	) -> ElementReaderOneDef <Val, Def> {
		ElementReaderOneDef::new (self, def)
	}

	fn reader_mul_opt (& self) -> ElementReaderMulOpt <Val> {
		ElementReaderMulOpt::new (self)
	}

	fn reader_mul_req (& self) -> ElementReaderMulReq <Val> {
		ElementReaderMulReq::new (self)
	}

	fn reader_mul_def <Def: ToOwned <Owned = Val> + ?Sized + 'static> (
		& self,
		def: & 'static [& 'static Def],
	) -> ElementReaderMulDef <Val, Def> {
		ElementReaderMulDef::new (self, def)
	}

}

pub trait ElementValue: Sized {
	fn get (reader: & mut dyn EbmlRead) -> anyhow::Result <Self>;
}

impl ElementValue for bool {
	fn get (reader: & mut dyn EbmlRead) -> anyhow::Result <bool> {
		Ok (reader.boolean () ?)
	}
}

impl ElementValue for u64 {
	fn get (reader: & mut dyn EbmlRead) -> anyhow::Result <u64> {
		Ok (reader.unsigned () ?)
	}
}

impl ElementValue for f64 {
	fn get (reader: & mut dyn EbmlRead) -> anyhow::Result <f64> {
		Ok (reader.float () ?)
	}
}

impl ElementValue for Blob {
	fn get (reader: & mut dyn EbmlRead) -> anyhow::Result <Blob> {
		Ok (reader.binary () ?)
	}
}

impl ElementValue for String {
	fn get (reader: & mut dyn EbmlRead) -> anyhow::Result <String> {
		Ok (reader.string () ?)
	}
}

impl <Elem: EbmlElement> ElementValue for Elem {
	fn get (reader: & mut dyn EbmlRead) -> anyhow::Result <Elem> {
		Elem::read (reader)
	}
}

pub type Blob = Vec <u8>;

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
				$elem_vis const $elem_name: $crate::element::ElementSpecImpl <$elem_type> =
					$crate::element::ElementSpecImpl::new ($elem_id, $elem_display_name);
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
				println! ("Ignoring {} child element 0x{elem_id:x}", $parent_spec.name);
				reader.skip () ?;
			}
			reader.unnest () ?;
			Ok (Self {
				$( $name: $name.get () ?, )*
			})
		}
	};
	( @decl one req $name:ident = $spec:expr ) => {
		let mut $name = $spec.reader_one_req ();
	};
	( @decl one opt $name:ident = $spec:expr ) => {
		let mut $name = $spec.reader_one_opt ();
	};
	( @decl one def $name:ident = $spec:expr, $default:expr ) => {
		let mut $name = $spec.reader_one_def ($default);
	};
	( @decl mul req $name:ident = $spec:expr ) => {
		let mut $name = $spec.reader_mul_req ();
	};
	( @decl mul opt $name:ident = $spec:expr ) => {
		let mut $name = $spec.reader_mul_opt ();
	};
	( @decl mul def $name:ident = $spec:expr, $default:expr ) => {
		let mut $name = $spec.reader_mul_def ($default);
	};
}
