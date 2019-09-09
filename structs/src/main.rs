/**
 * Author: Christian Visintin
 * Description: An introduction to the Rust datatypes and variables
 */

//In rust the struct concept is pretty much the same of the other languages...

//@! Define rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

//But in Rust they can have methods
//@! Implementing struct methods in Rust
impl Rectangle {
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }
    fn calc_perimeter(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
    }
}
//To call a Rectangle method just use your_rectangle.method_name();

//Enums in Rust are pretty much more interesting than in other languages, let's see why
//@! Define enum
//This will probably sound familiar to you
enum IpAddrVersion {
    V4,
    V6,
}

//But now, imagine this case, we have a struct which represents an ip address
struct IpAddr {
    address: String,
    version: IpAddrVersion,
}

//In Rust we can represent the same struct using enums though...

enum IpAddress {
    V4(u8, u8, u8, u8), //To the enum value V4 we associated a tuple of 4 bytes
    V6(String),         //While to V6 we associated a string
}

//We can also use enums as "options", as in this case
//Imagine we have a protocol manager, that's how we can handle the commands using enums

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
    UselessMessage,
    MuchUselessMessage,
}

//We can handle the message implementing a method for it
impl Message {
    fn call(&self) {
        //How to discriminate the behaviour based on the value of the enum?
        //Using match!
        match self {
            Message::Quit => {
                println!("Command: Quit!");
            }
            Message::Move { x, y } => {
                println!("Command: Move to pos {},{}", x, y);
            }
            Message::Write(s) => {
                println!("Command: Write {}", s);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Command: Color changed to {}, {}, {}", r, g, b);
            }
            _ => {
                //The key '_' indicates 'for all the others'; it's like the default in C/C++ switch
                println!("This command has no purpose");
            } //Placeholder in case not al cases have to be implemented
        }
    }
}

//We can now implement an output for our ip address enum

impl IpAddrVersion {
    fn repr(&self) -> String {
        match self {
            IpAddrVersion::V4 => String::from("4"),
            IpAddrVersion::V6 => String::from("6"),
        }
    }
}

impl IpAddress {
    fn print(&self) {
        match self {
            IpAddress::V4(a, b, c, d) => {
                println!("{}.{}.{}.{}", a, b, c, d);
            }
            IpAddress::V6(addr) => {
                println!("{}", addr);
            }
        }
    }
}

fn main() {
    //@! Structs
    let myrect = Rectangle {
        width: 48,
        height: 32,
    };
    println!("Rect width: {}; height: {}", myrect.width, myrect.height);
    println!(
        "Rect area: {}; perimeter: {}",
        myrect.calc_area(),
        myrect.calc_perimeter()
    );
    //@! Enums
    let home = IpAddr {
        version: IpAddrVersion::V4,
        address: String::from("172.0.0.1"),
    };
    let loopback = IpAddr {
        version: IpAddrVersion::V6,
        address: String::from("::1"),
    };
    let localhost = IpAddress::V4(172, 0, 0, 1);
    let gateway = IpAddress::V4(192, 168, 0, 1);
    let loopback_ipv6 = IpAddress::V6(String::from("::1"));
    println!("Home: {}/{}", home.address, home.version.repr());
    println!("Loopback: {}/{}", loopback.address, loopback.version.repr());
    //print IpAddress enums
    localhost.print();
    gateway.print();
    loopback_ipv6.print();

    let command = Message::ChangeColor(255, 255, 255);
    let command2 = Message::Move { x: 32, y: -24 };
    let command3 = Message::Write(String::from("Ma che ou"));
    let command4 = Message::Quit;
    let command5 = Message::UselessMessage;
    let command6 = Message::MuchUselessMessage;
    command.call();
    command2.call();
    command3.call();
    command4.call();
    command5.call();
    command6.call();

    //@! Option enum
    //Rust provides another important enum features
    //In order to prevent unhandled Null values as happens in theother languages
    //In Rust the None (null) value can exists only for a particular (and standard)
    //Enum called Option. The option enum has two members: Some, which can be assigned with any value
    //Any None, which is indeed None
    let some_number = Option::Some(5);
    let some_string = Some("text");
    let null_value: Option<i32> = None;
    println!(
        "Some number {:?}, some string {:?}; null value {:?}",
        some_number, some_string, null_value
    );
    //You can't sum some_number to a literal i32; it is required to convert some number to a i32 first
    //This prevents null values!
    let that_number = some_number.unwrap();
    let sum_number = 8;
    println!("Sum between 8 and Some5: {}", that_number + sum_number);
}
