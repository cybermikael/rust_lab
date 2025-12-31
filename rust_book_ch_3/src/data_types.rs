pub fn dtypes() {
    // Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so that it knows how to work with that data. We'll look at two data type subsets: scalar and compound.
    
    // Data Types
    println!("| Data Types |\n");
    // Scalar Types
    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    println!("Integers");
    let integer: u32 = "42".parse().expect("Not a number!");
    println!("The integer 32 bit variable: {integer}\n");

    println!("Foating Point Numbers");
    let floating: f32 = 3.1416;
    println!("The floating 32 but variable: {floating}\n");

    println!("Numeric Operations");
    let quotient = 56.7 / 32.2;
    println!("The numeric quotient operation: {quotient}\n");

    println!("Boolean");
    let f: bool = true;
    println!("The boolean variable: {f}\n");

    // Characters must be specified with single quotation marks, opposed to strings that required double quotes.
    println!("The Character");
    let z: char = 'Z';
    println!("The character variable: {z}\n");

    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    println!("Tuple");
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let one = tup.2;
    let (_x, y, _z) = tup;
    println!("The value of y in the tuple: {y}");
    println!("Five hundred: {five_hundred}");
    println!("One: {one}\n");

    // Array
    // Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far, rather than the heap (we will discuss the stack and the heap more in Chapter 4) or when you want to ensure that you always have a fixed number of elements.
    let a: [i32; 5] = [1,2,3,4,5];
    let first = a[0];
    println!("The array: {first}\n");



}