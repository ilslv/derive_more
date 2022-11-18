// The following code is from https://github.com/withoutboats/display_derive/blob/232a32ee19e262aacbd2c93be5b4ce9e89a5fc30/tests/tests.rs
// Written by without boats originally

use derive_more::Display;

#[derive(Display)]
#[display("An error has occurred.")]
struct UnitError;

#[test]
fn unit_struct() {
    let s = format!("{}", UnitError);
    assert_eq!(&s[..], "An error has occurred.");
}

#[derive(Display)]
#[display("Error code: {}", code)]
struct RecordError {
    code: u32,
}

#[test]
fn record_struct() {
    let s = format!("{}", RecordError { code: 0 });
    assert_eq!(&s[..], "Error code: 0");
}

#[derive(Display)]
#[display("Error code: {}", _0)]
struct TupleError(i32);

#[test]
fn tuple_struct() {
    let s = format!("{}", TupleError(2));
    assert_eq!(&s[..], "Error code: 2");
}

#[derive(Display)]
enum EnumError {
    #[display("Error code: {}", code)]
    StructVariant { code: i32 },
    #[display("Error: {}", _0)]
    TupleVariant(&'static str),
    #[display("An error has occurred.")]
    UnitVariant,
}

#[test]
fn enum_error() {
    let s = format!("{}", EnumError::StructVariant { code: 2 });
    assert_eq!(&s[..], "Error code: 2");
    let s = format!("{}", EnumError::TupleVariant("foobar"));
    assert_eq!(&s[..], "Error: foobar");
    let s = format!("{}", EnumError::UnitVariant);
    assert_eq!(&s[..], "An error has occurred.");
}
