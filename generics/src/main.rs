/**
 * Author: Christian Visintin
 * Description: An overview of generics in Rust
 */

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//Generics in structs
struct Point<T> {
    x: T,
    y: T
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

fn main() {
    let point_int = Point {x: 5, y: 10};
    let point_float = Point {x: 5.4, y: 13.2};

    println!("Point int ({};{})", point_int.x, point_int.y);
    println!("Point float ({};{})", point_float.x, point_float.y);

    let number_list = vec![34, 50, 25, 100, 64];
    println!("The largest in number_list is {}", largest(&number_list));
    let char_list = vec!['y', 'A', 'b', 'm'];
    println!("The largest in char_list is {}", largest(&char_list));

    
}
