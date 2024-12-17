#![allow(unused, dead_code)]

use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne {}

impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!")
    }
}

#[derive(Debug)] // Do the same thing with ErrorTwo
struct ErrorTwo;

impl Error for ErrorTwo {}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

// Make a function that just returns a String or an error
fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me.".to_string()),
    }
}

// fn returns_errors_1(input: u8) -> Result<String, impl Error> {
//     // With Box<dyn Error> you can return anything that has the Error trait
//     match input {
//         0 => Err(ErrorOne),
//         1 => Err(ErrorTwo), //impl on a type adds functionality to one particular type!!!
//         _ => Ok("Looks fine to me.".to_string()),
//     }
// }

fn main() {
    let vec_of_u8s = vec![0_u8, 1, 80];

    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }
}

// use std::mem::size_of;
//
// trait JustATrait {
//     fn print_a_trait(&self) {
//         println!("I am just a trait!")
//     }
// }
//
// enum EnumOfNumbers {
//     I8(i8),
//     AnotherI8(i8),
//     OneMoreI8(i8),
// }
// impl JustATrait for EnumOfNumbers {}
//
// struct StructOfNumbers {
//     an_i8: i8,
//     another_i8: i8,
//     one_more_i8: i8,
// }
// impl JustATrait for StructOfNumbers {}
//
// enum EnumOfOtherTypes {
//     I8(i8),
//     AnotherI8(i8),
//     Collection(Vec<String>),
// }
// impl JustATrait for EnumOfOtherTypes {}
//
// struct StructOfOtherTypes {
//     an_i8: i8,
//     another_i8: i8,
//     a_collection: Vec<String>,
// }
// impl JustATrait for StructOfOtherTypes {}
//
// struct ArrayAndI8 {
//     array: [i8; 1000], // This one will be very large
//     an_i8: i8,
//     in_u8: u8,
// }
// impl JustATrait for ArrayAndI8 {}
//
// fn returns_just_a_trait_1() -> impl JustATrait {
//     EnumOfNumbers::OneMoreI8(8)
// }
//
// fn returns_just_a_trait_2() -> Box<dyn JustATrait> {
//     Box::new(EnumOfNumbers::I8(8))
// }
//
// fn main() {
//     println!(
//         "{}, {}, {}, {}, {}",
//         size_of::<EnumOfNumbers>(),
//         size_of::<StructOfNumbers>(),
//         size_of::<EnumOfOtherTypes>(),
//         size_of::<StructOfOtherTypes>(),
//         size_of::<ArrayAndI8>(),
//     );
//     let x = returns_just_a_trait_1();
//     let y: Box<dyn JustATrait> = returns_just_a_trait_2();
//     x.print_a_trait();
//     y.print_a_trait();
// }
