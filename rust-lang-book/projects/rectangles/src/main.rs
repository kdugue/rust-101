#[derive(Debug)] // allows to print out debugging information for struct

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // rectangle argument is an immutable borrow of the Rectangle struct
    // main fn retains ownership of the struct and can
    // continue using 'rect1'
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, compare_rectangle: &Rectangle) -> bool {
        compare_rectangle.width <= self.width && compare_rectangle.height <= self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 is {:?}", rect1);

    println!("******");

    // Another way to print out formatting
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
