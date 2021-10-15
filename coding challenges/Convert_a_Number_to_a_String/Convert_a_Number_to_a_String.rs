fn number_to_string(i: i32) -> String {
    let num_string = i.to_string();
    num_string // no semi colon because this is what is being returned
}

// alternative solution
fn number_to_string(i: i32) -> String {
    i.to_string()
}
