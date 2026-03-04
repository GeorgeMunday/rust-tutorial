pub fn mutating_variables() -> String {

    let mut text = String::from("hello");
    text.push_str(" world");
    mutates_value(&mut text);
    text
    // 1. Declare a mutable variable `text` with value "hello"

    // 2. Call `mutates_value` with a mutable reference to `text`

    // 3. Return the value of `text` as a String
}

// Do not change this function
pub fn mutates_value(value: &mut String) {
    *value = String::from("bye")
}
