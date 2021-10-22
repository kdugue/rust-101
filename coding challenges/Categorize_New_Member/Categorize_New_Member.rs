fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut result = Vec::new();

    for elem in data.into_iter() {
        if elem.0 >= 55 && elem.1 > 7 {
            result.push(String::from("Senior"));
        } else {
            result.push(String::from("Open"));
        }
    }

    result
}

// TODO: add alternate solution
