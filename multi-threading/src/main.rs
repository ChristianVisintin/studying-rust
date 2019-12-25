/**
 * Author: Christian Visintin
 * Description: An overview of multi threading in Rust
 */

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    //Use thread spawn with a closure to create a thread
    let my_thread = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //Wait for the thread to end
    println!("Waiting for the thread to finish");
    my_thread.join().expect("Thread paniced");

    //We can move a value to the thread using move
    let my_vector = vec![1, 2, 3, 4, 5];
    let my_thread = thread::spawn(move || {
        for elem in my_vector {
            println!("Spawned thread 2: {}", elem);
        }
    });
    println!("Waiting for the second thread to finish");
    my_thread.join().unwrap();

    //In rust threads, as in GoLand, don't share memory to communicate, but communicate to share memory
    //Let's see how to achieve this
    //A communication channel has a transmitter and a receiver
    //Create a communication channel between threads
    let (tx, rx) = mpsc::channel(); //TX and RX end points are returned
    let my_thread = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //Send a value to rx
    });
    let received = rx.recv().unwrap();
    println!("Got '{}' from thread", received);
    my_thread.join().unwrap();
    //Let's send More values
    let (tx, rx) = mpsc::channel();

    let my_thread = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    my_thread.join().unwrap();

}
