#![feature(deprecated)]

// relevant commits:
// - 36b6630771cb
// - c5cc29b0e0bb8238955691dc0cc324050653cbeb

#[deprecated(since = "1.0.0", note = "text", suggestion = "replacement")]
pub fn deprecated() {}
pub fn replacement() {}
#[deprecated(since = "1.0.0", note = "text", suggestion = "replacement_text")]
pub fn deprecated_text() {}
pub fn replacement_text() {}

pub struct MethodTester;

impl MethodTester {
    #[deprecated(since = "1.0.0", note = "text", suggestion = "method_replacement")]
    pub fn method_deprecated(&self) {}
    pub fn method_replacement(&self) {}
    #[deprecated(since = "1.0.0", note = "text", suggestion = "method_replacement_text")]
    pub fn method_deprecated_text(&self) {}
    pub fn method_replacement_text(&self) {}
}

pub trait Trait {
    #[deprecated(since = "1.0.0", note = "text", suggestion = "trait_replacement")]
    fn trait_deprecated(&self) {}
    fn trait_replacement(&self) {}
    #[deprecated(since = "1.0.0", note = "text", suggestion = "trait_replacement_text")]
    fn trait_deprecated_text(&self) {}
    fn trait_replacement_text(&self) {}
}

#[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementTrait")]
pub trait DeprecatedTrait { fn dummy(&self) { } }
pub trait ReplacementTrait { fn dummy(&self) { } }

impl Trait for MethodTester {}

#[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementStruct")]
pub struct DeprecatedStruct {
    pub i: isize
}
pub struct ReplacementStruct {
    pub i: isize
}

#[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementUnitStruct")]
pub struct DeprecatedUnitStruct;
pub struct ReplacementUnitStruct;

pub enum Enum {
    #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementVariant")]
    DeprecatedVariant,
    ReplacementVariant,
}

#[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementTupleStruct")]
pub struct DeprecatedTupleStruct(pub isize);
pub struct ReplacementTupleStruct(pub isize);

pub mod nested {
    #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementStruct")]
    pub struct DeprecatedStruct {
        pub i: isize
    }
    pub struct ReplacementStruct {
        pub i: isize
    }

    #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementUnitStruct")]
    pub struct DeprecatedUnitStruct;
    pub struct ReplacementUnitStruct;

    pub enum Enum {
        #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementVariant")]
        DeprecatedVariant,
        ReplacementVariant,
    }

    #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementTupleStruct")]
    pub struct DeprecatedTupleStruct(pub isize);
    pub struct ReplacementTupleStruct(pub isize);
}

pub struct Stable {
    #[deprecated(since = "1.0.0", note = "text", suggestion = "replacement2")]
    pub override2: u8,
    pub replacement2: u8,
}

pub struct Stable2(pub u8, pub u8, #[deprecated(since = "1.0.0", note = "text", suggestion = "1")] pub u8);

#[deprecated(since = "1.0.0", note = "text", suggestion = "Replacement")]
pub struct Deprecated {
    pub inherit: u8,
}
pub struct Replacement {
    pub inherit: u8,
}

#[deprecated(since = "1.0.0", note = "text", suggestion = "Replacement2")]
pub struct Deprecated2(pub u8,
                       pub u8,
                       pub u8);
pub struct Replacement2(pub u8,
                        pub u8,
                        pub u8);

#[deprecated(since = "1.0.0", note = "text", suggestion = "replacement_mod")]
pub mod deprecated_mod {
    pub fn deprecated() {}
}
pub mod replacement_mod {
    pub fn deprecated() {}
}

#[macro_export]
macro_rules! macro_test {
    () => (deprecated());
}

#[macro_export]
macro_rules! macro_test_arg {
    ($func:expr) => ($func);
}

#[macro_export]
macro_rules! macro_test_arg_nested {
    ($func:ident) => (macro_test_arg!($func()));
}
