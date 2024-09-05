//
// Rust Learn
// Copyright 2020-24, Alessio Saltarin
// This software is licensed under MIT license.
//


struct Person {
    name: String,
    surname: String,
}
impl Person {

    fn get_full_name(&self) -> String {
        let mut _name = self.name.to_string();
        let _space = " ".to_string();
        let _surname = self.surname.to_string();
        _name.push_str(&_space);
        _name.push_str(&_surname);
        _name
    }
}


pub(crate) fn classes_demo() {

    let alessio = Person {
        name: "Alessio".to_string(),
        surname: "Saltarin".to_string(),
    };
    println!("Alessio full name is {}", alessio.get_full_name());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_alessio_name() {
        let alessio = Person {
            name: "Alessio".to_string(),
            surname: "Saltarin".to_string(),
        };
        assert_eq!("Alessio Saltarin", alessio.get_full_name());
    }

}
