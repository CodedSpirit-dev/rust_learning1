fn main() {
    println!("{},{}!", "Hello", "world"); // Hello, world!
    println!("{0},{1}!", "Hello", "world"); // Hello, world!
    println!("{greeting},{name}!", greeting="Hello", name="world"); // Hello, world!

    let name = "world";
    println!("Hello, {name}!"); // Hello, world!

    let name = "world";
    println!("Hello, {name}!"); // Hello, world!


    let (greeting, name) = ("Hello", "world");
    println!("Hello, {greeting} {name}!"); // Hello, world!


    println!("{:?}", [1, 2, 3]); // [1, 2, 3]
    println!("{:#?}", [1, 2, 3]); //
    /*
    [
        1,
        2,
        3,
    ]
    */

    // The format! macro is used to store the formatted string in a variable.
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello world!

    // Rust has a print!() macro that is similar to println!() but does not add a newline.
    print!("Hello, world!"); // Hello, world! without a newline
    println!(); // Prints a newline

    print!("Hello, world!\n"); // Hello, world! With a newline

}
