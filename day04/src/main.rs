use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use std::env;

struct Card {
  number: usize,
  count: u32,
  winning_numbers: Vec<u32>,
  my_numbers: Vec<u32>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      println!("Missing argument for input file");
      exit(1)
    }

    if let Ok(lines) = read_lines(&args[1]) {

      let mut schematic: Vec<Card> = lines.into_iter()
        .map(create_cards)
        .collect();
      println!("res: {}", schematic.iter().map(part1).sum::<u32>());
      println!("res: {}", part2(&mut schematic));
    }
}

fn create_cards<'a>(line: Result<String, std::io::Error>) -> Card {
  let binding = line.unwrap().to_string();
  let full_game_line: Vec<_> = binding.split(':').collect();
  let game_number: String = full_game_line[0].chars().filter(|x| x.is_digit(10)).collect();

  let mut engine_line: Card = Card {
    number: game_number.parse::<usize>().unwrap(),
    count:1,
    winning_numbers: Vec::new(),
    my_numbers: Vec::new(),
  };

  let draw: Vec<_> = full_game_line[1].split('|').collect();
  engine_line.winning_numbers = draw[0].split(' ').filter(|x| !x.is_empty()).map(|x| { let number:u32=x.parse().unwrap_or_default(); return number}).collect();
  engine_line.winning_numbers.sort();
  engine_line.my_numbers = draw[1].split(' ').filter(|x| !x.is_empty()).map(|x| { let number:u32=x.parse().unwrap_or_default(); return number}).collect();
  engine_line.my_numbers.sort();

  return engine_line;
}

fn part1(card: &Card) -> u32 {
  let sum;
  let numbers: u32 = card.my_numbers
    .iter()
    .map(|x| if card.winning_numbers.iter().any(|f| f == x) {1} else {0})
    .sum();
  match numbers {
    x if(x>1) => { sum = 2u32.pow(x-1)},
    _ =>{ sum = numbers;}
  }
  return sum;
}

fn part2( card_stack:  &mut Vec<Card>) -> u32 {
  for card_idx in 0 .. card_stack.len() {
    let card = &card_stack[card_idx];
    let numbers:usize = card.my_numbers
                  .iter()
                  .map(|x| if card.winning_numbers.iter().any(|f| f == x) {1} else {0})
                  .sum();

    let card_count = card.count;
    for x  in card.number..(card.number+numbers).min(card_stack.len()) {
      let card = card_stack.get_mut(x).unwrap();
      card.count += card_count;
    }
  }

  card_stack.iter().map(|card| card.count).sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
