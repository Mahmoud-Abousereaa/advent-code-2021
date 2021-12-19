use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::process;

fn main() {
  let horizontal_depth_product = compute_submarine_horizontal_depth_values().unwrap_or_else(|err| {
    println!("Error reading the file: {}", err);
    process::exit(1);
  });

  println!("{}", horizontal_depth_product);
}

fn compute_submarine_horizontal_depth_values() -> Result<i32, Box<dyn Error>> {
  let file = File::open("../commands.txt")?;
  let reader = BufReader::new(file);
  let mut horizontal_pos = 0;
  let mut depth_pos = 0;
  let commands = vec!["forward", "down", "up"];

  for line in reader.lines() {
    let file_line = line.unwrap();
    let command = extract_command(&file_line, &commands);
    let command_value = file_line.replace(command, "");
    let movement: i32 = command_value.trim().parse().expect("Measurement is not available");

    if command == "forward" {
      horizontal_pos += movement;
    }

    if command == "down" {
      depth_pos += movement;
    }

    if command == "up" {
      depth_pos -= movement;
    }
  }

  let product = horizontal_pos * depth_pos;
  Ok(product)
}

fn extract_command <'a>(file_line: &'a str, commands: &'a Vec<&str>) -> &'a str {
  let mut command = "";
  for c in commands {
    if file_line.contains(c) {
      command = c;
    }
  }

  command
}