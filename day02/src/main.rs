use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use anyhow::Result;
use std::env;

struct Game {
  index: u32,
  max_red: i32,
  max_green: i32,
  max_blue: i32
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
      println!("Missing argument for input file");
      exit(1)
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let game = parse(lines).unwrap();
    let sum:Result<u32> = game.iter().map(evaluate_game_part1).sum();
    println!("Part1: {}", sum.unwrap());

    let sum:Result<u32> = game.iter().map(evaluate_game_part2).sum();
    println!("Part2: {}", sum.unwrap());
  }
}

fn parse(input: io::BufReader<File>) -> Result<Vec<Game>> {
    input.lines().map(create_game).collect()
}

fn create_game(line: Result<String, std::io::Error>) -> Result<Game> {
    let binding = line.unwrap().to_string();
    let game_line: Vec<&str> = binding.split(":").collect();

    let numbers_str: String = game_line[0].chars().filter(|x| x.is_digit(10)).collect();
    let game_draw: Vec<&str> = game_line[1].split(";").collect();
    let mut game = Game {
      index: numbers_str.parse().unwrap(),
      max_red: 0,
      max_green: 0,
      max_blue: 0
    };
    // Print results.
    for v in game_draw {
      let draws: Vec<&str> = v.split(",").collect();
      for x in draws {
        let draw: Vec<&str> = x.split(" ").collect();
        match draw[2] {
          "red" => {
            let number = draw[1].parse::<i32>().unwrap();
            if game.max_red < number {
              game.max_red = number;
            }
          }
            ,
          "blue" => {
            let number = draw[1].parse::<i32>().unwrap();
            if game.max_blue < number {
              game.max_blue = number;
            }
          },
          "green" => {
            let number = draw[1].parse::<i32>().unwrap();
            if game.max_green < number {
              game.max_green = number;
            }
          },
          _ => {}
        }
      }
    }
    Ok(game)
}

fn evaluate_game_part1(game: &Game) -> Result<u32> {
  if game.max_red > 12 || game.max_green > 13 || game.max_blue > 14 {
    Ok(0)
  } else {
    Ok(game.index)
  }
}

fn evaluate_game_part2(game: &Game) -> Result<u32> {
  let power = game.max_red * game.max_blue *game.max_green;
  return Ok(power.try_into().unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
