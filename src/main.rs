//
// Rust Learn
// Copyright 2020, Alessio Saltarin
// This software is licensed under MIT license.
//

mod collections;
mod variables;
mod loops;
mod functions;

const VERSION: &str = "0.1.0";

fn main() {
    println!("Rust Learn v.{}", VERSION);

    sep("Variables");
    variables::the_variables();
    sep("Collections");
    collections::an_array_of_strings();
    collections::a_vector_of_numbers();
    sep("Functions");
    let twelve = functions::a_simple_sum(7, 5);
    println!(" 7 + 5 = {}", twelve);
    sep("Loops");
    let result = loops::loop_way_one();
    println!(" Loop Result = {}", result);
}

fn sep(name: &str) {
    println!("\n***********************************************");
    if name.len() > 0 {
        println!("  {}", name);
        println!("***********************************************");
    }
}
