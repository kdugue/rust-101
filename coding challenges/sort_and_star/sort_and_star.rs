fn two_sort(arr: &[&str]) -> String {
    let mut v = arr.to_vec(); // copies to a vector
    v.sort();
    v[0].chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>() // turns interable into collection
        .join("***") //turns back into string
}

// Solution 2
fn two_sort(arr: &[&str]) -> String {
    arr.iter()
        .min()
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("***")
}

// Solution 3
fn two_sort(arr: &[&str]) -> String {
    arr.iter()
        .min()
        .unwrap()
        .chars()
        .fold(String::new(), |acc, s| format!("{}***{}", acc, s))[3..]
        .to_string()
}
