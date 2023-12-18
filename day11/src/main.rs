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
      let (x_lines, y_lines, galaxies) =create_cards(&mut lines);

      println!("res: {}",part1(&x_lines,&y_lines, &galaxies, 1));
      println!("res: {}",part1(&x_lines,&y_lines, &galaxies, 99999));
    }
}

fn part1(x_lines: &Vec<usize>, y_lines: &Vec<usize>,galaxies: &Vec<(usize, usize)>, galaxy_size: usize) ->usize {
  let mut count = 0;

  for idx in 0 .. galaxies.len()-1{
    for x in idx.. galaxies.len(){
      if galaxies[idx] != galaxies[x] {
        let galaxy_1_x =galaxies[idx].0;
        let galaxy_1_y =galaxies[idx].1;
        let galaxy_2_x =galaxies[x].0;
        let galaxy_2_y =galaxies[x].1;

        let mut distance = (galaxy_1_x).abs_diff(galaxy_2_x) + (galaxy_1_y).abs_diff(galaxy_2_y);
        let r = if galaxy_1_x < galaxy_2_x {std::ops::Range { start: galaxy_1_x, end: galaxy_2_x }} else {std::ops::Range { start: galaxy_2_x, end: galaxy_1_x }};
        distance += x_lines.iter().filter(|x| r.contains(x)).count()*galaxy_size;
        let r = if galaxy_1_y < galaxy_2_y {std::ops::Range { start: galaxy_1_y, end: galaxy_2_y }} else {std::ops::Range { start: galaxy_2_y, end: galaxy_1_y }};
        distance += y_lines.iter().filter(|x| r.contains(x)).count()*galaxy_size;
        count += distance;
      }
    }
  }
  count
}

fn create_cards<'a>(lines: &mut io::Lines<io::BufReader<File>>) -> (Vec<usize>, Vec<usize>,Vec<(usize, usize)>) {
  let mut input: Vec<Vec<char>> = Vec::new();
  let mut x_lines: Vec<_> = Vec::new();
  let mut y_lines: Vec<_> = Vec::new();
  while let Some(line) = lines.next() {
    input.push(line.unwrap().chars().collect());
  }

  for (idx, line) in input.iter().enumerate() {
    if line.iter().all(|x| x == &'.'){
      x_lines.push(idx);
    }
  }
  let mut galaxies = Vec::new();
  for idy in 0 .. input[0].len() {
    let mut found_galaxy = false;
    for idx in 0.. input.len() {
      if input[idx][idy] == '#' {
        found_galaxy = true;
        galaxies.push((idx,idy));
      }
    }
    if !found_galaxy { y_lines.push(idy) }
  }

  input.clear();
  (x_lines,y_lines, galaxies)
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
