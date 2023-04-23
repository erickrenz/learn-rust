#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let square1 = Rectangle::square(3);
    dbg!(&square1);

    // dbg! both takes and returns ownership of the value it's given.
    dbg!(&rect1);

    println!(
        "The area of this rectangle is {} square pixels.",
        rect1.area()
    );
}
