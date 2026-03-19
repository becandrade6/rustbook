use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // prints a new line in terminal
    println!("Guess the number!");

    // thread_rng func that gives the particular random number generator were going to use
    // one that is local to the current thread of execution and is seeded by the operating system
    // then, we call the gen_range method on the random number generator
    // this method takes a range expression as an argument and generates a random number in the range
    // the kind of range expression we are using here takes the form start..=end and is
    // inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number
    // between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop creates an infinite loop
    loop {
        println!("Please input your guess.");

        // let statement to create a variable
        // all variables are immutable by default
        // adding mut makes the variable mutable (the value can change)
        // String::new() returns a new instance of a String
        // So here we create a mutable variable that is currently
        // bound to a new, empty instance of a String.
        let mut guess = String::new();

        // Call the stdin function from the io module
        // which allow us to handle user input
        // the .read_line stores the input in the variable passed
        // the read_line returns a Result type that is an enum
        // this enum can be Ok or Err
        // The Result type has a method .expect
        // if the Result is an Err, the .expect will cause the
        // program to crash and display the message passed as argument
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // here we use match so the program does not crash if
        // what was inputed is not a number
        // now we handle the Err returned by parse()
        // the '_' is a catch-all value, in this example we are saying
        // that we want to match all Err values, no matter what info
        // they have inside them
        // this part here is very powerful
        // we are saying that we will store something in guess and is an u32
        // then we call a match statement to evaluate the result of the
        // guess.trim().parse()
        // if it is an Ok with a num then we return the num
        // if not and is an Err with catch-all we call continue so the
        // program goes to another iteration of the loop, very interesting
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
