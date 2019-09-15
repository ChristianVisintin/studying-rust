/**
 * Author: Christian Visintin
 * Description: An introduction to the Rust common collections
 */

use std::collections::HashMap;

fn main() {
    //@! Vectors
    let mut v32: Vec<i32> = Vec::new();
    //Add element to vector
    v32.push(5);
    v32.push(6);
    v32.push(7);
    v32.push(8);
    //Read vector elements
    let first_element = &v32[0];
    println!("Vector first element: {}", first_element);
    //But it's also possible to access using options, which is more secure,
    //since the element doesn't exist, it would return None
    let third_element: Option<&i32> = v32.get(2);
    if third_element.is_none() {
        println!("Third element is none");
    } else {
        println!("The third element is {}", third_element.unwrap());
    }
    //When accessing the reference, if the element doesn't exist, the program panics

    //You can't have a reference to the vector and at the same time adding memory
    //Since the previous address is no more valid
    v32.push(9); 
    //println!("Vector first element: {}", first_element); ---> ERROR!!!

    //Iterate over the vector
    for i in &v32 {
        
        println!("{}", i);
    }
    //It is also possible to edit the vector values
    for i in &mut v32 {
        *i += 50;
        println!("{}", i);
    }

    //@! Strings (in detail)

    //Create a string
    let mut _s = String::new();
    //To create a String from a str literal use the to_string() method
    let data = "Initial content";
    let s = data.to_string();
    println!("{}", s);
    //But it's also possible to use String::From
    let mut s = String::from(data);
    println!("{}", s);
    //Rust string supports natively the UTF-8 format
    s.push_str("🐶️");
    println!("{}", s);
    //String can be concatenated with the + operator
    let s2 = String::from(" added content");
    s = s + &s2;
    println!("{}", s);
    //How to access a string's character by its index
    let s3 = String::from("🐶️");
    //let ch0 = s3[0]; ----> ERROR!!!
    //You can't, since the rust strings are encoded in UTF8
    //Let's see the case using the len; we expect 1 as value here
    println!("'{}' length is {}", s3, s3.len());
    //But it's 7... that's because that emoji takes 7 bytes
    //So you can't access the index in the conventional way bdcause of the encoding
    //To understand how to do then, we need to understand that rust sees strings in 3 different ways
    //As bytes, scalar values (chars) and grapeheme clusters (words)
    let sample = "उदाहरण"; //Exaple in hindi
    //If we access using the index, we'll get the letter which needs 3 bytes to be written
    //First letter using slices
    let ch0 = &sample[0..3];
    println!("{} first letter: {}", sample, ch0);
    //If we access 0..2 we'll "get panicked at 'byte index 2 is not a char boundary; it is inside 'द'"
    //So this method is not safe
    //There's a safe method though
    //For chars
    for c in sample.chars() {
        println!("{}", c);
    }
    //For bytes
    for by in sample.bytes() {
        println!("{}", by);
    }
    //To access grapheme clusters there is a crate on crate.io <https://crates.io/crates/unicode-segmentation>

    //@! Hash Maps
    let mut results: HashMap<String, u32> = HashMap::new();
    //Insert values
    results.insert(String::from("Mark"), 8);
    results.insert(String::from("Elizabeth"), 9);
    results.insert(String::from("Alex"), 4);
    //For maps, the ownership 
    let student = String::from("Tyler");
    results.insert(student, 5);
    //Here student string is out of scope

    //Access hash map values
    let elizabeth_result = results.get("Elizabeth"); //Returns Option
    let martha_result = results.get("Martha");

    if elizabeth_result.is_some() {
        println!("Elizabeth got {}", elizabeth_result.unwrap());
    } else {
        println!("There's no Elizabeth in this class");
    }

    if martha_result.is_some() {
        println!("Martha got {}", martha_result.unwrap());
    } else {
        println!("There's no Martha in this class");
    }

    //It's also possible to overwrite a value
    results.insert(String::from("Elizabeth"), 0);
    let elizabeth_result = results.get("Elizabeth");
    if elizabeth_result.is_some() {
        println!("The teacher discovered that Elizabeth cheated the test, her new result is {}", elizabeth_result.unwrap());
    } else {
        println!("There's no Elizabeth in this class");
    }
    //To update a value only there's no one associated, this syntax can be used
    results.entry(String::from("Tyler")).or_insert(2);
    //The result should still be 5
    let martha_result = results.get("Tyler");
    println!("Tyler got {}", martha_result.unwrap());

    //We can also use this example to update the value of a key based on its previous value
    let text = "Hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_ascii_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);

}
