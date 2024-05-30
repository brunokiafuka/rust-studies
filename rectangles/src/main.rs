#[derive(Debug)] // used to allow struct to be printed
struct Rectangle {
    // structs are basically interfaces
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // area is a method that implements the Rectangle interface
        &self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 70,
        height: 80,
    };

    let rect2 = Rectangle { width: 60, ..rect1 };

    dbg!(&rect1); // debug struct
    println!("rect2 is {:#?}", rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

// fn area(dimensions: Rectangle) -> u32 {
//     dimensions.width * dimensions.height
// }
