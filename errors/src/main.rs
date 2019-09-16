/**
 * Author: Christian Visintin
 * Description: An overview of error handling in Rust
 */

use std::io;
use std::io::Read;
use std::fs::File;

fn read_from_file(file: String) -> Result<String, io::Error> {
    let mut f = File::open(file)?; //If an error is raised, it returns with an error result
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


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
    {
        let f = File::open("/tmp/hello.txt");
        //If file doesn't exist, the program will panic
        let _f = match f {
            Ok(file) => file,
            //It is possible to match the error
            Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
                match File::create("/tmp/hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("Could not create file /tmp/hello.txt: {:?}", e);
                    }
                }
            }
            Err(error) => {
                panic!(
                    "There was a problem opening file /tmp/hello.txt {:?}",
                    error
                );
            }
        };
    }
    //File is closed here

    //@! Unwrap and expect
    {
        let _f = File::open("/tmp/hello2.txt").unwrap();
        //If file doesn't exist with unwrap, panic will be automatically called
        //Another method is using expect, which is the same, but allows to
        //Specify the error message
        let _f = File::open("/tmp/hello2.txt").expect("Failed to open /tmp/hello2.txt");
    }

    //@! Error propagation
    let res = read_from_file(String::from("Cargo.toml"));
    match res {
        Ok(content) => {
            println!("File read: {}", content);
        },
        Err(error) => {
            println!("Error while reading file {:?}", error);
        }
    }

}
