pub fn get_tuple() {
    let tup = (1, 'c', true);
    let first = tup.0; //example of tuple indexing
    let second = tup.1;
    println!("the first is {}", first);
    println!("the second is {}", second);

    // destructuring
    let (x, y, z) = tup;
    println!("the first is {}", x);
    println!("the second is {}", y);
    println!("the third is {}", z);
}
