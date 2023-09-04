use nullable_struct::Nullable;

#[derive(Nullable)]
struct MyStruct {
    field1: i32,
    field2: String,
}

fn main() {
    let mut instance = NullableMyStruct::new(42, "Hello".to_string());
    println!("Field1: {}", instance.field1());  // Output: 42
    println!("Field2: {}", instance.field2());  // Output: Hello

    instance.set_field1(13);
    instance.set_field2("World".to_string());

    if let Some(value) = instance.get_field1() {
        println!("Field1 exists: {}", value); // Output: 13
    }

    if let Some(value) = instance.get_field2() {
        println!("Field2 exists: {}", value); // Output: World
    }
}
