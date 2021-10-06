// My solution

fn accum(s: &str) -> String {
    let mut result = String::new();

    let mut count = 0;
    let mut temp_count = 0;

    for ch in s.chars() {
        temp_count = 0;
        count += 1;

        let uppercase_char = ch.to_uppercase().next().expect("string is empty");
        result.push(uppercase_char);
        temp_count += 1;

        while temp_count < count {
            let lowercase_char = ch.to_lowercase().next().expect("string is empty");
            result.push(lowercase_char);
            temp_count += 1;
        }

        if count != s.len() {
            result.push('-');
        }
    }

    result
}

// TODO: add alternate solutions
