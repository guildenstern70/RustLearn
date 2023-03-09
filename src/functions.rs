//
// Rust Learn
// Copyright 2020-23, Alessio Saltarin
// This software is licensed under MIT license.
//

// pub(create) means: This function is visible within this crate (library/namespace)
pub(crate) fn a_simple_sum(a: i32, b: i32) -> i32 {
    return implicit_return(a, b)
}

// Last value is implicitly returned
// Note: there is no final ;
fn implicit_return(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_simple_sum_test() {
        assert_eq!(264, a_simple_sum(234, 30));
    }
}


