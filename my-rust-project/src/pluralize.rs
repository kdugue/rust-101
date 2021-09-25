pub fn make_plural() {
    let str = String::from("book");
    let new_str = str.clone();

    println!("I have one {}, you have two {}", str, pluralize(new_str));
}

fn pluralize(singular: String) -> String {
    singular + "s"
}
