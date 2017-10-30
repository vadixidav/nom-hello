#[macro_use]
extern crate nom;

use std::io::stdin;
use std::io::Read;
use std::str::{self, FromStr};
use std::f64;

fn main() {
    // Extract any number (5, -5, 1.0, -1.0, etc) and convert to f64.
    named!(num<f64>, ws!(alt_complete!(
        // Handle the case it is 1.0, -1.0, etc. (normal float)
        call!(nom::double) |
        // Handle the case that it is -5, 5, etc. (integer)
        map_res!(
            map_res!(
                // Try to recognize the consumption of an optional - followed by any number of digits.
                recognize!(preceded!(opt!(tag!("-")), nom::digit)),
                str::from_utf8
            ),
            // Convert integer to f64.
            f64::from_str
    ))));

    // Evaluate remaining list elements and apply the function on it to modify the accumulator initialized with `init`.
    named_args!(foldop<'a>(init: f64, f: &'a Fn(f64, f64) -> f64)<f64>, fold_many0!(eval, init, f));

    // Evaluate a list which is wrapped in parenthesis and begins with an atom followed by expressions.
    named!(list<f64>, delimited!(tag!("("), ws!(alt_complete!(
        // Addition
        preceded!(tag!("+"), apply!(foldop, 0.0, &|acc, item| acc + item)) |
        // Subtraction
        do_parse!(tag!("-") >> init: num >> res: apply!(foldop, init, &|acc, item| acc - item) >> (res)) |
        // Multiplication
        preceded!(tag!("*"), apply!(foldop, 1.0, &|acc, item| acc * item)) |
        // Division
        do_parse!(tag!("/") >> init: num >> res: apply!(foldop, init, &|acc, item| acc / item) >> (res))
    )), tag!(")")));

    // Evaluate list or a number.
    named!(eval<f64>, ws!(alt!(list | num)));

    let mut s = String::new();
    stdin().read_to_string(&mut s).expect("error: input was not a valid utf-8 string");

    println!("\n{:?}", eval(s.as_bytes()));
}
