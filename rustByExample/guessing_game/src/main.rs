// use the standard input output library
use std::io;


// declares a new function called main with no parameters
fn main() {
    // a macro that prints to console
    println!("Guess the number!");
    println!("Enter a guess: ");

    // Creates a mutable variable named guess. 
// Then, assigns a new String, gotten by calling the new() associated function from the String 
    // type. This creates a new, empty String. Associated functions are called with "::".
    let mut guess = String::new();
    

    // This calls the stdin (standard input) function from the io module
    // Alternatively this could be written without importing std::io by writing "std::io::stdin()"
    // The stdin function returns an instance of std::io::Stdin, which handles standard input in
    // terminal. 
    io::stdin()
        // This calls the read_line method on the standard input handle to recieve user input.
        // "&mut guess" is passed to tell the method which variable to store input in.
        // "&" means the argument is a reference, so multiple parts of the code can referece this
        // one bit of data without copying it to memory multiple times. Due to references being
        // immutable, we need to clarify that it is mutable with "mut".
        .read_line(&mut guess)
        // This is used to handle errors. read_line puts what is entered into a string, but also
        // returns Result, which is either 'Ok' or 'Err'. If it is Err, expect goes off and crashes
        // the program, which normally contains some information about the error. 
        .expect("Failed to read line!");

    // guess in brackets {var} passes a variable into the macro.
    // Alternatively, they can be a variable name, and can be passed in after.
    // println!("x = {x} and y + 2 = {}", y + 2);
    // println!("{sigma}", sigma = 4);
    println!("You guessed {guess}");
}
