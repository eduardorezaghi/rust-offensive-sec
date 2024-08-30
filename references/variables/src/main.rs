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
}
