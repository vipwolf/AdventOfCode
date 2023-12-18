use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      println!("Missing argument for input file");
      exit(1)
    }

    if let Ok(mut lines) = read_lines(&args[1]) {
      let races = create_races(lines.next().unwrap().unwrap(), lines.next().unwrap().unwrap());

      println!("res: {}", races.iter().map(part1).product::<u64>());
      println!("res: {:#?}", races.into_iter().reduce(part2).map(|a| part1(&a)).unwrap());
    }
}

fn create_races(line1: String, line2: String) -> Vec<(u64,u64)> {
  let time_str: Vec<&str> = line1.split(": ").collect();
  let distance_str: Vec<&str> = line2.split(": ").collect();
  let mut tuples: Vec<(u64,u64)> = Vec::new();

  if let Some(time_numbers_str) = time_str.last() {
    if let Some(distance_numbers_str) = distance_str.last() {
      let time_numbers: Vec<u64> = time_numbers_str.to_owned().split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
      let distance_numbers: Vec<u64> = distance_numbers_str.to_owned().split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
      for idt in 0..time_numbers.len() {
        if let Some(distance) = distance_numbers.get(idt) {
          let time = time_numbers.get(idt).unwrap();
          tuples.push((*time, *distance));
        }
      }
    }
  }
  return tuples;
}

fn part1(seed: &(u64,u64))-> u64 {
  let p:f64 = seed.0 as f64;
  let q:f64 = seed.1 as f64;
  let lower: f64 = (p/2.0)-(((p/2.0).powf(2.0))-q).sqrt();
  let higher: f64 = (p/2.0)+(((p/2.0).powf(2.0))-q).sqrt();

  let result:u64 = (higher.ceil()- lower.floor()-1.0) as u64;

  return result;
}

fn part2<'a>(seed: (u64,u64), seed2: (u64,u64)) -> (u64,u64) {
  let number1 = format!("{}{}",seed.0, seed2.0).parse::<u64>().unwrap();
  let number2 = format!("{}{}",seed.1, seed2.1).parse::<u64>().unwrap();
  return (number1, number2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
