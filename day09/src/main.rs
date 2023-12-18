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
      let input_map =create_cards(&mut lines);
      println!("res: {}",input_map.iter().map(part1).sum::<i64>());
      println!("res: {}",input_map.iter().map(part2).sum::<i64>());
    }
}


fn create_cards<'a>(lines: &mut io::Lines<io::BufReader<File>>) -> Vec<Vec<i64>> {
  let mut input: Vec<Vec<i64>> = Vec::new();

  while let Some(line) = lines.next() {
    input.push(line.unwrap().split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect());
  }

  input
}

fn part1(input: &Vec<i64>)-> i64 {
  let mut next_input: Vec<_> = Vec::new();
  for idx in 0 .. input.len()-1 {
    next_input.push(input[idx+1]-input[idx]);
  }
  if next_input.iter().all(|x| *x == 0i64) {
    *input.last().unwrap()
  } else {
    let r = part1(&next_input);
    input.last().unwrap() + r
  }

}

fn part2(input: &Vec<i64>)-> i64 {
  let mut next_input: Vec<_> = Vec::new();
  for idx in (1..=input.len()-1).rev() {
      next_input.insert(0,input[idx]-input[idx-1]);
  }
  if next_input.iter().all(|x| *x == 0i64) {
    *input.first().unwrap()
  } else {
    let r = part2(&next_input);
    input.first().unwrap() - r
  }
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
