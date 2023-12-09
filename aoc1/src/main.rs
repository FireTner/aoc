use std::fs;

fn aoc1(input: String) -> Vec<u32> {
  let mut result: Vec<u32> = Vec::new();

  for line in input.lines() {
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut found_digit = false;

    for char in line.chars() {
      if !char.is_numeric() { continue; }
      
      let current_digit = char.to_digit(10).expect(
        "Error occured: expected digit"
      );

      if !found_digit {
        first_digit = current_digit;
        found_digit = true;
      }

      last_digit = current_digit;
    }

    result.push(first_digit + last_digit * 10);
  }
  
  return result;
}

fn main() {
  let file_path = "example";
  let input = fs::read_to_string(file_path).expect("Error occured while opening a file");
  
  println!("Input:\n{}", input);

  let result = aoc1(input);

  println!("{:?}", result);
}
