//
// Rust Learn
// Copyright 2020, Alessio Saltarin
// This software is licensed under MIT license.
//

mod collections;
mod variables;
mod loops;
mod functions;
mod ownership;

const VERSION: &str = "0.1.4";

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
    let result = loops::loop_ways();
    println!(" Loop Result = {}", result);
    sep("Ownerhip");
    ownership::ownerhip_demo();
}

fn sep(name: &str) {
    println!("\n***********************************************");
    if name.len() > 0 {
        println!("  {}", name);
        println!("***********************************************");
    }
}
