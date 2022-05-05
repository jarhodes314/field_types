#![allow(dead_code)]

extern crate variant_count;
extern crate field_types;
extern crate strum_macros;
extern crate strum;

use variant_count::VariantCount;
use field_types::FieldName;
use strum_macros::EnumString;

#[derive(FieldName)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_name(skip)]
    third: bool,
    #[field_name = "skip"]
    fourth: bool,
}

#[derive(FieldName)]
#[field_name_derive(VariantCount, Debug, Clone, PartialEq)]
struct TestGen<'a, T: 'a, U>
    where U: 'a
{
    first: T,
    second_field: Option<&'a U>,
    #[field_name(skip)]
    third: &'a T,
    #[field_types = "skip"]
    fourth: U,
}

#[derive(FieldName)]
#[field_types_derive(VariantCount, Debug, Clone, PartialEq)]
struct TestTypesDerive {
    first: i32,
    second: bool,
}

#[derive(FieldName)]
#[field_name_derive(VariantCount, Debug, Clone, PartialEq)]
struct TestNameDerive {
    first: i32,
    second: bool,
}

#[derive(FieldName)]
#[field_name_derive(VariantCount, Debug, Clone, PartialEq, EnumString)]
#[field_name_attr(strum(ascii_case_insensitive))]
struct TestDeriveArguments {
    first: i32,
    second_field: bool,
}

#[test]
fn full_field_name_variants() {
    let _field = TestFieldName::First;
    let field = TestFieldName::SecondField;
    match field {
        TestFieldName::First => (),
        TestFieldName::SecondField => (),
    }

    let _field = TestGenFieldName::First;
    let field = TestGenFieldName::SecondField;
    match field {
        TestGenFieldName::First => (),
        TestGenFieldName::SecondField => (),
    }

    let _field = TestTypesDeriveFieldName::First;
    let field = TestTypesDeriveFieldName::Second;
    match field {
        TestTypesDeriveFieldName::First => (),
        TestTypesDeriveFieldName::Second => (),
    }

    let _field = TestNameDeriveFieldName::First;
    let field = TestNameDeriveFieldName::Second;
    match field {
        TestNameDeriveFieldName::First => (),
        TestNameDeriveFieldName::Second => (),
    }
}

#[test]
fn derive_field_name() {
    let name = TestFieldName::First;
    assert_eq!(TestFieldName::First, name);
    assert_ne!(TestFieldName::SecondField, name);

    let name = TestGenFieldName::First;
    assert_eq!(TestGenFieldName::First, name);
    assert_ne!(TestGenFieldName::SecondField, name);

    let name = TestTypesDeriveFieldName::First.clone();
    assert_eq!(TestTypesDeriveFieldName::First, name);
    assert_ne!(TestTypesDeriveFieldName::Second, name);

    let name = TestTypesDeriveFieldName::Second.clone();
    assert_eq!(TestTypesDeriveFieldName::Second, name);
    assert_ne!(TestTypesDeriveFieldName::First, name);
}

#[test]
fn into_field_name() {
    let test = Test {
        first: 1,
        second_field: Some("test".to_string()),
        third: true,
        fourth: true,
    };
    let fields: [TestFieldName; 2] = (&test).into();
    assert!(match fields {
        [TestFieldName::First, TestFieldName::SecondField] => true,
        _ => false,
    });

    let message = "test".to_string();
    let test = TestGen {
        first: 1,
        second_field: Some(&message),
        third: &2,
        fourth: message.clone(),
    };
    let fields: [TestGenFieldName; TestGenFieldName::VARIANT_COUNT] = (&test).into();
    assert!(match fields {
        [TestGenFieldName::First, TestGenFieldName::SecondField] => true,
        _ => false,
    });

    let test = TestTypesDerive {
        first: 1,
        second: true,
    };
    let fields: [TestTypesDeriveFieldName; TestTypesDeriveFieldName::VARIANT_COUNT] = (&test).into();
    assert_eq!(TestTypesDeriveFieldName::First, fields[0]);
    assert_eq!(TestTypesDeriveFieldName::Second, fields[1]);
}

#[test]
fn field_name_str() {
    assert_eq!(TestFieldName::First.name(), "first");
    assert_eq!(TestFieldName::SecondField.name(), "second_field");

    assert_eq!(Some(TestFieldName::First), TestFieldName::by_name("first"));
    assert_eq!(Some(TestFieldName::SecondField), TestFieldName::by_name("second_field"));
    assert_eq!(None, TestFieldName::by_name("third"));
}

#[test]
fn field_name_fromstr() {
    assert_eq!("FIRST".parse::<TestDeriveArgumentsFieldName>().unwrap(), TestDeriveArgumentsFieldName::First);
    assert_eq!("first".parse::<TestDeriveArgumentsFieldName>().unwrap(), TestDeriveArgumentsFieldName::First);
    assert_eq!("secondField".parse::<TestDeriveArgumentsFieldName>().unwrap(), TestDeriveArgumentsFieldName::SecondField);
}
