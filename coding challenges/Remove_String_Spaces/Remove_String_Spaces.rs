// My solution

fn no_space(x: String) -> String {
    let mut result = String::new();

    for ch in x.chars() {
        if ch != ' ' {
            result.push(ch);
        }
    }

    result
}

// alternate solution
fn no_space(x: String) -> String {
    x.replace(" ", "")
}

// alternate solution
fn no_space(x: String) -> String {
    x.chars().filter(|c| !c.is_whitespace()).collect()
}

// alternate solution
fn no_space(x: String) -> String {
    x.split_whitespace().collect()
}
