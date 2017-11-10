#[macro_use]
extern crate nom;

use std::io::stdin;
use std::io::Read;
use std::str::{self, FromStr};
use std::f64;

// Extract any number (5, -5, 1.0, -1.0, etc) and convert to f64.
named!(num<f64>, alt_complete!(
    // Handle the case it is 1.0, -1.0, etc. (normal float)
    call!(nom::double) |
    // Handle the case that it is -5, 5, etc. (integer)
    map_res!(
        map_res!(
            // Try to recognize the consumption of an optional - followed by any number of digits.
            recognize!(preceded!(opt!(tag!("-")), nom::digit)),
            // Convert from bytes to utf-8 string.
            str::from_utf8
        ),
        // Convert integer string to f64.
        f64::from_str
)));

// Evaluate a list which is wrapped in parenthesis and begins with an atom followed by expressions.
named!(list<f64>, delimited!(tag!("("), ws!(alt_complete!(
    // Addition
    preceded!(tag!("+"), fold_many0!(eval, 0.0, |acc, item| acc + item)) |
    // Subtraction
    do_parse!(tag!("-") >> init: num >> res: fold_many0!(eval, init, |acc, item| acc - item) >> (res)) |
    // Multiplication
    preceded!(tag!("*"), fold_many0!(eval, 1.0, |acc, item| acc * item)) |
    // Division
    do_parse!(tag!("/") >> init: num >> res: fold_many0!(eval, init, |acc, item| acc / item) >> (res))
)), tag!(")")));

// Evaluate list or a number.
named!(eval<f64>, ws!(alt!(list | num)));

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).expect("error: input was not a valid utf-8 string");

    println!("\n{:?}", eval(s.as_bytes()));
}

#[test]
fn test0() {
    match eval("(+    (   - 2 0.1    (*   4 (+ 2 -0.3) 2) (   + 4 2   )   )(+ 1 1))".as_bytes()) {
        nom::IResult::Done(_, res) => assert_eq!(res, -15.7),
        _ => panic!(),
    }
}
