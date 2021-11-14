fn generate_shape(n: i32) -> String {
    let mut result = String::new();
    let mut count = 0;

    for _ in 0..n {
        for _ in 0..n {
            result.push_str("+");
        }
        count += 1;
        if count != n {
            result.push_str("\n");
        }
    }

    result
}

// TODO: Add alternate solution
