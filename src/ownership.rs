//
// Rust Learn
// Copyright 2020, Alessio Saltarin
// This software is licensed under MIT license.
//

pub(crate) fn ownerhip_demo() {

    let one_string = String::from("One String");
    let mut result: usize = take_ownership(one_string);
    println!("1. The 1 string size is = {}", result);

    // No! This violates OWNERSHIP rules
    // println!("Oh, the string was " + one_string);

    // This is OK
    let two_string = String::from("Two String");
    let results = return_ownership(two_string);
    println!("2. The 2 string size is = {}", results.0);

    // This is ALSO OK
    let three_string = String::from("Three String");
    result = operates_on_reference(&three_string);
    println!("3. The 3 string size is = {}", result);

}

fn take_ownership(a_string: String) -> usize {
    // This function TAKES OWNERSHIP of the a_string
    let string_len = a_string.len();
    string_len
}

fn return_ownership(a_string: String) -> (usize, String) {
    let string_len = a_string.len();
    (string_len, a_string)
}

fn operates_on_reference(a_string_reference: &String) -> usize {
    let string_len = a_string_reference.len();
    string_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_len_test() {
        let a_string = String::from("A test string");
        let string_size = operates_on_reference(&a_string);
        assert_eq!(13, string_size);
    }
}

