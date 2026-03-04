pub fn data_types() -> (u8, f64, bool, char) {
    let num: u8 = 42;
    // 1. Define variable of type `u8` and value `42`
    let float: f64 = 3.14;
    // 2. Define variable of type `f64` and value `3.14`
    let boolean: bool = false;
    // 3. Define variable of type `bool` and value `false`
    let character: char = 'a';
    // 4. Define variable of type `char` and value `a`
    (num, float, boolean, character)
    // 5. Return a tuple with the variables in the order they were defined
}
