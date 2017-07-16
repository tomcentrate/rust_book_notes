fn main() {
    /// Mutability.
    /// Since variables are immutable by default, you'll need to use
    /// the keyword mut to change it.
    let mut x = 5;
    println!("The value of x is: {}", x);

    // This line would fail if the prior call did not have mut.
    x = 6;
    println!("The value of x is: {}", x);

    /// Shadowing
    /// It is similar, except that it creates a new variable with the same name.
    let y = 10;
    println!("The value of y is: {}", y);

    // Note that we changed the type to a string instead.
    let y = "Ten";

    println!("The value of y is: {}", y);
}