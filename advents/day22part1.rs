extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;
use ndarray::prelude::*;
use ndarray::{Array, Ix3};

Enum OnOff {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[derive(Debug, Deserialize)]
struct Record {
    on_off: OnOff,
    x_low: i32,
    x_high: i32,
    y_low: i32,
    y_high: i32,
    z_low: i32,
    z_high: i32,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut a: Array::<bool, Ix3>::from_elem((100,100,100), false);
    
    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("{:?}", record);
        bit_string_length = record.bit_string.len();
        break
    }

    let mut epsilon_vec = vec![0i32; bit_string_length];
    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("{:?}", record);
        for n in 0..record.bit_string.len() {
            match record.bit_string.chars().nth(n).as_ref() {
                Some('0') => epsilon_vec[n] -= 1,
                Some('1') => epsilon_vec[n] += 1,
                _ => ()
            }
        }
    }

    let epsilon_bit_string: String = epsilon_vec.iter().copied().map(|item|
        if item>0 {
            return 1;
        } else {
            return 0;
        }).map(|d| std::char::from_digit(d,2).unwrap()).collect();
    let gamma_bit_string: String = epsilon_vec.iter().copied().map(|item|
        if item>0 {
            return 0;
        } else {
            return 1;
        }).map(|d| std::char::from_digit(d,2).unwrap()).collect();
    let epsilon_int: i32 = i32::from_str_radix(&epsilon_bit_string, 2).unwrap();
    let gamma_int: i32 = i32::from_str_radix(&gamma_bit_string, 2).unwrap();

    let mut oxygen;
    let mut co2;
    
    println!("Product: {:?}", epsilon_int * gamma_int);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}