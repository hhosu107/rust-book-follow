#[derive(Debug)] // Adding Useful Functionality with Derived Traits
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // println!("rect1 is {}", rect1); // Error: `Rectangle` doesn't implement `std::fmt::Display`
    // // `Rectangle` doesn't implement `std::fmt::Display`
    // NOTE: in format string you may be able to use {:?} (or {:#?} for pretty-print) instead
    // - {:?} - tells `println!` we want to use an output format `Debug`.
    // - {:#?} - pretty-print
    // NOTE: just using :? is insufficient. We have to implement the `fmt::Debug` trait on the
    // type. It is covered by #[derive(Debug)], which adds outer attribute.
    println!("rect1 is {:?}", rect1);
    // More easier to read - for instance, adds line-break, etc.
    println!("rect1 is {:#?}", rect1);

    // NOTE: `dbg!` takes ownership of an expression, prints the file and line number of where that
    // `dbg!` macro call occurs in the code along with the resultant value of that expression, and
    // returns ownership of the value.
    // To avoid giving ownership of `rect1`, we use `&rect1` instead.
    dbg!(&rect1); // Calling `dbg!` macro prints to the `stderr`.
}

fn area(rectangle: &Rectangle) -> u32 {
    // Signature of this method was originally `width: i32, height: i32`.
    // This is very unclear and haven't been bound with Rectangle.
    // What about using tuple? It is still unclear because we depend on the indices.
    // So we changed it to get the reference of Rectangle.
    rectangle.width * rectangle.height
}
