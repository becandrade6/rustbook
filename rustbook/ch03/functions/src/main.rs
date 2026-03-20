fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    // Here we create a block that evalutes to 4 (an expression, a new scope block)
    // That value gets bound to y as part of the let statement
    // Note the x + 1 without a semicolon ending
    // It is an expression, expressions do not include ending semicolons
    // So, it returns a value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");
    println!("The value of x plus one is: {}", plus_one(x));

}

// Example of function definition
fn another_function(x: i32) {
    println!("Running in function. The value of x is: {x}");
}

// Define function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Example of function with a returning value (expression) at the end
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
