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
      let mut map: Vec<Vec<char>> = split_into_map(&mut lines);

      println!("res: {}", count(&tilt(&map)));

      let mut known_maps = Vec::new();
      let cycles: usize = 1000000000;
      for itx in 0..cycles {
        let key = map.iter().map(|l| l.iter().collect::<String>()).collect::<String>();
        if known_maps.contains(&key) {
          let position = known_maps.iter().position(|k| k == &key).unwrap_or(0);
          let remaining = (cycles - itx) % (known_maps.len()-position);
          map = (0..remaining*4).fold(map, |map2,_| {
            turn(&tilt( &map2))
          });
          break;
        }
        known_maps.push(key);
        map = (0..4).fold(map,|map3,_| turn(&tilt( &map3)));
      }

      println!("res: {}", count(&map));
    }
}

fn turn<A: Copy>(x: &Vec<Vec<A>>) -> Vec<Vec<A>> {
  let rows = x.len();
  (0..x[0].len()).map(|col| {
    (0..rows).rev()
        .map(|row| x[row][col])
        .collect()
  }).collect()
}

fn tilt<'a>(map2: & Vec<Vec<char>>) -> Vec<Vec<char>>{
  let mut map = map2.clone();
  for itx in 1 .. map.len() {
    for ity in 0 .. map[itx].len(){
      for itx_reverse in (0..itx).rev() {
        match (map[itx_reverse][ity], map[itx_reverse+1][ity]) {
          ('.','O') => {map[itx_reverse][ity]= 'O'; map[itx_reverse+1][ity]='.'}
          _ => {}
        }
      }
    }
  }
  map
}

fn count(map: & Vec<Vec<char>>) -> usize {
  let max = map.len();
  map.iter().enumerate().map(|(line, row)| {
    row.iter().filter(|l| {
      *l == &'O'
    }).count()*(max-line)
  }).sum::<usize>()
}

fn split_into_map<'a>(lines: &mut io::Lines<io::BufReader<File>> ) -> Vec<Vec<char>> {
  let mut map: Vec<Vec<char>> = Vec::new();
  while let Some(line) = lines.next() {
    let line_str = line.unwrap_or("".to_string());
    let a: Vec<char> = line_str.chars().collect();
    map.push(a);
  }
  map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
