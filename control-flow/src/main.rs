/**
 * Author: Christian Visintin
 * Description: An introduction to the Rust control flows types
 */

use std::io;

fn main() {
    let array = [1, 2, 3, 4, 5];
    println!("Array[0] {}", array[0]);
    //Define and use a function
    println!("Pow2 of {} = {}", array[2], pow2(array[2]));
    //@! For loop
    for element in array.iter() {
        println!("Value is {}", element);
    }
    let mut i = 0;
    //@! Loop
    loop {
        //This cycle loops infinetely, until a break statement is reached
        if i == 10 {
            break;
        }
        println!("looping, i: {}", i);
        i += 1;
    }
    //@! Range
    for number in 1..4 { //Range
        println!("{}", number);
    }

    for number in (1..4).rev() { //Countdown
        println!("{}", number);
    }

    //@! CLI Input
    println!("Type something: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("you typed: {}", input);
}

//To define a function we specify in the brackets the argument type, then with the symbol '->' its return type
fn pow2(arg1: i32) -> i32 {
    //No return statement? Yep, and not event a semicolon. The return type of a function is expressed by
    //An expression, while the semicolon is put at the end of a statement.
    //The return keyword in rust exists, but should be used only when a function returns prematurely
    arg1 * arg1
}

