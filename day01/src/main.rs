use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;


struct NumberLocation {
  index: Option<usize>,
  number: (String, String)
}

fn main() {
    let file_path = "/workspaces/AdventOfCode/day1/input/input.txt";
    let mut output = 0;
    if let Ok(lines) = read_lines(file_path) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
          if let Ok(mut ip) = line {


              //println!("For this  line :{}", ip);
              //find_numbers(&ip);
              let ip2 = replace_string(ip);
              //println!("{}",ip2);

              let mut two_digits: String = "".to_string();
              let mut found_number = false;
              let mut second_digit: String = "".to_string();
              for c in ip2.chars() {
                if c.is_numeric() {

                  if !found_number {
                    two_digits = c.to_string().clone();
                    found_number = true;
                  }
                  second_digit = c.to_string().clone();
                }
              }
              two_digits.push_str(&second_digit);
              let num: u32 = two_digits.parse().unwrap();
              //println!(", sum = {}", num);
              output += num;
          }
      }
    }
    println!("res {}",{output});
}

fn replace_string(line: String) -> String{
  let mut result = line;
  if let Some(n) = find_numbers(&mut result, 0) {
    let char_index = Range{start: n.index.unwrap(), end: n.index.unwrap()+n.number.0.len()};
    result.replace_range(char_index, &n.number.1);
  }
  if let Some(n) = find_numbers(&mut result, 1) {
    let char_index = Range{start: n.index.unwrap(), end: n.index.unwrap()+n.number.0.len()};
    result.replace_range(char_index, &n.number.1);
  }
  return result;
}

fn find_numbers(line:&mut String, mode: usize) -> Option<NumberLocation>{
  let mut result = NumberLocation {
    index: None,
    number: (String::from(""), String::from(""))
  };
  let NUMBER_REPLACEMENTS: [( String, String); 18] = [
    (String::from("one"),String::from("1")),
    (String::from("two"), String::from("2")),
    (String::from("three"),String::from("3")),
    (String::from("four"),String::from("4")),
    (String::from("five"),String::from("5")),
    (String::from("six") ,String::from("6")),
    (String::from("seven"),String::from("7")),
    (String::from("eight"),String::from("8")),
    (String::from("nine"), String::from("9")),
    (String::from("1"),String::from("1")),
    (String::from("2"), String::from("2")),
    (String::from("3"),String::from("3")),
    (String::from("4"),String::from("4")),
    (String::from("5"),String::from("5")),
    (String::from("6") ,String::from("6")),
    (String::from("7"),String::from("7")),
    (String::from("8"),String::from("8")),
    (String::from("9"), String::from("9"))

  ];


  for x in NUMBER_REPLACEMENTS {
    match_number(&line, x, &mut result, mode);
  }

  match result.index {
    None => return None,
    Some(_r) => {
      //println!("Found number {} at {}", result.number.0, result.index.unwrap());
      return Some(result);
    }
  }
}

fn match_number(line:&String, matcher: (String, String), index: &mut NumberLocation, mode: usize) {
  let index_match;
  if mode == 0 {
    index_match = line.find(&matcher.0);
  } else {
    index_match = line.rfind(&matcher.0);
  }
  if let Some(m) = &index_match {
    //println!("Found {} at {}",matcher.0, m);
    match index.index {
      None => {
        index.index= Some(*m);
        index.number = matcher;
      },
      Some(number) => {
        if (mode == 0 && number > *m ) ||
          (mode == 1 && number < *m){
          index.index= Some(*m);
          index.number = matcher;
        }
      }
    }
  }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
