struct Person {
    name: String,
}

fn congratulate(person: &Person) {
    println!("Congratulations, {}!!!", person.name);
}

pub fn call() {
    let p = Person {
        name: String::from("Jake"),
    };

    congratulate(&p);
    println!("Can still use p here: {}", p.name);

    println!("**************");

    let s = String::from("toy");
    let pl = pluralize(&s);
}

fn pluralize(str: &str) -> String {
    str.to_owned() + "s"
}
