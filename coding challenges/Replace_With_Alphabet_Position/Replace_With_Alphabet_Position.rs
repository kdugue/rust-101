use std::collections::HashMap;

fn alphabet_position(text: &str) -> String {
    let mut map = HashMap::new();
    let alphabet_str = "abcdefghijklmnopqrstuvwxyz";

    let mut count = 1;
    let mut result = String::new();

    for ch in alphabet_str.chars() {
        map.insert(ch, count.to_string());
        count += 1;
    }

    let mut position = String::new();

    for (i, ch) in text.to_lowercase().chars().enumerate() {
        position = map.get(&ch).unwrap_or(&" ".to_string()).to_string();

        if position != " " {
            result.push_str(&position);
        }
        if i != text.len() - 1 && position != " " {
            result.push(' ');
        }
    }

    result.trim().to_string()
}

// TODO: Add alternate, more efficient solution
