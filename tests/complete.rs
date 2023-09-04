// Bring the procedural macro into scope
use nullable_struct::Nullable;

#[derive(Nullable)]
struct TestStruct {
    field1: i32,
    field2: String,
}

#[test]
fn test_new() {
    let s = NullableTestStruct::new(42, "hello".to_string());
    assert_eq!(s.field1(), 42);
    assert_eq!(s.field2(), "hello".to_string());
}

#[test]
fn test_new_default() {
    let s = NullableTestStruct::new_default();
    assert_eq!(s.field1(), 0); // Default for i32 is 0
    assert_eq!(s.field2(), ""); // Default for String is ""
}

#[test]
fn test_getters() {
    let mut s = NullableTestStruct::new(42, "hello".to_string());
    assert_eq!(s.get_field1(), Some(&42));
    assert_eq!(s.get_field2(), Some(&"hello".to_string()));

    s = NullableTestStruct::default();
    assert_eq!(s.get_field1(), None);
    assert_eq!(s.get_field2(), None);
}

#[test]
fn test_setters() {
    let mut s = NullableTestStruct::default();
    s.set_field1(42);
    s.set_field2("hello".to_string());

    assert_eq!(s.field1(), 42);
    assert_eq!(s.field2(), "hello".to_string());
}

#[test]
fn test_default() {
    let s = NullableTestStruct::default();
    assert_eq!(s.field1(), 0); // Default for i32 is 0
    assert_eq!(s.field2(), ""); // Default for String is ""
}