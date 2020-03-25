//
// Rust Learn
// Copyright 2020, Alessio Saltarin
// This software is licensed under MIT license.
//

pub(crate) fn the_variables() {
    const RUNTIME_CONST: i32 = 10_000;
    const RUNTIME_STRING: &str = "OK";

    println!("This is a runtime int: {}", RUNTIME_CONST);
    println!("This is a runtime string: {}", RUNTIME_STRING);

    let immutable = 5;

    println!("This immutable variable holds number {}", immutable);

    let mut mutable_float = 27.0;
    mutable_float += 23.22;

    println!("This mutable variable holds number {}", mutable_float);
}