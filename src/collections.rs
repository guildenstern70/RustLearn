//
// Rust Learn
// Copyright 2020, Alessio Saltarin
// This software is licensed under MIT license.
//
pub(crate) fn a_list_of_numbers() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let second_element = v.get(2);
    let mut total = 0;

    for i in &v {
        total += *i;
    }

    println!("Second Element is {:?}", second_element.unwrap());
    println!("Sum is {:?}", total);
}
