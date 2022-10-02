// define a struct as usual as a list of key:value fields
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// then use the impl keyword to start adding a method to
// the Rectangle struct
impl Rectangle {
    // the name of this function is going to be the name of
    // the method. We must use the '&self' keyword as the first
    // parameter.
    fn area(&self) -> u32 {
        // use the self keyword to reference the structs fields.
        return self.width * self.height;
    }

    // impl can add multiple methods to a struct. They can even have
    // the same name as a field which can then be used for purposes
    // like checking if a field was defined.
    fn width(&self) -> bool {
        return self.width > 0;
    }
}

// multiple impl statements can be declared for the same struct
// meaning methods can be added in multiple files.
impl Rectangle {
    // methods can also reference other instances of a rectangle such as
    // this example where we ask if another rectangle is small enough
    // to fit inside this one.
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.height > other.height && self.width > other.width;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 100
    };

    println!("The area of the rectangle is {} pixels.", rect1.area());

    // adding parenthesis tells Rust to use the method instead of the field
    if rect1.width() {
        // then not using parenthesis tells Rust to use the field
        println!("The width of rect1 is {}.", rect1.width);
    }

    let rect2 = Rectangle {
        width: 50,
        height: 50
    };

    let rect3 = Rectangle {
        width: 150,
        height: 150
    };

    println!("rect2 can fit into rect1: {}.", rect1.can_hold(&rect2));
    println!("rect3 can fit into rect1: {}.", rect1.can_hold(&rect3));
}
