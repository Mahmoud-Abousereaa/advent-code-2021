use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::process;

fn main() {
  let number_of_measurements = get_large_measurments().unwrap_or_else(|err| {
    println!("Error reading the file: {}", err);
    process::exit(1);
  });

  println!("{}", number_of_measurements);
}

fn get_large_measurments() -> Result<i32, Box<dyn Error>> {
  let file = File::open("../measurements.txt")?;
  let reader = BufReader::new(file);
  let mut count = 0;
  let mut prev_measurement: Option<i32> = None;
  
  for line in reader.lines() {
    let measurement_line = line.unwrap();
    let measurement: i32 = measurement_line.trim().parse().expect("Measurement is not available");
    let is_none = match prev_measurement {
      Some(_val) => true,
      None => false,
    };

    if is_none && Some(measurement).unwrap() > prev_measurement.unwrap() {
      count+= 1;
    }

    prev_measurement = Some(measurement);
  }

  Ok(count)
}
