fn main() {

    // Example of mutability
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    // Example of shadowing
    let x = 5;
    // shadowing, now x = 5+1
    let x = x + 1;

    {
        // inner shadowing: x = 6*2
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // inner shadowing ends, value returns x = 6
    println!("The value of x is: {x}");
}
