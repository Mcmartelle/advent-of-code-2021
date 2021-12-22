extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;


// enum Direction {
//     String::from("forward"),
//     String::from("down"),
//     String::from("up"),
// }

#[derive(Debug, Deserialize)]
struct Record {
    direction: String,
    magnitude: i32,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut aim: i32 = 0;
    let mut horiz_pos: i32 = 0;
    let mut depth: i32 = 0;
    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("{:?}", record);
        match record.direction.as_ref() {
            "forward" => {
                horiz_pos += record.magnitude;
                depth += aim * record.magnitude;
            },
            "down" => aim += record.magnitude,
            "up" => aim -= record.magnitude,
            _ => ()
        }
    }
    println!("Product: {:?}", horiz_pos * depth);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}