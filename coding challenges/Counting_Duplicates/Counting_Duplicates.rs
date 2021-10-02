fn count_duplicates(text: &str) -> u32 {
    use std::collections::HashMap;

    let lower_case_text = text.to_lowercase();
    let mut map = HashMap::new();
    let mut duplicates_count = 0;

    for ch in lower_case_text.chars() {
        let count = map.entry(ch).or_insert(0);
        // * dereferences to be able to modify value
        *count += 1;
    }

    for (key, value) in map {
        if value > 1 {
            duplicates_count += 1;
        }
    }

    duplicates_count
}

// alternate solution

fn count_duplicates(text: &str) -> u32 {
    use std::collections::HashMap;
    let mut char_count: HashMap<char, u32> = HashMap::new();
    for c in text.to_lowercase().chars() {
        let mut e = char_count.entry(c).or_default();
        *e += 1;
    }

    // count returns usize, which is why it is being casted to u32
    // usize is pointer-sized, depends on the architecture you're compiling program for
    char_count.values().filter(|&&v| v > 1).count() as u32
}
