//
// Rust Learn
// Copyright 2020-24, Alessio Saltarin
// This software is licensed under MIT license.
//


// pub(create) means: This function is visible within this crate (library/namespace)
pub(crate) fn an_array_of_strings() {
    println!("The third month of the year is {}", get_month_name(2));
}


pub(crate) fn a_vector_of_numbers() {
    let second_element = get_number(1);
    println!("Second Element is {:?}", second_element);
}

fn get_number(index: usize) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let number_option = v.get(index);
    match number_option{
        None => panic!("Bad index"),
        Some(&number) => {
            number
        },
    }
}

fn get_month_name(index: usize) -> String {
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    String::from(months[index - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_month_name_test() {
        assert_eq!("February", get_month_name(2));
    }

    #[test]
    fn get_number_test() {
        let number_option = get_number(2);
        assert_eq!(7, number_option);
    }

    #[test]
    #[should_panic]
    fn get_non_existent_number_test() {
        let _number_option = get_number(123);
    }
}
