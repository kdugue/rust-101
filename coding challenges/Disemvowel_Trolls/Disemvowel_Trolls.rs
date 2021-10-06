fn disemvowel(s: &str) -> String {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let vowels = "aeiouAEIOU";

    for ch in vowels.chars() {
        map.insert(ch, true);
    }

    let mut result = String::new();

    for ch in s.chars() {
        if !map.contains_key(&ch) {
            result.push(ch);
        }
    }

    result
}
