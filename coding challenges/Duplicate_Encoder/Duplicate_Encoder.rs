// My solution
fn duplicate_encode(word: &str) -> String {
    use std::collections::HashMap;
    let lower_case = word.to_lowercase();
    let mut map = HashMap::new();
    let mut result = String::new();

    for ch in lower_case.chars() {
        let count = map.entry(ch).or_insert(0);
        *count += 1;
    }

    for ch in lower_case.chars() {
        let val = map.get(&ch).unwrap().clone();
        if val > 1 {
            result.push(')');
        } else {
            result.push('(');
        }
    }

    result
}

// TODO: understand alternate solutions
