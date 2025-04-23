// i32 is a signed number so only positive no. are allowed
// u32 is an unsigned no. negative postive doesn't matter
// by default a variable is immutable, and if we want to change the value of that variable we have to make this variable immutable.
// Data Types in Rust
// Scalar Types : Represents single value. 4 primry types : Integers, float,  booleans and characters

// Integer Types : 
// length Signed    Unsigned
//  8 bit   i8      u8
//  16
//. 32 
//  64
//  128
//  arch    isize   usize
// Each signed variant can store numbers from -(2 power n - 1) to 2power(n - 1) - 1
// Unsigned variants can store numbers from 0 to 2n - 1

// Floating Point Types : 
// All floating-point types are signed.
// Rust’s floating-point types are f32 and f64. 32 bits and 64 bits
// Default type is f64


// Boolean Types : 
// true and false
// used using bool let f : bool = true;

// Character TYpes : 
// declared using char
// let c = 'z';
// let x : char = 'Z';
// Char literals with single quotes


// Compound Types : 
// group mutiple values into one type.
// Two primitive compound types : tuples and arrays.

// The tuple type : 
// A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
// Immutable as cannont grow and shrink in size
// let tup : (i32, f64, u8) = (500, 6.4, 1);
// can access tuple elements directly by period (.)
// let five = tup.0;
// The tuple without any values has a special name, unit.
// its valueand type are both written() and represent an empty value or an empty return type


// The array Type :
// every element of an array must have the same type, not like tuple.
// arrays in Rust have a fixed length
// let a = [1,2,3,4,5];
// Arrays are useful when you want your data allocated on the stack.
// let months = ["January", "February", "March", "April", "May", "June", "July",
//             "August", "September", "October", "November", "December"];
//
// array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array,
// eg . let a : [i32, 5] = [1,2,3,4,5];
// we can initialize an array with same values, like :
// let samevaluearray : [i32, 5] = [3; 5];
// Accessing Array Elements:
// let first = a[0];


// Functions : 
// functions can have parameters. 
// When a function has parameters, you can provide it with concrete values for those parameters.
// fn main() {
//   print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// Statements do not return values
// fn main() {
//     let x = (let y = 6);
// }
// It will give error as let y = 6 is an statement and it will not return anything.
// in rust you can't do something like :   let x = (let y = 6);
// let x = let y = 6;

// here youdo something like : 
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1 // This is an expression
//     };

//     println!("The value of y is: {y}");
// }

// Expressions evaluate to a value.
// {
//     let x = 3;
//     x + 1
// }
//  That value gets bound to y as part of the let statement. Note that the x + 1 line doesn’t have a semicolon at the end, 
// which is unlike most of the lines you’ve seen so far.
// Expressions do not include ending semicolons
// If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value


// Functions with Return Values
// Functions can return values to the code that calls them
//  You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
// fn five() -> i32 {
//     5
// }
// here are no function calls, macros, or even let statements in the five function—just the number 5 by itself.
// and it is a valid function in Rust.it automatically returns value 5

// Here is another example : 
// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
// If we put semicolon in the end in x + 1; then we'll get the error as it is a statement now.

// If else
// It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error.
// fn main() {
//     let number = 3;
//     if number {
//         println!("number was three");
//     }
// }
// we will get error her eas number is 3 which is not true not false
// Rust will not automatically try to convert non-Boolean types to a Boolean.
// 
// Using if in a let Statement : 
// Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable,
// let condition = true;
// let number = if condition { 5 } else { 6 };
// the values that have the potential to be results from each arm of the if must be the same type
// eg :  let condition = true;
// let number = if condition { 5 } else { "six" };
// here it will give an type error

// Repetition with Loops
// 3 types of loop : loop, for, while
// loop : 
// It will run again and again until you will tell explicitly to stop
// Fortunately, Rust also provides a way to break out of a loop using code. You can place the break keyword within the loop to tell the program when to stop executing the loop
// One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2; uses break to get out of the loop
//         }
//     };

//     println!("The result is {result}");
// }

// for loop : 
// let a = [10, 20, 30, 40, 50];
//  for element in a {
//         println!("the value is: {element}");
//     }
// }

// fn main() {
//     for number in (1..4).rev() { .rev() is reverse function
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }




//                      OWNERSHIP IN RUST                   //
// Onwership rules : 
// 1. Each value in rust has an owner
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
// 
// 
// 
// 
// 
// 

fn c_to_f(temp :f64) -> f64{
    temp * (9.0/5.0) as f64 + 32.0
}

fn lookingforloop(s : &str) {
    for i in s.chars(){
        println!("{}", i);
    }
    //If we want the indexes also we can do like this : .enumerate() adds the iteration no. in loop
    for (i, c) in s.chars().enumerate(){
        println!("Index : {}, char : {}", i, c); 
    }
}

fn main(){
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}");

    let value = s; // S now dont have the ownership of hello it is transferred to the value; s's memory is delocated
    let value2 = value.clone();
    // println!("{s}"); It wont work as the ovnership of s is now passed to value
    println!("{value}");
    println!("{value2}");
    


    let mut i = 32;
    let x = i;

    println!("{i}");
    println!("{x}");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let ans = c_to_f(12.4);
    println!("The temperature in Fahrenheit is {} Fahrenheit", ans);
    println!("{}", c_to_f(123.4));

    //looping in str() string needs str.chars() to do it:
    lookingforloop("The Twelve Days of Christmas");

    // let guess : u32 = "43".parse().expect("Not a number");
    // print!("{guess}");




    // let mut x = 5;
    // println!("The value of x is : {x}");
    // x = 4;
    // println!("The value of new x is : {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("The time in seconds is  : {THREE_HOURS_IN_SECONDS}");

    // let spaces = "    ";
    // let spaces =  spaces.len();
    // print!("Spaces : {spaces}");


    return;
}


fn shadowing(){
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn is_even(num : i64) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}