/**
 * Author: Christian Visintin
 * Description: An introduction to the Rust ownership paradigm
 */

fn main() {
    //@! Ownership and memory management
    //In rust the memory is not managed by a garbage collector nor manually.
    //In rust a variable is freed when it gets out of its scope.
    //Let's declare a mutable String variable
    //NOTE: a string is a standard type which, as in C++, uses the heap to allocate the string.
    let mut s = String::from("Hello");
    //Let's push to string world!
    s.push_str(", world!");
    println!("{}", s);
    //But is actually also possible to free a variable manually, you know, to optimize your program
    drop(s); //Free memory manually
    //Let's redeclare s using shadowing
    let mut s = String::from("Potatoes");
    s = s.to_uppercase();
    println!("{}", s);
    //@!Move
    //What happens when we assign s to a string variable s2?
    //Will s2 be a copy or a reference to s? Let's see what happens
    let s2 = s;
    //Let's Change s' value
    //s = s.to_lowercase(); ----> ERROR!!! Wait what?
    //None of them... the memory got moved to s2! What does it mean?
    //It means the ownership of s' data has been passed to s2 and s is no more usable.
    println!("{}", s2);
    //How to create a copy of s then? Using clone
    let mut s = s2.clone();
    s = s.to_lowercase();
    println!("S: {}; S2: {}", s, s2);
    //The ownership is taken by the scope where the variable is or goes, so when we pass s2 to a function, what happens then?
    take_ownership(s2); //The ownership of s2 goes to the function; move is called
    //println!("{}", s2); ERROR --> s2 has been moved and its now out of scope, so it doesn't exist anymore!
    //How can I pass a string to a variable and then re-use it here? Using references!!!
    do_not_take_ownership(&s);
    //You can still call s
    println!("s is still here: {}", s);
    //And is it possible to mutate it? Yes, of course!
    do_not_take_ownership_and_mutate(&mut s);
    //You can still call s
    println!("s is still here, but changed: {}", s);
    //@! Create a reference to s
    let s_ptr = &s;
    println!("String ptr: {}", s_ptr);
    //@! We can't though create a mutable ptr to a non-mutable
    //let mut s2_ptr = &mut s2;
    //But we can create a non-mutable reference of a mutable string
    //Get first word of a string as seen at row 47
    //We cannot have more than one mutable string reference at the time
    let mut_s_ptr1 = &mut s;
    //let mut _mut_s_ptr2 = &mut s; ---> ERROR, we cannot borrow a mutable reference more than once
    //let _s_ptr = &s; ----> ERROR, we cannot have at the same time a mutable reference and a immutable reference
    mut_s_ptr1.push_str(" with tomato sauce");
    println!("mut s reference: {}", mut_s_ptr1);
    //@! Slices
    //Image we want to to get the first word of our string
    let mut s2 = s.clone();
    let first_word = get_first_word(&s2);
    println!("First word: {}", first_word);
    //What happens though, if we now clear s?
    s2.clear();
    println!("s2 after clear: {}; first word of s2: {}", s2, first_word);
    //S2 is empty, but first word is still potatoes. If you came from other languages, you'll probably say there's nothing strange
    //But in Rust there's an important component which can keep coherence between the first word and the string.
    //This component is called slices.
    //Slices are a reference to a part of a string or to a part of a compound data
    //If we now cleared the string, the first_word would become not coherent with the string
    let first_word = get_first_word_slices(&s);
    println!("First word: {}", first_word);
    let first_6_letters = &s[0..6];
    println!("First 6 letters: '{}'", first_6_letters);
    //s.clear(); NOTE: can't do this, unless first_5_letters is dropped
    //println!("First 5 letters after clear {}", first_5_letters);
    //@! Slices exists also for array
    let array = [1, 2, 3, 4, 5];
    let array_slice = &array[1..3];
    println!("Arr.0: {}; Slice.0: {}", array[0], array_slice[0]);
} //S is automatically dropped here since go out of scope

fn take_ownership(a_string: String) {
    println!("I own {} now v", a_string);
}

fn do_not_take_ownership(a_string: &String) {
    println!("Ref value: {}", a_string);
    //But we can't modify its value!!!
    //a_string.push_str(" and beans");
}

fn do_not_take_ownership_and_mutate(a_string: &mut String) {
    a_string.push_str(" and beans");
}

fn get_first_word(s: &String) -> String {
    let vec: Vec<&str> = s.split(' ').collect();
    String::from(vec[0])
}

fn get_first_word_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
