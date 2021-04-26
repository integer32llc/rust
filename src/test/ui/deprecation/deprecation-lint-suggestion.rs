// `suggestion` in `deprecated` attributes and automatic fixing thereof

// aux-build:deprecation-suggestion.rs
// run-rustfix

#![deny(deprecated)]
#![allow(warnings)]

#[macro_use]
extern crate deprecation_suggestion;

mod cross_crate {
    use deprecation_suggestion::*;

    fn test() {
        type Foo = MethodTester;
        let foo = MethodTester;

        deprecated(); //~ ERROR use of deprecated function `deprecation_suggestion::deprecated`
        foo.method_deprecated(); //~ ERROR use of deprecated associated function `deprecation_suggestion::MethodTester::method_deprecated`
        Foo::method_deprecated(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::MethodTester::method_deprecated`
        <Foo>::method_deprecated(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::MethodTester::method_deprecated`
        foo.trait_deprecated(); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`
        Trait::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`
        <Foo>::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`
        <Foo as Trait>::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`

        deprecated_text(); //~ ERROR use of deprecated function `deprecation_suggestion::deprecated_text`: text
        foo.method_deprecated_text(); //~ ERROR use of deprecated associated function `deprecation_suggestion::MethodTester::method_deprecated_text`: text
        Foo::method_deprecated_text(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::MethodTester::method_deprecated_text`: text
        <Foo>::method_deprecated_text(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::MethodTester::method_deprecated_text`: text
        foo.trait_deprecated_text(); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text
        Trait::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text
        <Foo>::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text
        <Foo as Trait>::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text

        let _ = DeprecatedStruct { //~ ERROR use of deprecated struct `deprecation_suggestion::DeprecatedStruct`: text
            i: 0 //~ ERROR use of deprecated field `deprecation_suggestion::DeprecatedStruct::i`: text
        };

        let _ = DeprecatedUnitStruct; //~ ERROR use of deprecated struct `deprecation_suggestion::DeprecatedUnitStruct`: text

        let _ = Enum::DeprecatedVariant; //~ ERROR use of deprecated variant `deprecation_suggestion::Enum::DeprecatedVariant`: text

        let _ = DeprecatedTupleStruct (1); //~ ERROR use of deprecated struct `deprecation_suggestion::DeprecatedTupleStruct`: text

        let _ = nested::DeprecatedStruct { //~ ERROR use of deprecated struct `deprecation_suggestion::nested::DeprecatedStruct`: text
            i: 0 //~ ERROR use of deprecated field `deprecation_suggestion::nested::DeprecatedStruct::i`: text
        };

        let _ = nested::DeprecatedUnitStruct; //~ ERROR use of deprecated struct `deprecation_suggestion::nested::DeprecatedUnitStruct`: text

        let _ = nested::Enum::DeprecatedVariant; //~ ERROR use of deprecated variant `deprecation_suggestion::nested::Enum::DeprecatedVariant`: text

        let _ = nested::DeprecatedTupleStruct (1); //~ ERROR use of deprecated struct `deprecation_suggestion::nested::DeprecatedTupleStruct`: text

        // At the moment, the lint checker only checks stability in
        // in the arguments of macros.
        // Eventually, we will want to lint the contents of the
        // macro in the module *defining* it. Also, stability levels
        // on macros themselves are not yet linted.
        macro_test_arg!(deprecated_text()); //~ ERROR use of deprecated function `deprecation_suggestion::deprecated_text`: text
        macro_test_arg!(macro_test_arg!(deprecated_text())); //~ ERROR use of deprecated function `deprecation_suggestion::deprecated_text`: text
    }

    fn test_method_param<Foo: Trait>(foo: Foo) {
        foo.trait_deprecated(); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`
        Trait::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`
        <Foo>::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`
        <Foo as Trait>::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`
        foo.trait_deprecated_text(); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text
        Trait::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text
        <Foo>::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text
        <Foo as Trait>::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text
    }

    fn test_method_object(foo: &Trait) {
        foo.trait_deprecated(); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated`
        foo.trait_deprecated_text(); //~ ERROR use of deprecated associated function `deprecation_suggestion::Trait::trait_deprecated_text`: text
    }

    struct S;

    impl DeprecatedTrait for S {} //~ ERROR use of deprecated trait `deprecation_suggestion::DeprecatedTrait`: text
    trait LocalTrait : DeprecatedTrait { } //~ ERROR use of deprecated trait `deprecation_suggestion::DeprecatedTrait`: text

    pub fn foo() {
        let x = Stable {
            override2: 3,
            //~^ ERROR use of deprecated field `deprecation_suggestion::Stable::override2`: text
        };

        let _ = x.override2;
        //~^ ERROR use of deprecated field `deprecation_suggestion::Stable::override2`: text

        let Stable {
            override2: _
            //~^ ERROR use of deprecated field `deprecation_suggestion::Stable::override2`: text
        } = x;
        // all fine
        let Stable { .. } = x;

        let x = Stable2(1, 2, 3);

        let _ = x.2;
        //~^ ERROR use of deprecated field `deprecation_suggestion::Stable2::2`: text

        let Stable2(_,
                   _,
                   _)
            //~^ ERROR use of deprecated field `deprecation_suggestion::Stable2::2`: text
            = x;
        // all fine
        let Stable2(..) = x;

        let x = Deprecated {
            //~^ ERROR use of deprecated struct `deprecation_suggestion::Deprecated`: text
            inherit: 1,
            //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated::inherit`: text
        };

        let _ = x.inherit;
        //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated::inherit`: text

        let Deprecated {
            //~^ ERROR use of deprecated struct `deprecation_suggestion::Deprecated`: text
            inherit: _,
            //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated::inherit`: text
        } = x;

        let Deprecated
            //~^ ERROR use of deprecated struct `deprecation_suggestion::Deprecated`: text
            { .. } = x;

        let x = Deprecated2(1, 2, 3);
        //~^ ERROR use of deprecated struct `deprecation_suggestion::Deprecated2`: text

        let _ = x.0;
        //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated2::0`: text
        let _ = x.1;
        //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated2::1`: text
        let _ = x.2;
        //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated2::2`: text

        let Deprecated2
        //~^ ERROR use of deprecated struct `deprecation_suggestion::Deprecated2`: text
            (_,
             //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated2::0`: text
             _,
             //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated2::1`: text
             _)
             //~^ ERROR use of deprecated field `deprecation_suggestion::Deprecated2::2`: text
            = x;
        let Deprecated2
        //~^ ERROR use of deprecated struct `deprecation_suggestion::Deprecated2`: text
            // the patterns are all fine:
            (..) = x;
    }
}

mod inheritance {
    use deprecation_suggestion::*;

    fn test_inheritance() {
        deprecated_mod::deprecated(); //~ ERROR use of deprecated function `deprecation_suggestion::deprecated_mod::deprecated`: text
    }
}

mod this_crate {
    #[deprecated(since = "1.0.0", note = "text", suggestion = "replacement")]
    pub fn deprecated() {}
    #[deprecated(since = "1.0.0", note = "text", suggestion = "replacement_text")]
    pub fn deprecated_text() {}

    #[deprecated(since = "99.99.99", note = "text", suggestion = "replacement_future")]
    pub fn deprecated_future() {}
    #[deprecated(since = "99.99.99", note = "text", suggestion = "replacement_future_text")]
    pub fn deprecated_future_text() {}

    pub struct MethodTester;

    impl MethodTester {
        #[deprecated(since = "1.0.0", note = "text", suggestion = "method_replacement")]
        pub fn method_deprecated(&self) {}
        #[deprecated(since = "1.0.0", note = "text", suggestion = "method_replacement_text")]
        pub fn method_deprecated_text(&self) {}
    }

    pub trait Trait {
        #[deprecated(since = "1.0.0", note = "text", suggestion = "trait_replacement")]
        fn trait_deprecated(&self) {}
        #[deprecated(since = "1.0.0", note = "text", suggestion = "trait_replacement_text")]
        fn trait_deprecated_text(&self) {}
    }

    impl Trait for MethodTester {}

    #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementStruct")]
    pub struct DeprecatedStruct {
        i: isize
    }
    pub struct UnstableStruct {
        i: isize
    }
    pub struct StableStruct {
        i: isize
    }

    #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementUnitStruct")]
    pub struct DeprecatedUnitStruct;

    pub enum Enum {
        #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementVariant")]
        DeprecatedVariant,
    }

    #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementTupleStruct")]
    pub struct DeprecatedTupleStruct(isize);

    mod nested {
        #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementStruct")]
        pub struct DeprecatedStruct {
            i: isize
        }

        #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementUnitStruct")]
        pub struct DeprecatedUnitStruct;

        pub enum Enum {
            #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementVariant")]
            DeprecatedVariant,
        }

        #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementTupleStruct")]
        pub struct DeprecatedTupleStruct(pub isize);
    }

    fn test() {
        use self::nested;

        // Only the deprecated cases of the following should generate
        // errors, because other stability attributes now have meaning
        // only *across* crates, not within a single crate.

        type Foo = MethodTester;
        let foo = MethodTester;

        deprecated(); //~ ERROR use of deprecated function `this_crate::deprecated`
        foo.method_deprecated(); //~ ERROR use of deprecated associated function `this_crate::MethodTester::method_deprecated`
        Foo::method_deprecated(&foo); //~ ERROR use of deprecated associated function `this_crate::MethodTester::method_deprecated`
        <Foo>::method_deprecated(&foo); //~ ERROR use of deprecated associated function `this_crate::MethodTester::method_deprecated`
        foo.trait_deprecated(); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`
        Trait::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`
        <Foo>::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`
        <Foo as Trait>::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`

        deprecated_text(); //~ ERROR use of deprecated function `this_crate::deprecated_text`: text
        foo.method_deprecated_text(); //~ ERROR use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text
        Foo::method_deprecated_text(&foo); //~ ERROR use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text
        <Foo>::method_deprecated_text(&foo); //~ ERROR use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text
        foo.trait_deprecated_text(); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text
        Trait::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text
        <Foo>::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text
        <Foo as Trait>::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text

        // Future deprecations are only permitted for rustc_deprecated.
        deprecated_future(); //~ ERROR use of deprecated function
        deprecated_future_text(); //~ ERROR use of deprecated function

        let _ = DeprecatedStruct {
            //~^ ERROR use of deprecated struct `this_crate::DeprecatedStruct`: text
            i: 0 //~ ERROR use of deprecated field `this_crate::DeprecatedStruct::i`: text
        };

        let _ = DeprecatedUnitStruct; //~ ERROR use of deprecated unit struct `this_crate::DeprecatedUnitStruct`: text

        let _ = Enum::DeprecatedVariant; //~ ERROR use of deprecated unit variant `this_crate::Enum::DeprecatedVariant`: text

        let _ = DeprecatedTupleStruct (1); //~ ERROR use of deprecated tuple struct `this_crate::DeprecatedTupleStruct`: text

        let _ = nested::DeprecatedStruct {
            //~^ ERROR use of deprecated struct `this_crate::nested::DeprecatedStruct`: text
            i: 0 //~ ERROR use of deprecated field `this_crate::nested::DeprecatedStruct::i`: text
        };

        let _ = nested::DeprecatedUnitStruct; //~ ERROR use of deprecated unit struct `this_crate::nested::DeprecatedUnitStruct`: text

        let _ = nested::Enum::DeprecatedVariant; //~ ERROR use of deprecated unit variant `this_crate::nested::Enum::DeprecatedVariant`: text

        let _ = nested::DeprecatedTupleStruct (1); //~ ERROR use of deprecated tuple struct `this_crate::nested::DeprecatedTupleStruct`: text
    }

    fn test_method_param<Foo: Trait>(foo: Foo) {
        foo.trait_deprecated(); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`
        Trait::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`
        <Foo>::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`
        <Foo as Trait>::trait_deprecated(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`
        foo.trait_deprecated_text(); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text
        Trait::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text
        <Foo>::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text
        <Foo as Trait>::trait_deprecated_text(&foo); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text
    }

    fn test_method_object(foo: &Trait) {
        foo.trait_deprecated(); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated`
        foo.trait_deprecated_text(); //~ ERROR use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text
    }

    #[deprecated(since = "1.0.0", note = "text")]
    fn test_fn_body() {
        fn fn_in_body() {}
        fn_in_body();
    }

    fn test_fn_closure_body() {
        let _ = || {
            #[deprecated]
            fn bar() { }
            bar(); //~ ERROR use of deprecated function `this_crate::test_fn_closure_body::{closure#0}::bar`
        };
    }

    impl MethodTester {
        #[deprecated(since = "1.0.0", note = "text", suggestion = "replacement_test_method_body")]
        fn test_method_body(&self) {
            fn fn_in_body() {}
            fn_in_body();
        }
    }

    #[deprecated(since = "1.0.0", note = "text", suggestion = "ReplacementTrait")]
    pub trait DeprecatedTrait {
        fn dummy(&self) { }
    }

    struct S;

    impl DeprecatedTrait for S { } //~ ERROR use of deprecated trait `this_crate::DeprecatedTrait`: text

    trait LocalTrait : DeprecatedTrait { } //~ ERROR use of deprecated trait `this_crate::DeprecatedTrait`: text
}

mod this_crate2 {
    struct Stable {
        #[deprecated(since = "1.0.0", note = "text", suggestion = "replacement2")]
        override2: u8,
    }

    struct Stable2(u8,
                   u8,
                   #[deprecated(since = "1.0.0", note = "text", suggestion = "1")] u8);

    #[deprecated(since = "1.0.0", note = "text", suggestion = "Replacement")]
    struct Deprecated {
        inherit: u8,
    }

    #[deprecated(since = "1.0.0", note = "text", suggestion = "Replacement2")]
    struct Deprecated2(u8,
                       u8,
                       u8);

    pub fn foo() {
        let x = Stable {
            override2: 3,
            //~^ ERROR use of deprecated field `this_crate2::Stable::override2`: text
        };

        let _ = x.override2;
        //~^ ERROR use of deprecated field `this_crate2::Stable::override2`: text

        let Stable {
            override2: _
            //~^ ERROR use of deprecated field `this_crate2::Stable::override2`: text
        } = x;
        // all fine
        let Stable { .. } = x;

        let x = Stable2(1, 2, 3);

        let _ = x.2;
        //~^ ERROR use of deprecated field `this_crate2::Stable2::2`: text

        let Stable2(_,
                   _,
                   _)
            //~^ ERROR use of deprecated field `this_crate2::Stable2::2`: text
            = x;
        // all fine
        let Stable2(..) = x;

        let x = Deprecated {
            //~^ ERROR use of deprecated struct `this_crate2::Deprecated`: text
            inherit: 1,
            //~^ ERROR use of deprecated field `this_crate2::Deprecated::inherit`: text
        };

        let _ = x.inherit;
        //~^ ERROR use of deprecated field `this_crate2::Deprecated::inherit`: text

        let Deprecated {
            //~^ ERROR use of deprecated struct `this_crate2::Deprecated`: text
            inherit: _,
            //~^ ERROR use of deprecated field `this_crate2::Deprecated::inherit`: text
        } = x;

        let Deprecated
            //~^ ERROR use of deprecated struct `this_crate2::Deprecated`: text
            // the patterns are all fine:
            { .. } = x;

        let x = Deprecated2(1, 2, 3);
        //~^ ERROR use of deprecated tuple struct `this_crate2::Deprecated2`: text

        let _ = x.0;
        //~^ ERROR use of deprecated field `this_crate2::Deprecated2::0`: text
        let _ = x.1;
        //~^ ERROR use of deprecated field `this_crate2::Deprecated2::1`: text
        let _ = x.2;
        //~^ ERROR use of deprecated field `this_crate2::Deprecated2::2`: text

        let Deprecated2
        //~^ ERROR use of deprecated tuple struct `this_crate2::Deprecated2`: text
            (_,
             //~^ ERROR use of deprecated field `this_crate2::Deprecated2::0`: text
             _,
             //~^ ERROR use of deprecated field `this_crate2::Deprecated2::1`: text
             _)
            //~^ ERROR use of deprecated field `this_crate2::Deprecated2::2`: text
            = x;
        let Deprecated2
        //~^ ERROR use of deprecated tuple struct `this_crate2::Deprecated2`: text
            // the patterns are all fine:
            (..) = x;
    }
}

fn main() {}
