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
    depth: i32,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut depth_vec: Vec<i32> = Vec::new();
    let mut tmsw: Vec<i32> = Vec::new(); // Three-Measurement sliding window

    for result in rdr.deserialize() {
        let record: Record = result?;
        depth_vec.push(record.depth);
    }

    for n in 1..depth_vec.len()-1 {
        let sum: i32 = depth_vec[n-1..=n+1].iter().copied().sum();
        tmsw.push(sum);
    }


    let mut increases: i32 = -1;
    let mut prev: i32 = -1;
    for depth in tmsw {
        if depth > prev {
            increases += 1;
        }
        prev = depth;
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