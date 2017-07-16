fn main() {

    /// Integers
    /// signed(i) or unsigned(u)
    /// comes in 8/16/32/64/size
    /// Defaults to i32
    /// Use isize or usize when indexing collections.
    let x: i32 = 2;

    /// Floats
    /// f32 - Floats
    /// f64 - Doubles
    let y: f32 = 3.02; // f32

    // Mathematical operations.
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let modulo = 43 % 5;

    /// Tuples.
    /// grouping stuff together.
    let tuple: (i32, f32, i32, f32, i32) = (sum,
                                            difference,
                                            product,
                                            quotient,
                                            modulo);

    /// Access tuple elements
    /// Use the . followed by the element number.
    println!("Your tuple is: {:?}", tuple);
    println!("Access the second element: {:?}", tuple.1);
    println!("Access the third element: {:?}", tuple.2);

    let tuple: (i32, f32) = (x,y);
    println!("Your tuple is: {:?}", tuple);

    // Destructuring.
    // Pattern matching to reassign elements
    let (a, b) = tuple;

    println!("Destructured x={} should equal a={}", x,a);
    println!("Destructured y={} should equal b={}", y,b);

    /// Arrays
    /// Standard C Array, fixed size, and allocated on the stack.
    let array = [1,2,3,4,5];

    println!("my array is {:?}", array);

    println!("The second element is {}", array[1]);
}
