use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::process;

fn main() {
  let number_of_positive_comparisons = get_large_measurments().unwrap_or_else(|err| {
    println!("Error reading the file: {}", err);
    process::exit(1);
  });

  println!("{}", number_of_positive_comparisons);
}

fn get_large_measurments() -> Result<i32, Box<dyn Error>> {
  let file = File::open("../measurements.txt")?;
  let reader = BufReader::new(file);
  let mut count = 0;
  let mut current_line = 0;
  let mut previous_three_measurements: Vec<i32> = vec![];
  let mut current_three_measurements: Vec<i32> = vec![];
  for line in reader.lines() {
    let measurement_line = line.unwrap();
    let measurement: i32 = measurement_line.trim().parse().expect("Measurement is not available");

    if current_line < 1 {
      previous_three_measurements.push(measurement);
    } else if current_line < 3 {
      previous_three_measurements.push(measurement);
      current_three_measurements.push(measurement);
    } else {
      current_three_measurements.push(measurement);
    }

    if current_line >= 3 {
      let previous_measurement_sum: i32 = previous_three_measurements.iter().sum();
      let current_measurement_sum: i32 = current_three_measurements.iter().sum();

      if previous_measurement_sum < current_measurement_sum {
        count += 1;
      }

      let prev_measurement = current_three_measurements[2].clone();
      previous_three_measurements.push(prev_measurement);
      previous_three_measurements.remove(0);
      current_three_measurements.remove(0);
    }

    current_line += 1;
  }

  Ok(count)
}
