//
// Rust Learn
// Copyright 2020-24, Alessio Saltarin
// This software is licensed under MIT license.
//

enum NumberEnum {
    One,
    Two,
    Three,
    Four,
}

pub(self) fn enum_match(number: NumberEnum) -> i32 {
    match number {
        NumberEnum::One => 1,
        NumberEnum::Two => 2,
        NumberEnum::Three => 3,
        NumberEnum::Four => 4,
    }
}

pub(crate) fn enum_demo() {
    let mut counter = 0;

    counter += enum_match(NumberEnum::One);
    counter += enum_match(NumberEnum::Two);
    counter += enum_match(NumberEnum::Three);
    counter += enum_match(NumberEnum::Four);

    println!("Counter is {}", counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_match_test() {
        assert_eq!(3, enum_match(NumberEnum::Three));
    }
}