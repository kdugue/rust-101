// My solution
fn solution(s: &str) -> String {
    let mut result = String::new();
    let mut count = 0;

    for ch in s.chars() {
        if count == 0 {
            result.push(ch);
        } else if ch.is_uppercase() {
            result.push(' ');
            result.push(ch);
        } else {
            result.push(ch);
        }
        count += 1;
    }

    result
}
