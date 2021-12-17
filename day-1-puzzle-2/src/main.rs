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
  let file = File::open("measurements.txt")?;
  let reader = BufReader::new(file);
  let mut count = 0;
  let mut current_line = 0;
  let mut previous_three_measurements = 0;
  let mut current_three_measurements = 0;
  for line in reader.lines() {
    if current_line >= 4 {
      println!("previous_three_measurements: {}", previous_three_measurements);
      println!("current_three_measurements: {}", current_three_measurements);
      let diff = previous_three_measurements - current_three_measurements;
      if previous_three_measurements < current_three_measurements {
        count += 1;
      }

      previous_three_measurements = current_three_measurements;
      current_three_measurements = current_three_measurements - diff;
    } else {
      previous_three_measurements = current_three_measurements;
    }

    if current_line > 4 {
      break;
    }

    let measurement_line = line.unwrap();
    let measurement: i32 = measurement_line.trim().parse().expect("Measurement is not available");

    current_three_measurements += measurement;

    current_line += 1;
  }

  Ok(count)
}
