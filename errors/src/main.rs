/**
 * Author: Christian Visintin
 * Description: An overview of error handling in Rust
 */

use std::fs::File;

fn main() {
    //In rust there two kinds of errors:
    //Recoverable errors (such as file not found)
    //Unrecoverable errors (Which will make the program panic)

    //When there's nothing to do to recover from an error
    //The panic macro will be called or can be called
    //This macro will print the error, then unwind and clean the stack and then quit

    //panic!("Abort!"); //Uncomment this line to see what happnes with panic

    //To handle recoverable errors, there is a standard enum called Result

    //
    // enum Result<T, E> {
    // Ok(T),
    // Err(E)
    // }

    //Let's try opening a file
    //This will return a Result anum
    let f = File::open("/tmp/hello.txt");
    //If file doesn't exist, the program will panic
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Could not open file /tmp/hello.txt {:?}", error);
        }
    };

}
