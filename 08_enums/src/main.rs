// an enum can contain a variey of values each of which can contain 
// any data type. In this example for instance the Quit value has no
// data type while ChangeColor is a tuple.
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("{:#?}", &self);
    }
}

fn main() {
    // we can create an instance of the enum using the :: notation
    // the word on the left is the enum name and the word on the
    // right is the value we want to instantiate.
    let m1 = Message::Quit;
    // method are then called in the same way as a structs methods are
    m1.call();

    let m2 = Message::Move{x: 1, y: 1};
    m2.call();

    let m3 = Message::Write(String::from("hello"));
    m3.call();

    let m4 = Message::ChangeColor(44, 44, 44);
    m4.call();
}
