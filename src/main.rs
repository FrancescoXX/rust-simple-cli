// A simple CLI in rust
// This CLI will get a string in input
// and return the string reversed

//import the env module to get the command line arguments
use std::env;

fn main() {
    // collect the arguments in a vector
    let args: Vec<String> = env::args().collect(); 

    // check if the user has entered a string
    // by checking the number of arg
    
    // args[0] is the name of the program
    if args.len() < 2 {
        println!("Please enter a string");
        return;
    }

    //store the string in a variable
    let input = args[1].clone();   

    // reverse the string
    let reversed = input.chars().rev().collect::<String>();

    // print the reversed string
    println!("Reversed string: {}", reversed);
}
