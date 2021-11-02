fn find_smallest_int(arr: &[i32]) -> i32 {
    let mut result = arr[0];

    for num in arr.clone() {
        if num < &result {
            result = *num;
        }
    }

    result
}

// TODO: add alternate solution
