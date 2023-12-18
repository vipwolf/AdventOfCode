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
      let maps = split_into_maps(&mut lines);
      println!("res: {}",maps.iter().map(|x| {
        let transposed: Vec<Vec<_>> = tranpose(&x);
        part1(&x)+part1(&transposed)*100
      }).sum::<usize>());

      println!("res: {}",maps.iter().map(|x| {
        let transposed: Vec<Vec<_>> = tranpose(&x);
        part2(&x)+part2(&transposed)*100
      }).sum::<usize>());
    }
}

fn tranpose<A: Copy>(x: &Vec<Vec<A>>) -> Vec<Vec<A>> {
  let rows = x.len();
  (0..x[0].len()).map(|col| {
    (0..rows)
        .map(|row| x[row][col])
        .collect()
  }).collect()
}

fn part2(map: &Vec<Vec<char>>) -> usize {
  let mut pairs: Vec<(usize,usize,u32)> = Vec::new();
  for ity in 0..map[0].len()-1 {
    let left_end = if ity == 0 { 1 } else {map[0].len()-1};
    for ity_reverse in (left_end..map[0].len()).rev() {
      if ity!= ity_reverse  && (ity+ity_reverse)%2 != 0{
        pairs.push((ity, ity_reverse, 0));
      }
    }
  }
  for itx in 0..map.len() {
    let line = &map[itx];
    pairs.retain_mut(|p| {
      let mut right = p.1;
      let mut left = p.0;
      while left < right {
        if line[left] != line[right]{p.2 += 1}
        if p.2>1 {return false}
        right -=1;
        left +=1;
      }
      true
    });
  }
  pairs.retain(|p| p.2==1);
  if pairs.len() == 1 {
    return pairs[0].0+(pairs[0].1-pairs[0].0).div_ceil(2);
  }
  0
}

fn part1(map: &Vec<Vec<char>>) -> usize {
  let mut pairs: Vec<(usize,usize)> = Vec::new();
  for ity in 0..map[0].len()-1 {
    let left_end = if ity == 0 { 1 } else {map[0].len()-1};
    for ity_reverse in (left_end..map[0].len()).rev() {
      if ity!= ity_reverse  && map[0][ity]==map[0][ity_reverse] && (ity+ity_reverse)%2 != 0{
        pairs.push((ity, ity_reverse));
      }
    }
  }
  for itx in 0..map.len() {
    let line = &map[itx];
    pairs.retain(|p|{
      let mut right = p.1;
      for left in p.0..p.1 {
        if line[left] != line[right] {return false}
        right -=1;
      }
      true
    });
  }

  if pairs.len() == 1 {
    return pairs[0].0+(pairs[0].1-pairs[0].0).div_ceil(2);
  }
  0
}

fn split_into_maps<'a>(lines: &mut io::Lines<io::BufReader<File>> ) -> Vec<Vec<Vec<char>>> {
  let mut maps:Vec<Vec<Vec<char>>> = Vec::new();
  let mut map: Vec<Vec<char>> = Vec::new();
  while let Some(line) = lines.next() {
    let line_str = line.unwrap_or("".to_string());
    if line_str.is_empty() {
      maps.push(map);
      map= Vec::new();
    } else {
      let a: Vec<char> = line_str.chars().collect();
      map.push(a);
    }
  }
  maps.push(map);
  maps
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
