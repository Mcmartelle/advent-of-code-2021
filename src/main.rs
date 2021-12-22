extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;
#[derive(Debug, Deserialize)]
struct Record {
    depth: u32,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut increases: u32 = 0;
    let mut prev: u32 = 9999;
    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("{:?}", record);
        if record.depth > prev {
            increases += 1;
        }
        prev = record.depth;
    }
    println!("Increases: {:?}", increases);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}