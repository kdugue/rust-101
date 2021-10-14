fn solution(s: &str) -> Vec<String> {
    let str = String::from(s);

    let mut count = 0;
    let mut result = Vec::new();
    let mut temp_str = String::new();

    for char in str.chars() {
        count += 1;
        temp_str.push(char);
        if count % 2 == 0 {
            result.push(temp_str);
            temp_str = String::new();
        }
    }

    if temp_str.len() != 0 {
        temp_str.push('_');
        result.push(temp_str);
    }

    result
}

// TODO: Add alternate, idiomatic approach
