fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Liftoff!");

    // for loop
    for num in (1..4) {
        println!("{}!", num);
    }
    println!("Liftoff!");
}
