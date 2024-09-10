RUST:

Why?

- Rust is faster
- Rust is memory safe
- Rust is multi-threaded

Cargo.toml => package.json
main.rs => index.js

CREATE A RUST PROJECT: cargo init

RUN THE MAIN.RS file: cargo run

OPTIMIZED BUILD: cargo build --release

Simple variables in rust:
- Numbers : Integers = let x: i32 = 5 (i32 is 32 bit signed integer) & Float = let z: f32 = 10.01
- Bool : let is_male: bool = true
- String : let greeting: String = String::from("Hello world")

ALL VARIABLES ARE IMMUTABLE BY DEFAULT: use "mut" keyword to convert variables to mutable

CONDITIONALS = if else statements

    
    let is_even: bool = true;
    if is_even {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }


LOOPS:

    
    for i in 0..=10 { // prints 0 to 10
        print!("{} ", i);
    }

Loops are required to iterate over arrays, maps, strings

FUNCTION: always provide a return type

    
    fn function_name() -> return_type {
	    function_body;
    }

MEMORY MANAGEMENT IN RUST:

Background of Memory management:

- Execution happens inside Random Access Memory : Whenever a program is run, it allocates and deallocates memory on the RAM.
- JavaScript uses Garbage Collector for memory management : deallocates memory on the RAM i.e. cannot perform manual memory management
- C allows for manual memory management but can lead to dangling pointers (pointer pointing to blank/invalid data) or memory issues


Rust has ownership model for memory, extremely safe to memory errors. (Not having a garbage collector is part of the reason Rust is fast)

Other key reasons Rust is fast:
- Mutability
- Heap and Memory
- Ownership model
- Borrowing and references
- Lifetimes

1. Mutability : By default, all variables in RUST are immutable

- No need for synchronization when data is accessed concurrently, thread safe - no thread can alter the data
- Allows compiler to optimize code

2. Stack vs Heap : Stack is organized, Heap is unorganized

Stack: Upon execution of a program in RUST, a portion of RAM memory is allocated to the program to store variables. 

- Stack is the portion of RAM which stores the fixed length variables (variables whose size is known at compile time). 
- Stack enables fast allocation and deallocation. 
- Every function pushes a stack frame inside the Stack. 
- The stack frame consists of per variable memory allocation for a function.

    ````
    let my_number: i32 = 3; // a stack frame containing my_number is pushed to the Stack
    other_function(other_number: my_number); // another stack frame containing other_number is pushed to the Stack
Heap: Heap is used to store strings or vectors whose size may change during runtime. 

- The program consisting of variables of varying sizes pushes a stack frame on the Stack, the stack frame consists of a pointer (address) to a contiguous memory (heap) to store varying length variables.


3. Ownership:  Memory is managed through a system of ownership (a set of rules). Compiler checks the rules, if any rule is violated, program won't compile.

**Heap management requires ownership**

- Every memory allocated on the heap has an owner stored on the Stack. 
- The owner on the Stack points to the memory allocated on the heap.
- The memory cannot live without the owner.
- The memory can only have 1 owner
    ```
    let s1: String = String::from("hello");
    let s2 = s1; // s2 is the new owner of "hello", s1 is cleaned up
    println!("{}", s1);

    let my_string: String = String::from("hello")
    takes_ownership(some_string: my_string) // some_string is the owner of "hello" and my_string is deleted
COPYING ON HEAP IS EXPENSIVE!

BORROWING AND REFERENCES:

- Multiple borrowers are allowed.

    ```
    let s1: String = String::from("Hello");
    let s2: &String = &s1; // use & to pass a reference to s2
    let s3: &String = &s1;
    let s4: &String = &s1;
    let s5: &String = &s1;
    let s6: &String = &s1;
 - s2 (points to s1) => s1 (points to memory on the heap) => heap

Mutable references:

- the original variable must be mutable
- only a single mutable reference is allowed.

    ```
    let mut s1: String = String::from("Hello"); // original variable is mutable 
    update_str(&mut s1); // use &mut to borrow mutable variables
    // CANNOT use references of s1 anymore
STRUCTS:

- structs bind variables together

    ```
    struct Rect {
        width: u32,
        height: u32,
    }
    impl Rect {
            fn area(&self) -> u32 {
                return self.width * self.height;
            }
            fn perimeter(&self) -> u32 {
                print!("Perimeter");
                return 2 * (self.width * self.height);
            }
    }

ENUMS:

- define variants instead of strings

    ```
    enum Direction {
    	North,
    	East,
    	South,
	    West
    }
PATTERN MATCHING ENUMS:

- use "match" keyword

    ```
    fn move_around(direction: Direction) {
    // implement logic to move a character
    match direction {
        Direction::East => print!("East"),
        Direction::North => print!("North"),
        Direction::South => print!("South"),
        Direction::West => print!("West"),
    };
}

ERROR HANDLING: 

- JavaScript has try catch blocks to handle runtime error
- Result enum handles error in Rust

    ```
   enum Result<T, E> {
	    Ok(T),
	    Err(E),
    }
    let res = fs::read_to_string("example.txt");
    match res {
        Ok(..) => print!("Read!"),
        Err(..) => print!("Error!"),
    };
OPTION ENUMS:

- to remove null types in rust

    ```
    pub enum Option<T> {
	    None,
	    Some(T)
    }
DEPENDENCIES: use `cargo add [package_name]` to install crates (packages)

https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-1
