fn solve(s: &str) -> String {
    let mut lower_count = 0;
    let mut upper_count = 0;

    for ch in s.chars() {
        if ch.to_string() == ch.to_string().to_lowercase() {
            lower_count += 1
        } else {
            upper_count += 1;
        }
    }

    if lower_count >= upper_count {
        s.to_string().to_lowercase()
    } else {
        s.to_string().to_uppercase()
    }
}

// TODO: add idiomatic Rust solution
