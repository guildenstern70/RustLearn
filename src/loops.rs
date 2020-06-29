//
// Rust Learn
// Copyright 2020, Alessio Saltarin
// This software is licensed under MIT license.
//

pub(crate) fn loop_way_one() -> i32 {
    let mut counter = 0;

    // Classic
    for x in 0..10 {
        counter += x;
    }

    counter = 0;

    // Loop keyword
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    result
}