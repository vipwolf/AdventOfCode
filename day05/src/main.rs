use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{RangeInclusive};
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
      let schematic = create_cards(&mut lines);
      let seeds = schematic.0.unwrap();
      let mutators = schematic.1;

      println!("res: {}", seeds.iter().map(|x| part1(x, &mutators)).min().unwrap());
      println!("res: {}", part2(&seeds, &mutators));
    }
}

fn create_cards<'a>(lines: &mut io::Lines<io::BufReader<File>>) -> (Option<Vec<u64>>,Vec<Vec<(RangeInclusive<u64>,RangeInclusive<u64>)>>) {
  let seed_line = lines.next().unwrap().unwrap();
  let seeds: Vec<&str> = seed_line.split(": ").collect();
  let seed_numbers: Option<Vec<u64>>;
  if let Some(seed_numbers_str) = seeds.last() {
    seed_numbers = Some(seed_numbers_str.to_owned().split_ascii_whitespace().map(|x| x.parse().unwrap()).collect());
  }
  else {
    seed_numbers = None;
  }
  // Skip empty line
  lines.next();
  let mut input_maps: Vec<Vec<(RangeInclusive<u64>,RangeInclusive<u64>)>> = Vec::new();
  for line in lines {
    match line {
      Ok(x) if(x.contains(':')) => { input_maps.push(Vec::new())}

      Ok(x) if(x.len() > 0) => {
        let numbers = input_maps.last_mut().unwrap();
        let seed_numbers: Vec<u64> = x.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        numbers.push((
          RangeInclusive::new(seed_numbers[1], seed_numbers[1]+seed_numbers[2]-1),
          RangeInclusive::new(seed_numbers[0], seed_numbers[0]+seed_numbers[2]-1)
        ));
      }
      _ => {}
    }
  }
  return (seed_numbers,input_maps);
}

fn part1(seed: &u64, mutators: &Vec<Vec<(RangeInclusive<u64>,RangeInclusive<u64>)>> ) -> u64 {
  let mut result=*seed;
  for maps in mutators {
    for mapc in maps {
      if mapc.0.contains(&result) {
        let offset = result - mapc.0.start();
        result = mapc.1.start()+ offset;
        break;
      }
    }
  }
  return result;
}

fn part2( original_seed: &Vec<u64>, mutators: &Vec<Vec<(RangeInclusive<u64>,RangeInclusive<u64>)>> ) -> u64 {
  let mut seeds: Vec<u64> = Vec::new();
  let mut it = original_seed.iter();
  while let Some(start_number) = it.next(){
    if let Some(end_number) = it.next(){
      let range:RangeInclusive<u64> = RangeInclusive::new(*start_number,start_number+end_number-1);
      seeds.push(range.map(|x| part1(&x, &mutators)).min().unwrap());
    }
  }
  *seeds.iter().min().unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
