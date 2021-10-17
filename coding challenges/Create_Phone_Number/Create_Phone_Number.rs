fn create_phone_number(numbers: &[u8]) -> String {
    let mut result = String::new();
    result.push('(');
    let mut count = 0;

    for ch in numbers.clone() {
        count += 1;
        let num_str = ch.to_string().clone();
        result.push_str(&num_str);
        if count == 3 {
            result.push(')');
            result.push(' ');
        } else if count == 6 {
            result.push('-');
        }
    }

    result
}
