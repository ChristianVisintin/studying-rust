/**
 * Author: Christian Visintin
 * Description: An overview of generics in Rust
 */

//We had as original function this
//fn largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        //@! Trait bounds
//        //To solve this issue, we need to implement a trait bound for
//        //std::cmd::PartialOrd
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}
//But it didn't compile, so we needed to use trait bounds, which implements
//The traits of PartialOrd and Copy to allows the use of generics for getting
//The largest value

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        //@! Trait bounds
        //To solve this issue, we need to implement a trait bound for
        //std::cmd::PartialOrd
        if item > largest {
            largest = item;
        }
    }
    largest
}

//Generics in structs
struct Point<T> {
    x: T,
    y: T,
}

//Implement generics in methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

//This method will be available only when Point is f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub trait SentenceComposer {
    fn compose(&self) -> String {
        //This is a default behaviour
        String::from("Don't know how to say that")
    }
    //We don't have to implement the default behaviour
    //fn compose(&self) -> String;
}

pub struct EnglishSentence {
    pub subject: String,
    pub verb: String,
    pub article: String,
    pub adjective: String,
    pub object: String,
}

impl SentenceComposer for EnglishSentence {
    fn compose(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.subject, self.verb, self.article, self.adjective, self.object
        )
    }
}

pub struct ItalianSentence {
    pub subject: String,
    pub verb: String,
    pub article: String,
    pub adjective: String,
    pub object: String,
}

impl SentenceComposer for ItalianSentence {
    fn compose(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.subject, self.verb, self.article, self.object, self.adjective
        )
    }
}

//And implement it just using empty brackets

pub struct MartianSentence {
    pub words: String,
}

impl SentenceComposer for MartianSentence {}

//@! Lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //Lifetime annotation &'a
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//Also structs can hold references, but they need a lifetime annotation
struct ExceptionHolder<'a> {
    ptr: &'a str
}

fn main() {
    //@! Generics
    let point_int = Point { x: 5, y: 10 };
    let point_float = Point { x: 5.4, y: 13.2 };

    println!("Point int ({};{})", point_int.x(), point_int.y());
    println!("Point float ({};{})", point_float.x(), point_float.y());
    println!("Point float: distance from origin: {}", point_float.distance_from_origin());

    let number_list = vec![34, 50, 25, 100, 64];
    println!("The largest in number_list is {}", largest(&number_list));
    let char_list = vec!['y', 'A', 'b', 'm'];
    println!("The largest in char_list is {}", largest(&char_list));

    //@! Traits (or in other languages interfaces)
    let italian_sentence = ItalianSentence {
        subject: String::from("io"),
        verb: String::from("ho"),
        article: String::from("un"),
        adjective: String::from("verde"),
        object: String::from("pappagallo"),
    };
    println!("{}", italian_sentence.compose());
    let english_sentence = EnglishSentence {
        subject: String::from("I"),
        verb: String::from("have got"),
        article: String::from("a"),
        adjective: String::from("green"),
        object: String::from("parrot"),
    };
    println!("{}", english_sentence.compose());

    let martian_sentence = MartianSentence {
        words: String::from("§§§§§§§§§§§§"),
    };
    println!("{}", martian_sentence.compose());

    //@! Lifetime

    //Let's use an example of the longest string
    let str1 = String::from("yellow");
    let str2 = "red";

    let result = longest(str1.as_str(), str2);
    println!("The longest string betwen {} and {} is {}", str1, str2, result);

    let string = String::from("A.B.C");
    let first_token = string.split('.').next().expect("Could not find '.'");
    let ex = ExceptionHolder {
        ptr: first_token
    };
    println!("ExceptionHolder.ptr {}", ex.ptr);

    //It's possible to specify a static lifetime, which indicates the reference will
    //live for the entire duration of the execution
    let s: &'static str = "I have a static lifetime";
    println!("{}", s);
}
