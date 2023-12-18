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
      println!("res: {}",part1(&input_map));
      println!("res: {}",part2(&input_map));
    }
}


fn create_cards<'a>(lines: &mut io::Lines<io::BufReader<File>>) -> Vec<Vec<char>> {
  let mut input: Vec<Vec<char>> = Vec::new();

  while let Some(line) = lines.next() {
    input.push(line.unwrap().chars().collect());
  }

  input
}

fn part1(input: &Vec<Vec<char>>)-> i32 {
  let mut position = (0,0);
  for (idy, line) in input.iter().enumerate(){
    for (idx, c) in line.iter().enumerate(){
      if c == &'S' { position=(idy, idx); break;}
    }
  }
  let mut clear_path: Vec<Vec<char>> = input.clone();

  if let Some(count) = find_next_tile(&input, &mut (position.0+1, position.1), &mut 'N', &mut clear_path){
    count/2
  } else if let Some(count) = find_next_tile(&input, &mut(position.0-1, position.1), &mut 'S', &mut clear_path){
    count/2
  } else  if let Some(count) = find_next_tile(&input, &mut(position.0, position.1+1), &mut 'W', &mut clear_path){
    count/2
  } else if let Some(count) = find_next_tile(&input, &mut(position.0, position.1-1), &mut 'E', &mut clear_path){
    count/2
  } else {
      0
  }
}

fn find_next_tile(map: &Vec<Vec<char>>, position: &mut (usize,usize), coming_from: &mut char,clear_path: &mut Vec<Vec<char>>) -> Option<i32>{
  let mut tile = map[position.0][position.1];
  let mut count = 0;
  while tile != 'S' && tile != '.'{
    count +=1;
    match tile {
      '|' => {
        clear_path[position.0][position.1] = tile;
        match coming_from {
          'N' => if position.0 == map.len()-1 { tile = '.' } else { position.0 +=1; *coming_from='N'; tile = map[position.0][position.1];},
          'S' => if position.0 == 0 { tile = '.' } else { position.0 -=1; *coming_from='S'; tile = map[position.0][position.1];},
          _ => return None::<i32>
        }
      },
      '-'=> {
        clear_path[position.0][position.1] = tile;
        match coming_from {
          'W' => if position.1 == map[0].len()-1 { tile = '.' } else { position.1 +=1; *coming_from='W'; tile = map[position.0][position.1];},
          'E' => if position.1 == 0 { tile = '.' } else { position.1 -=1; *coming_from='E'; tile = map[position.0][position.1];},
          _ =>return  None::<i32>
        }
      },
      'L'=> {
        clear_path[position.0][position.1] = tile;
        match coming_from {
        'N' => if position.1 == map[0].len()-1 { tile = '.' } else { position.1 +=1; *coming_from='W'; tile = map[position.0][position.1];},
        'E' => if position.0 == 0 { tile = '.' } else { position.0 -=1; *coming_from='S'; tile = map[position.0][position.1];},
        _ =>return  None::<i32>
      }},
      'J'=> {
        clear_path[position.0][position.1] = tile;
        match coming_from {
        'W' => if position.0 == 0 { tile = '.' } else { position.0 -=1; *coming_from='S'; tile = map[position.0][position.1];},
        'N' => if position.1 == 0 { tile = '.' } else { position.1 -=1; *coming_from='E'; tile = map[position.0][position.1];},
        _ =>return  None::<i32>
      }},
      '7'=> {
        clear_path[position.0][position.1] = tile;
        match coming_from {
        'W' => if position.0 == map.len()-1 { tile = '.' } else { position.0 +=1; *coming_from='N'; tile = map[position.0][position.1];},
        'S' => if position.1 == 0 { tile = '.' } else { position.1 -=1; *coming_from='E'; tile = map[position.0][position.1];},
        _ =>return  None::<i32>
      }},
      'F'=> {
        clear_path[position.0][position.1] = tile;
        match coming_from {
        'E' => if position.0 == map.len()-1 { tile = '.' } else { position.0 +=1; *coming_from='N'; tile = map[position.0][position.1];},
        'S' => if position.1 == map[0].len()-1 { tile = '.' } else { position.1 +=1; *coming_from='W'; tile = map[position.0][position.1];},
        _ => return None::<i32>
      }},
      _ => return None::<i32>
    };
  }
  if tile == 'S' {Some(count+1)} else {None}
}

fn part2(input: &Vec<Vec<char>>)-> i32 {
  let mut position = (0,0);
  for (idy, line) in input.iter().enumerate(){
    for (idx, c) in line.iter().enumerate(){
      if c == &'S' { position=(idy, idx); break;}
    }
  }
  let mut clear_path: Vec<Vec<char>> = input.clone();

  for line in clear_path.iter_mut(){
    for  c in line.iter_mut(){
      *c = ' ';
    }
  }

  if let Some(_) = find_next_tile(&input, &mut (position.0+1, position.1), &mut 'N', &mut clear_path){
    if position.0 > 0 && "7F|".chars().any(|x| x == clear_path[position.0-1][position.1]) {clear_path[position.0][position.1] = '|';}
    if position.1 > 0 && "-LF".chars().any(|x| x == clear_path[position.0][position.1-1]) {clear_path[position.0][position.1] = '7';}
    if position.1 < clear_path[0].len()-1 && "-7J".chars().any(|x| x == clear_path[position.0][position.1-1]) {clear_path[position.0][position.1] = 'F';}
  } else if let Some(_) = find_next_tile(&input, &mut(position.0-1, position.1), &mut 'S', &mut clear_path){
    if position.0 < clear_path.len()-1 && "JL|".chars().any(|x| x == clear_path[position.0+1][position.1]) {clear_path[position.0][position.1] = '|';}
    if position.1 > 0 && "-LF".chars().any(|x| x == clear_path[position.0][position.1-1]) {clear_path[position.0][position.1] = 'J';}
    if position.1 < clear_path[0].len()-1 && "-7J".chars().any(|x| x == clear_path[position.0][position.1-1]) {clear_path[position.0][position.1] = 'L';}
  } else  if let Some(_) = find_next_tile(&input, &mut(position.0, position.1+1), &mut 'W', &mut clear_path){
    if position.1 > 0 && "-LF".chars().any(|x| x == clear_path[position.0][position.1-1]) {clear_path[position.0][position.1] = '-';}
    if position.0 > 0 && "|F7".chars().any(|x| x == clear_path[position.0-1][position.1]) {clear_path[position.0][position.1] = 'L';}
    if position.0 < clear_path.len()-1 && "|JL".chars().any(|x| x == clear_path[position.0+1][position.1]) {clear_path[position.0][position.1] = 'F';}
  } else if let Some(_) = find_next_tile(&input, &mut(position.0, position.1-1), &mut 'E', &mut clear_path){
    if position.1 < clear_path[0].len()-1 && "-7J".chars().any(|x| x == clear_path[position.0][position.1+1]) {clear_path[position.0][position.1] = '-';}
    if position.0 > 0 && "|F7".chars().any(|x| x == clear_path[position.0-1][position.1]) {clear_path[position.0][position.1] = 'L';}
    if position.0 < clear_path.len()-1 && "|JL".chars().any(|x| x == clear_path[position.0+1][position.1]) {clear_path[position.0][position.1] = 'F';}
  }
  let mut count = 0;

  for (idx, line) in input.iter().enumerate(){
    let mut inside = false;
    let mut last = ' ';
    for idy in 0..line.len() {
      match clear_path[idx][idy] {
        s if s != ' ' => {
          if s == '|'
          || s == 'F'
          || s == 'L'
          || (s == 'J' && last == 'L')
          || (s == '7' && last == 'F') {
            last = s;
            inside ^= true;
          }
        }
        _ => {if inside { count += 1;}}
      }
    }
  }
  count
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
