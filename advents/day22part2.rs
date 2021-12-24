extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
enum OnOff {
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

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
struct Cuboid {
    x_low: i32,
    x_high: i32,
    y_low: i32,
    y_high: i32,
    z_low: i32,
    z_high: i32,
}

impl Cuboid {
    fn new(x_low: i32, x_high: i32, y_low: i32, y_high: i32, z_low: i32, z_high: i32) -> Cuboid {
        Cuboid {
            x_low: x_low,
            x_high: x_high,
            y_low: y_low,
            y_high: y_high,
            z_low: z_low,
            z_high: z_high,
        }
    }

    fn intersects(a: Cuboid, b: Cuboid) -> bool {
        if a.x_high >= b.x_low &&
            a.x_low <= b.x_high &&
            a.y_high >= b.y_low &&
            a.y_low <= b.y_high &&
            a.z_high >= b.z_low &&
            a.z_low <= b.z_high
        {
            true
        } else {
            false
        }
    }

    fn split(a: Cuboid, b: Cuboid) -> Vec<Cuboid> {
        let mut v: Vec<Cuboid> = Vec::new();
        v.append(&mut Cuboid::split_x(a,b));
        v
    }

    fn split_x(a: Cuboid, b: Cuboid) -> Vec<Cuboid> {
        let mut v: Vec<Cuboid> = Vec::new();
        if a.x_low < b.x_low && b.x_low < a.x_high && a.x_low < b.x_high && b.x_high < a.x_high { //notch case
            v.push(Cuboid::new(a.x_low, b.x_low-1, a.y_low, a.y_high, a.z_low, a.z_high));
            v.push(Cuboid::new(b.x_high+1, a.x_high, a.y_low, a.y_high, a.z_low, a.z_high));
            v.append(&mut Cuboid::split_y(Cuboid::new(b.x_low, b.x_high, a.y_low, a.y_high, a.z_low, a.z_high), b));
        } else if a.x_low < b.x_low && b.x_low <= a.x_high { // high corner cut case
            v.push(Cuboid::new(a.x_low, b.x_low-1, a.y_low, a.y_high, a.z_low, a.z_high));
            v.append(&mut Cuboid::split_y(Cuboid::new(b.x_low, a.x_high, a.y_low, a.y_high, a.z_low, a.z_high), b));
        } else if a.x_low <= b.x_high && b.x_high < a.x_high { // low corner cut case
            v.push(Cuboid::new(b.x_high+1, a.x_high, a.y_low, a.y_high, a.z_low, a.z_high));
            v.append(&mut Cuboid::split_y(Cuboid::new(a.x_low, b.x_high, a.y_low, a.y_high, a.z_low, a.z_high), b));
        } else { // cheese slice case
            v.append(&mut Cuboid::split_y(a, b));
        }
        v
    }

    fn split_y(a: Cuboid, b: Cuboid) -> Vec<Cuboid> {
        let mut v: Vec<Cuboid> = Vec::new();
        if a.y_low < b.y_low && b.y_low < a.y_high && a.y_low < b.y_high && b.y_high < a.y_high { //notch case
            v.push(Cuboid::new(a.x_low, a.x_high, a.y_low, b.y_low-1, a.z_low, a.z_high));
            v.push(Cuboid::new(a.x_low, a.x_high, b.y_high+1, a.y_high, a.z_low, a.z_high));
            v.append(&mut Cuboid::split_z(Cuboid::new(a.x_low, a.x_high, b.y_low, b.y_high, a.z_low, a.z_high), b));
        } else if a.y_low < b.y_low && b.y_low <= a.y_high { // high corner cut case
            v.push(Cuboid::new(a.x_low, a.x_high, a.y_low, b.y_low-1, a.z_low, a.z_high));
            v.append(&mut Cuboid::split_z(Cuboid::new(a.x_low, a.x_high, b.y_low, a.y_high, a.z_low, a.z_high), b));
        } else if a.y_low <= b.y_high && b.y_high < a.y_high { // low corner cut case
            v.push(Cuboid::new(a.x_low, a.x_high, b.y_high+1, a.y_high, a.z_low, a.z_high));
            v.append(&mut Cuboid::split_z(Cuboid::new(a.x_low, a.x_high, a.y_low, b.y_high, a.z_low, a.z_high), b));
        } else { // cheese slice case
            v.append(&mut Cuboid::split_z(a, b));
        }
        v
    }

    fn split_z(a: Cuboid, b: Cuboid) -> Vec<Cuboid> {
        let mut v: Vec<Cuboid> = Vec::new();
        if a.z_low < b.z_low && b.z_low < a.z_high && a.z_low < b.z_high && b.z_high < a.z_high { //notch case
            v.push(Cuboid::new(a.x_low, a.x_high, a.y_low, a.y_high, a.z_low, b.z_low-1));
            v.push(Cuboid::new(a.x_low, a.x_high, a.y_low, a.y_high, b.z_high+1, a.z_high));
        } else if a.z_low < b.z_low && b.z_low <= a.z_high { // high corner cut case
            v.push(Cuboid::new(a.x_low, a.x_high, a.y_low, a.y_high, a.z_low, b.z_low-1));
        } else if a.z_low <= b.z_high && b.z_high < a.z_high { // low corner cut case
            v.push(Cuboid::new(a.x_low, a.x_high, a.y_low, a.y_high, b.z_high+1, a.z_high));
        } // No cheese slice case possible for last dimension (i think... or if they are it will all be part of b)
        v
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut v: Vec<Cuboid> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("{:?}", record);
        let b: Cuboid = Cuboid::new(record.x_low, record.x_high, record.y_low, record.y_high, record.z_low, record.z_high);
        let mut temp_v: Vec<Cuboid> = Vec::new();
        let mut read_head: usize = 0;
        match record.on_off {
            OnOff::On => {
                for _n in 0..v.len() {
                    if Cuboid::intersects(v[read_head], b) {
                        temp_v.append(&mut Cuboid::split(v.swap_remove(read_head),b));
                    } else {
                        read_head += 1;
                    }
                }
                v.push(b);
                v.append(&mut temp_v);
            },
            OnOff::Off => {
                for _n in 0..v.len() {
                    if Cuboid::intersects(v[read_head], b) {
                        temp_v.append(&mut Cuboid::split(v.swap_remove(read_head),b));
                    } else {
                        read_head += 1;
                    }
                }
                v.append(&mut temp_v);
            },
        }
    }
    // for c in &v {
    //     println!("{:?}, Volume:{:?}", c, calc_volume(*c));
    // }

    let sum: u64 = v.into_iter().map(calc_volume).sum();
    
    println!("Sum: {:?}", sum);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn calc_volume(item: Cuboid) -> u64 {
    (item.x_high-item.x_low+1) as u64 *(item.y_high-item.y_low+1) as u64 *(item.z_high-item.z_low+1) as u64
}