fn number_to_string(i: i32) -> String {
    let numString = i.to_string();
    numString // no semi colon because this is what is being returned
}

// alternative solution
fn number_to_string(i: i32) -> String {
    i.to_string()
}
