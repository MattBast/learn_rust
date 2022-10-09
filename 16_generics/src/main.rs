// functions are not the only ones that can use generic types.
// structs can use them as well if they would like their fields
// to accept any type.
struct Point<T> {
    x: T,
    y: T,
}

// this is how the syntax differs for adding methods to structs of a 
// generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// even though Point uses geenric types we can still define methods that
// expect a certain type. In this case we are adding a method that will only
// work if the developer used the f32 type for x and y. If they use another 
// type when initialising the struct, this method will not be defined for it.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// this struct allows for different types for the two fields
struct PointTwo<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // these structs must always contain the same type for both fields. 
    // We would get an error if we passed an integer to x and a float 
    // to y inside the same struct.
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("{}, {}", integer_point.x, integer_point.y);
    println!("{}, {}", float_point.x, float_point.y);

    // this struct on the other hand can use different types in its two fields
    let varied_point = PointTwo { x: 1, y: 4.0 };
    println!("{}, {}", varied_point.x, varied_point.y);
}

// we can use any word to describe a generic type but the Rust convention is to use
// an uppercase "T". This needs to be placed inside angle brackets between the function
// name and the parameters starting circle bracket as well as within the parameter brackets
// themselves. There are some restrictions to using generics though. In this case we can
// only accept types that can be ordered (have the PartialOrd trait) so we need to specify
// this when defining the function.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}