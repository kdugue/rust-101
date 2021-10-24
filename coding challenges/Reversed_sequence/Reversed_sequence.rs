fn reverse_seq(n: u32) -> Vec<u32> {
    let mut result = Vec::new();

    for num in (1..=n).rev() {
        result.push(num);
    }

    result
}
