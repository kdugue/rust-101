// Can't just return "true" string literal
// this is because string literals represent a reference
// we need to return a new string

fn boolean_to_string(b: bool) -> String {
    if b == true {
        String::from("true")
    } else {
        String::from("false")
    }
}

// alternative
fn boolean_to_string(b: bool) -> String {
    if b == true {
        "true".to_string()
    } else {
        "false".to_string()
    }
}
