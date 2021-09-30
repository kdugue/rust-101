// My solution
fn abbrev_name(name: &str) -> String {
    let mut split = name.split_whitespace();
    let mut first_name = String::new();
    let mut last_name = String::new();

    let mut result = String::new();

    let mut count = 0;

    for word in split {
        if count == 0 {
            first_name.push_str(word);
        } else {
            last_name.push_str(word);
        }
        count = count + 1;
    }

    //loop over first name
    for ch in first_name.chars() {
        result.push(ch);
        break;
    }

    result.push('.');

    for ch in last_name.chars() {
        result.push(ch);
        break;
    }

    result.to_ascii_uppercase()
}

// alternate solution
// TODO: explain this
fn abbrev_name(name: &str) -> String {
    name.split(' ')
        .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
        .collect::<Vec<_>>()
        .join(".")
}

// efficient solution
fn abbrev_name(name: &str) -> String {
    // split returns an iterator
    let mut names = name.split(" ");

    //.next() will grab next value in iterator
    // unwrap returns the value itse;f
    let first = names.next().unwrap();
    let second = names.next().unwrap();

    // grab first letter, uppercase it, conver from str to String
    // second has to be reference since the add signature
    // requires reference type for the str being added onto the initial str
    // initial str will be owned
    // self + &str
    first[0..1].to_uppercase().to_string() + "." + &second[0..1].to_uppercase();
}
