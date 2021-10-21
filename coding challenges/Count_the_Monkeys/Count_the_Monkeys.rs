fn monkey_count(n: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 1..n + 1 {
        result.push(i);
    }

    result
}

// Alternate Solution
fn monkey_count(n: i32) -> Vec<i32> {
    // =n includes loop through n
    // collect will add values into a
    // data structure
    (1..=n).collect()
}
