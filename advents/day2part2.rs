extern crate csv;
// extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
enum Direction {
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "up")]
    Up,
}

#[derive(Debug, Deserialize)]
struct Record {
    direction: Direction,
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
        match record.direction {
            Direction::Forward => {
                horiz_pos += record.magnitude;
                depth += aim * record.magnitude;
            },
            Direction::Down => aim += record.magnitude,
            Direction::Up => aim -= record.magnitude,
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