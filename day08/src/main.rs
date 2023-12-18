use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use std::env;
use regex::Regex;
use std::collections::HashMap;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      println!("Missing argument for input file");
      exit(1)
    }

    if let Ok(mut lines) = read_lines(&args[1]) {
      let input = lines.next().unwrap().unwrap();
      let mut input_map =create_cards(&mut lines);
      println!("res: {}",part1(&input,&input_map));
      println!("res: {}",part2(&input, &mut input_map));
    }
}


fn create_cards<'a>(lines: &mut io::Lines<io::BufReader<File>>) -> HashMap<String, (String, String)> {
  let mut mapping_values = HashMap::new();


  let re = Regex::new(r"([^ ]+) = \(([^,]+), ([^)]+)\)").unwrap();
  while let Some(line) = lines.next() {
    for (_, [identifier, left, right]) in re.captures_iter(&line.unwrap()).map(|c| c.extract()) {
      mapping_values.insert(identifier.to_string(), (left.to_string(),right.to_string()));
    }
  }

  return mapping_values;
}

fn part1(input: &String, input_map: &HashMap<String, (String, String)>)-> usize {
  let mut counter: usize = 0;
  let mut result = "AAA".to_string();
  while result != "ZZZ".to_string() {

    let lookup_value = input.chars().clone().nth(counter % input.len() ).unwrap();
    let lookup_tuple = input_map.get(&result).unwrap();
    match lookup_value {
      'R' => result = lookup_tuple.1.clone(),
      'L' => result = lookup_tuple.0.clone(),
      _ => {panic!("Found unmatched")}
    }

    counter +=1;
  }
  counter
}

fn part2(input: &String, input_map: &mut HashMap<String, (String, String)>)-> usize {

  let results: Vec<_> = input_map.keys().filter(|x| x.chars().nth(2) == Some('A')).collect();

  let steps_per_start:Vec<usize> = results.iter().map(|f| {
    let mut counter: usize = 0;
    let mut result: String = (**f).clone();
    while !(result).ends_with('Z')  {

      let lookup_value = input.chars().clone().nth(counter % input.len() ).unwrap();
      let lookup_tuple = input_map.get(&result).unwrap();
      match lookup_value {
        'R' => result = lookup_tuple.1.clone(),
        'L' => result = lookup_tuple.0.clone(),
        _ => {panic!("Found unmatched")}
      }
      counter +=1;
    }

    counter
    }
  ).collect();
  let ggt = ggt(&steps_per_start);

  ggt * steps_per_start.iter().map(|value| value / ggt).product::<usize>()
}

fn ggt(values: &[usize]) -> usize
{
    let mut ggt = values[0];
    for value in values {
        ggt = find_ggt(ggt, *value);
        if ggt == 1 {
            return 1;
        }
    }

    ggt
}

fn find_ggt(a: usize, b: usize) -> usize
{
    if a == 0 {
        return b;
    }

    find_ggt(b % a, a)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
