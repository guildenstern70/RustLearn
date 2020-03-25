//
// Rust Learn
// Copyright 2020, Alessio Saltarin
// This software is licensed under MIT license.
//

mod collections;
mod variables;

const VERSION: &str = "0.1.0";

fn main() {
    println!("Rust Learn v.{}", VERSION);

    sep("Variables");
    variables::the_variables();
    sep("Collections");
    collections::an_array_of_strings();
    collections::a_vector_of_numbers();
    sep("");
}

fn sep(name: &str) {
    println!("***********************************************");
    if name.len() > 0 {
        println!("  {}", name);
        println!("***********************************************");
    }
}
