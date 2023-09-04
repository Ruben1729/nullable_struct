# Nullable Struct
[<img alt="github" src="https://img.shields.io/badge/github-ruben1729/nullable_struct-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Ruben1729/nullable_struct)
[<img alt="crates.io" src="https://img.shields.io/crates/v/nullable_struct.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/nullable_struct)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.nullable_struct-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/nullable_struct)

## Introduction
`nullable_structs` is a Rust crate that provides a Nullable derive macro. This macro makes it incredibly easy to create structs where each field is wrapped in Option<T>, enabling each field to be nullable. In addition, the crate generates convenient getter and setter methods for working with these fields.

```toml
[dependencies]
nullable_struct = "0.1.0"
```

## Features
- Wraps each field of a struct in an Option<T>.
- Generates getter methods that return either the wrapped value or a default value. 
- Generates getter methods that return Option<&T>. 
- Generates setter methods for updating the value of each field. 
- Provides a constructor to initialize each field with None. 
- Implements the Default trait, initializing all fields to None.

## Example
Here is a basic example demonstrating how to use `nullable_struct`.

```rust
extern crate nullable_structs;
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
```

## License
This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
