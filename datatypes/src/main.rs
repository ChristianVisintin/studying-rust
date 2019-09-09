/**
 * Author: Christian Visintin
 * Description: An introduction to the Rust datatypes and variables
 */

fn main() {
    //A variable in rust is declared with the let keyword
    //By default all variables are immutable
    let x = 5;
    println!("The value of x is {}", x);
    //x = x + 1 => ERROR!!!
    //It is possible, though to create mutable variables.
    //By default Rust doesn't encourage the use of mutable variables, if not strictly necessary
    //Indeed, you'll be warned by the rust compiler about mutable variables which value doesn't change
    let mut y = 10;
    println!("Mutable variable 'y': {}", y);
    y = 55;
    println!("y's value has now changed :D {}", y);
    //Rust is strong typed, so the type can't be changed to a variable
    //y = 'A'; => ERROR!!!
    //The type in the previous cases were provided implicitly by the value assigned, but it is possible to specify it by the sintax
    //let variable: type = value;
    let z: u8 = 224;
    println!("z is an unsigned of size 1 byte: {}", z);
    //@!Shadowing: the shadowing is a rust feature which allows to redeclare a variable with the same name of a previously declared variable
    //When a variable is shadowed, the previously declared variable goes out of scope and gets freed.
    let x = 7;
    println!("The value of x is {}", x);
    //It is also possible, while shadowing a variable, to refer to its previous value
    let x = x + 10;
    println!("The value of x is {}", x);
    
    //@! Rust scalar types
    let _char: char = 'A'; //Char
    let _i8: i8 = 127; //Integer 1 byte
    let _u8: u8 = 255; //Unsigned 1 byte
    let _i16: i16 = -32000; //Integer 2 bytes
    let _u16: u16 = 65535; //Unsigned 2 bytes
    let _i32: i32 = -3000000; //Integer 4 bytes
    let _u32: u32 = 6000000; //Unsigned 4 bytes
    let _i64: i64 = -11222233; //Integer 8 bytes
    let _u64: u64 = 1111222334; //Unsigned 8 bytes
    let _arch: isize = 1223344; //Or usize: i64/u64 or i32/u32 based on system architecture
    //It is also possible to use different number representation to set the numeric value
    let _u8: u8 = 0xf0; //Hex
    let _u8: u8 = 0b1111_0000; //Binary
    let _u8: u8 = 0o360; //Octal
    let _u8: u8 = b'A'; //ASCII value
    //It is also possible to split number for big values
    let _u32 = 97_305; //97,305
    //NOTE: If wou're wondering why I'm declaring all the variables with the undescore in fron, the reason is that
    //Using the underscore in fron of the name, prevents rustc to raise warnings of unused variable!
    //Floating points
    let _f: f32 = 0.32;
    //Boolean
    let _b: bool = true;
    //@! Rust Compound types
    let tup: (i32, f64, char) = (500, 6.4, 'A'); //Tuple
    //NOTE: for tuples is possible to combine more types - as for other languages such as Python - the types in the tuple aren't required
    let _tup2 = (305, 789.67, 'C');
    //It's possible to unpack a tuple with the following syntax
    let (a, b, c) = tup;
    println!("Tuple values ({}, {}, {})", a, b, c);
    //But it's also possible to access to their element with the dot notation
    println!("Tuple values ({}, {}, {})", tup.0, tup.1, tup.2);
    //Array
    let array = [1, 2, 3, 4, 5];
    println!("Array[0] {}", array[0]);  
}

