// struct User {
//     name: String,
//     age: u32,
//     active: bool,
// }

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }
//     fn perimeter(&self) -> u32 {
//         print!("Perimeter");
//         return 2 * (self.width * self.height);
//     }
// }

// enum Direction {
//     North,
//     South,
//     East,
//     West,
// }

// fn move_around(direction: Direction) {
//     // implement logic to move a character
//     match direction {
//         Direction::East => print!("East"),
//         Direction::North => print!("North"),
//         Direction::South => print!("South"),
//         Direction::West => print!("West"),
//     };
// }

// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     // if let Shape::Circle(radius) = shape { print!("{}", radius)}
//     let ans = match shape {
//         Shape::Circle(radius) => 3.14 * radius * radius,
//         Shape::Square(side) => side * side,
//         Shape::Rectangle(width, height) => width * height,
//     };

//     return ans;
// }

// use std::fs;

use chrono::{Local, Utc};

fn main() {
    let now = Utc::now();
    println!("Current date and time in UTC : {}", now);

    let local = Local::now();
    println!("Current date and time in Local : {}", local);

    // Error Handling
    // let res = fs::read_to_string("example.txt");
    // match res {
    //     Ok(..) => print!("Read!"),
    //     Err(..) => print!("Error!"),
    // };

    // Pattern match
    // let circle = Shape::Circle(5.0);
    // let square = Shape::Square(5.0);
    // let rectangle = Shape::Rectangle(5.0, 10.0);

    // print!("{}", calculate_area(circle));

    // let direction = Direction::West;
    // move_around(direction);

    // Enums
    // move_around(Direction::East)

    // Struct implementation
    // let rect = Rect {
    //     width: 30,
    //     height: 60,
    // };

    // print!("The area of the rectangle is {}", rect.area());
    // print!("The perimeter of the rectangle is {}", rect.perimeter());

    // Structs
    // let user = User {
    //     name: String::from("Alice"),
    //     age: 30,
    //     active: true,
    // };
    // print!("{} is {} years old", user.name, user.age);

    // Numbers: Integers and Floats
    // let x: i32 = -5;
    // let z: f32 = 1000.45;

    // print!("x is {}", x);
    // print!("z is {}", z);

    // Booleans
    // let is_male: bool = false;
    // println!("You are {}", is_male);

    // String
    // let greeting: String = String::from("hello world");
    // println!("{}", greeting);
    // let char1 = greeting.chars().nth(4);
    // print!("char1: {}", char1.unwrap());

    // CONDITIONALS = if else statements
    // let is_even: bool = true;
    // if is_even {
    //     println!("The number is even");
    // } else {
    //     println!("The number is odd");
    // }

    // LOOPS
    // for i in 0..=10 {
    //     print!("{} ", i);
    // }

    // let sentence: String = String::from("my name is jack");
    // let first_word = get_first_word(sentence);
    // print!("First word is {}", first_word);

    // Mutable

    // let mut x = 5;
    // println!("{}", x);
    // x = 2;
    // println!("{}", x);

    // STACK VS HEAP

    // fn_stack();
    // fn_heap();
    // fn_update();

    // OWNERSHIP
    // let s1: String = String::from("hello");
    // let s2 = s1;
    // println!("{}", s1);
    // println!("{}", "Hello")

    // BORROWING
    // let mut s1 = String::from("Hello"); // original variable
    // let s2 = &mut s1; // borrowed mutable reference to original variable

    // // println!("{}", s1);
    // println!("{}", s2);
    // print!("{}", update_word(s2));
}

// fn update_word(word: &mut String) -> &mut String {
//     word.push_str(" nerd");
//     return word;
// }

// fn fn_stack() {
//     let a: i32 = 5;
//     let b: i32 = 10;
//     let c = a + b;
//     println!("a: {}, b: {}, c: {}", a, b, c);
// }

// fn fn_heap() {
//     let s = String::from("hello");
//     println!("{}", s);
// }

// fn fn_update() {
//     let mut s: String = String::from("Initial String");
//     println!("Before update: {}", s);

//     println!(
//         "Capacity {}, Length {}, Pointer {:p}",
//         s.capacity(),
//         s.len(),
//         s.as_ptr()
//     );

//     s.push_str(" and some extra text");
//     println!("After update {}", s);
//     println!(
//         "Capacity {}, Length {}, Pointer {:p}",
//         s.capacity(),
//         s.len(),
//         s.as_ptr()
//     );
// }

// fn get_first_word(sentence: String) -> String {
//     let mut ans: String = String::from("");

//     // for char in sentence.chars() {
//     //     ans.push_str(char.to_string().as_str());
//     //     if char == ' ' {
//     //         break;
//     //     }
//     // }

//     // Another method of running for loops

//     for (i, c) in sentence.chars().enumerate() {
//         ans.push_str(c.to_string().as_str());
//         ans.push_str(i.to_string().as_str());
//     }

//     return ans;
// }
