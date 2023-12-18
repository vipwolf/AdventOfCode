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

    if let Ok(mut lines) = read_lines(&args[1]) {
      if let Some(input) = split_into_map(&mut lines) {
        let mut map: HashMap<i32, Vec<(String, u32)>> = HashMap::new();
        println!("res: {}", input.iter().map(part1).sum::<i32>());
        input.iter().for_each(|x| part2(&mut map, x));
        println!("res: {}", map.iter().map(|(box_number,v)| part2_evaluate(&box_number, v)).sum::<u32>());
      }
    }
}


fn convert_to_int(input: &str) -> i32{
  (*input).chars().fold(0, |value, c| {
    let c_v = c as i32;
    let i: i32 = (value + c_v) * 17;
    //println!("{} i {} is {} {}for {}",value,  c_v, i, i % 256, c);
    return i  % 256
  })
}
fn part2_evaluate(bn: &i32, lenses: &Vec<(String, u32)>) -> u32 {
  lenses.iter().enumerate()
    .map(|(position, (_, value))| ((bn+1) as u32 ) * (position+1) as u32 * value)
    .sum()
}

fn part2(map: &mut HashMap<i32, Vec<(String, u32)>>,input: &String) {
  if input.contains('-') {
    let label = &input[0..input.len()-1];
    let i_label = convert_to_int(label);
    if let Some(v) = map.get_mut(&i_label) {
      v.retain(|x| x.0 != label );
    }
  } else {
    let lense: u32 = input.chars().last().unwrap().to_digit(10).unwrap();
    let lens_label = &input[0..input.len()-2];
    let box_label = convert_to_int(lens_label);
    if let Some(lenses) = map.get_mut(&box_label) {
      let mut label_changed = false;
      lenses.into_iter().for_each(|x| {if x.0 == lens_label { x.1 = lense; label_changed = true}}) ;
      if !label_changed {
        lenses.push((lens_label.to_owned(),lense));
      }
    } else {
      let mut lenses= Vec::new();
      lenses.push((lens_label.to_owned(),lense));
      map.insert(box_label, lenses);
    }
  }
}

fn part1(input: &String) -> i32 {
  convert_to_int(input)
}

fn split_into_map(lines: &mut io::Lines<io::BufReader<File>> ) -> Option<Vec<String>> {
  let mut inputs: Vec<String> = Vec::new();
  if let Some(line) = lines.next() {
    line.unwrap().clone().split(',').for_each(|x| inputs.push(x.to_owned()));
    Some(inputs)
  } else {
    None
  }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
