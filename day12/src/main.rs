use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use std::env;
use hashbrown::HashMap;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      println!("Missing argument for input file");
      exit(1)
    }

    if let Ok(lines) = read_lines(&args[1]) {
      let mut lines_it: io::Lines<io::BufReader<File>> = lines.lines();
      let mut sum_1 = 0;
      let mut sum_2 = 0;
      while let Some(line ) = lines_it.next(){
        let l = line.unwrap();
        sum_1 += part1(split_into_patterns(&l));
        sum_2 += part2(split_into_patterns(&l));
      }

      println!("res: {}",sum_1);
      println!("res: {}",sum_2);
    }
}

fn part1(tuple:(& str, Vec<usize>) ) -> usize {
  let mut cache: HashMap<(&str, usize, &[usize]), usize> = HashMap::new();
  possible_ways(&mut cache, tuple.0, None, &tuple.1)
}

fn part2(tuple:(& str, Vec<usize>) ) -> usize {
  let input :Vec<_>= (0..5).map(|_| tuple.0).collect();
  let patterns : Vec<usize>= (0..5).flat_map(|_| &tuple.1).map(|y| *y).collect();

  let mut cache: HashMap<(&str, usize, &[usize]), usize> = HashMap::new();
  possible_ways(&mut cache, &input.join("?"), None, &patterns)
}

fn possible_ways<'a>(cache: &mut HashMap<(&'a str, usize, &'a[usize]), usize> ,input: &'a str, matched_hashes: Option<usize>, patterns: &'a[usize]) -> usize {
  if input.is_empty() {
    return match (matched_hashes, patterns.len()) {
      (None, 0) => 1,
      (Some(x), 1) if x == patterns[0] => 1,
      _ => 0
    };
  }
  if matched_hashes.is_some() && patterns.is_empty() {
    return 0;
  }

  let key: (&str, usize, &[usize]) = (input,matched_hashes.unwrap_or(0), patterns);
  if let Some(k) =  cache.get(&key) {
    return *k;
  }

  let ways = match (input.chars().nth(0).unwrap(), matched_hashes) {
    ('.', Some(x)) if x != patterns[0] => 0,
    ('.', Some(_)) => possible_ways( cache, &input[1..], None, &patterns[1..]),
    ('.', None)    => possible_ways( cache, &input[1..], None, patterns),
    ('#', Some(_)) => possible_ways( cache,&input[1..], matched_hashes.map(|x| x+1), patterns),
    ('#', None)    => possible_ways( cache,&input[1..], Some(1), patterns),
    ('?', Some(x)) => {
      let mut ans = possible_ways( cache,&input[1..], matched_hashes.map(|x| x+1), patterns);
      if x == patterns[0] {
        ans += possible_ways( cache,&input[1..], None, &patterns[1..])
      }
      ans
    }
    ('?', None) =>
      possible_ways( cache, &input[1..], Some(1), patterns) +
      possible_ways( cache, &input[1..], None, patterns),
    _ => unreachable!(),
  };
  cache.insert(key, ways);
  ways
}

fn split_into_patterns<'a>(line: &'a str) -> (&'a str, Vec<usize>) {
  let tuple = line.split_once(" ").unwrap();
  let right_side: Vec<usize> = tuple.1.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
  (tuple.0, right_side)
}

fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
