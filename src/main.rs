#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate nom;

use std::io::stdin;
use std::io::Read;
use nom::IResult;
use std::str::{self, FromStr};
use std::f64;

fn main() {
    named!(num<f64>, ws!(alt_complete!(call!(nom::double) | map_res!(
      map_res!(
        recognize!(preceded!(opt!(tag!("-")), nom::digit)),
        str::from_utf8
      ),
      f64::from_str
    ))));

    // Fold operation
    fn foldop<F: Fn(f64, f64) -> f64>(input: &[u8], init: f64, f: F) -> IResult<&[u8], f64> {
        named_args!(fopl<'a>(init: f64, f: &'a Fn(f64, f64) -> f64)<f64>, fold_many0!(eval, init, f));
        fopl(input, init, &f)
    }

    // Evaluate list or number
    named!(eval<f64>, ws!(alt!(delimited!(tag!("("), ws!(alt_complete!(
        preceded!(tag!("+"), apply!(foldop, 0.0, |acc, item| acc + item)) |
        do_parse!(tag!("-") >> init: num >> res: apply!(foldop, init, |acc, item| acc - item) >> (res)) |
        preceded!(tag!("*"), apply!(foldop, 1.0, |acc, item| acc * item)) |
        do_parse!(tag!("/") >> init: num >> res: apply!(foldop, init, |acc, item| acc / item) >> (res))
    )), tag!(")")) | num)));

    let mut s = String::new();
    let input = stdin();
    input.lock().read_to_string(&mut s).unwrap();

    println!("\n{:?}", eval(s.as_bytes()));
}
