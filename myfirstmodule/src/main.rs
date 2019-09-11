/**
 * Author: Christian Visintin
 * Description: This example shows how to use myfirstmodule
 */

//Let's call the crate myfirstmodule in the project
extern crate myfirstmodule;

//We can explicitly say we want to use only a part of the module with the use keyword
use myfirstmodule::utils::sysutils;

//We can also use the use keyword to import all the declarations into the namespace

enum TrafficLight {
    Red,
    Yellow,
    Green
}

use TrafficLight::*;

fn main() {
    let my_string = String::from("foobar");
    let needle = String::from("bar");
    println!("Does {} ends with {}? {}", my_string, needle, myfirstmodule::utils::strutils::ends_with(&my_string, &needle));
    let _green = Green; //Here the usage of the glob
    let _red = Red;
    let _yellow = Yellow;
    sysutils::reboot();
}
