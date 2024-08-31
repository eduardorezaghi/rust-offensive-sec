fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // mut allows us to change the value of x
    x = x + 5;
    println!("The value of x is: {}", x);

    // Constants
    // -- unlike let, constants must be annotated with a type and are always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    let boolean: bool = true;

    let character: char = '😻';
    
    let integer: i32 = -42;

    let a_float: f64 = 3.14;

    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("Integer: {}", integer);
    println!("Float: {}", a_float);


    // Compound types
    // In Rust, there are two compound types: tuples and arrays
    // -- tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup; // destructuring (in Python, this would be x, y, z = tup)
    println!("The value of y is: {}", y);

    // -- arrays
    // Like in C or C++, arrays in Rust have a fixed length
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The array a is: {:?}", a);
    println!("The first element of a is: {}", first);
    println!("The second element of a is: {}", second);

    for element in a.iter() {
        println!("The next element is: {}", element);
    }
}