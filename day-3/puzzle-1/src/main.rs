use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::process;

fn main() {
  let submarine_power_consumption = compute_submarine_power_consumption().unwrap_or_else(|err| {
    println!("Error reading the file: {}", err);
    process::exit(1);
  });

  println!("{}", submarine_power_consumption);
}

fn compute_submarine_power_consumption() -> Result<i32, Box<dyn Error>> {
  let file = File::open("../report.txt")?;
  let reader = BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|res| res.expect("Could not parse line")).collect();

  Ok(1)
}