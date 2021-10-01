// My solution

fn odd_or_even(numbers: Vec<i32>) -> String {
    // mut because sum will be modified
    let mut sum = 0;

    // can loop through vector with for in
    // why don't I need the .iter()
    for num in numbers {
        sum = sum + num;
    }

    // returning String
    if sum % 2 == 0 {
        String::from("even")
    } else {
        String::from("odd")
    }
}

// TODO: add alternate solutions
