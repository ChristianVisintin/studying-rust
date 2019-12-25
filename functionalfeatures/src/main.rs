/**
 * Author: Christian Visintin
 * Description: A program which shows the functional features in Rust
 */

use std::thread;
use std::time::Duration;

extern crate rand;
use rand::Rng;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    //NOTE: A closure is defined using |param1,...,paramn| { function body };
    //Store the function into a variable; Using the cacher will allow to call the function only if the value is not set
    let mut expensive_closure = Cacher::new(|intensity| {
        println!("This will take a long time...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct NumberWorker {
    number: i32,
    worker_ptr: fn(&NumberWorker) -> i32
}

impl NumberWorker {
    fn worker(&self) -> i32 {
        return (self.worker_ptr)(self)
    }
}

fn main() {
    //@! Closures
    let simulated_user_specified_value = 30;
    let mut rng = rand::thread_rng();
    let simulated_random_number = rng.gen_range(1, 4);
    println!("Random value: {}", simulated_random_number);
    generate_workout(simulated_user_specified_value, simulated_random_number);
    //Capturing the enviornment with closures
    let x = 4;
    let equal_to_x = |z| { z == x };
    let y = 4;
    assert!(equal_to_x(y));
    //Force a closures to use fn
    let equal_to_x = move |z| z == x;
    assert!(equal_to_x(y));
    //~~You can't use x anymore~~ WARN: has this changed in newest version of rust?
    println!("X: {}", x);
    //@! Iterators
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter { //NOTE: you could actually do in v1...
        println!("Vec {}", val);
    }
    //NOTE: All iterators implement the trait 'Iterator'
    //pub trait Iterator {
    //    type Item;
    //
    //    fn next(&mut self) -> Option<Self::Item>;
    //
    //    // methods with default implementations elided
    //}
    //Closures that capture their environment: See Show struct
    let shoes = vec![
        Shoe { size: 43, style: String::from("sneaker") },
        Shoe { size: 41, style: String::from("sandal") },
        Shoe { size: 42, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 43);
    for shoe_for_me in in_my_size {
        println!("A shoe for me: {} (Size: {})!", shoe_for_me.style, shoe_for_me.size);
    }

    //Know that we've learned about closures, we can introduce function pointers
    //@!Function pointers
    //Let's use the struct NumWorker
    //We can now create a Number Worker
    let my_first_number_worker = NumberWorker {
        number: 64,
        worker_ptr: |my_first_number_worker| { return my_first_number_worker.number * 2 }
    };
    println!("My worked value: {}", my_first_number_worker.worker());
}
