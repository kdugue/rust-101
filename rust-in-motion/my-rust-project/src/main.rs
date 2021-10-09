use std::io;

mod arrays;
mod borrowing;
mod loops;
mod matches;
mod pluralize;
mod slices;
mod structs;
mod tuples;

fn main() {
    let mut x = 5;
    x += 1;
    println!("x is now {}", x);

    next_birthday("Kemet", 28);
    sum(3);
    discount(2);
    secret_word();

    loops::for_loop();
    matches::get_match();
    structs::get_struct();
    tuples::get_tuple();
    arrays::get_array();

    slices::get_slice();
    slices::vectors();

    pluralize::make_plural();
    borrowing::call();
}

fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!(
        "Hi {}, on your next birthday, you'll be {}!",
        name, next_age
    );
}

fn sum(num: i32) -> i32 {
    // no semi colon means we want the
    // resulting value of expression returned
    // out of the fn body
    num * num
}

fn discount(day_of_month: u8) {
    let amount = if day_of_month % 2 == 0 { 50 } else { 10 };

    println!("Your discount is {}%!", amount);
}

fn secret_word() {
    let mut word = String::new();

    while word.trim() != "rust" {
        println!("What's the sercet word?");
        word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
    }

    println!("You know the secret word! Please proceed");
}
