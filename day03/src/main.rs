use std::ops::Range;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use std::env;

struct EngineSchematic {
  line_number: usize,
  part_number: Vec<(Range<usize>,u32)>,
  symbols: Vec<(usize, char)>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      println!("Missing argument for input file");
      exit(1)
    }

    if let Ok(lines) = read_lines(&args[1]) {
      let mut line_number = 0;
      let schematic: Vec<EngineSchematic> = lines.into_iter()
        .map(|line| { line_number += 1; create_engine(&line.unwrap(), line_number) })
        .collect();
      println!("res: {}", part1(&schematic));
      println!("res: {}", part2(&schematic));
    }
}
fn part2(schematic: &Vec<EngineSchematic>) -> u32 {
    let mut sum = 0;
    for v in schematic {
      match v.line_number-1 {
        0 =>
          { sum += evaluate_engine2(v, &schematic[1], &v)
                    + evaluate_engine2(v, &v, &v )
                    + evaluate_engine2(v, &schematic[1], &schematic[1] )
          },
        x if(x==schematic.len()-1) =>
          {
            sum += evaluate_engine2(&v, &schematic[schematic.len()-2], &v )
                  + evaluate_engine2(&v, &schematic[schematic.len()-2], &schematic[schematic.len()-2] )
                  + evaluate_engine2(v, &v, &v)
          },
        _ =>
          { sum += evaluate_engine2(&v, &v, &schematic[v.line_number-2])
                    + evaluate_engine2(v, &v, &v)
                    + evaluate_engine2(&v, &v, &schematic[v.line_number])
                    + evaluate_engine2(&v, &schematic[v.line_number-2], &schematic[v.line_number])
                    + evaluate_engine2(&v, &schematic[v.line_number-2], &schematic[v.line_number-2])
                    + evaluate_engine2(&v, &schematic[v.line_number], &schematic[v.line_number])
          }
      }
    }
    sum
}

fn part1(schematic: &Vec<EngineSchematic>) -> u32 {
    let mut sum = 0;
    for v in schematic {
      match v.line_number-1 {
        0 =>
          {
            sum += evaluate_engine(v, &schematic[1])
                 + evaluate_engine(v, &v)
          },
        x if(x==schematic.len()-1) =>
          {
            sum += evaluate_engine(&v, &schematic[schematic.len()-2])
                  +evaluate_engine(v, &v)
          },
        _ =>
          {
            sum += evaluate_engine(&v, &schematic[v.line_number-2])
                + evaluate_engine(&v, &schematic[v.line_number])
                + evaluate_engine(v, &v)
          }
      }
    }
    sum
}

fn create_engine<'a>(line: &String, line_number:usize) -> EngineSchematic {
    let mut engine_line: EngineSchematic = EngineSchematic {
        line_number: line_number,
        part_number: Vec::new(),
        symbols: Vec::new(),
    };
    let mut number_str: String = String::from("");
    let mut char_start: usize = 0;
    let mut char_end: usize = 0;
    for (i, c) in line.chars().enumerate() {
      match c {
        '.' => {
            if number_str.len()>0 {
                let range = Range{start: char_start-1, end: char_end+1};
                engine_line.part_number.push(
                    (
                        range,
                        number_str.parse().unwrap()
                    )
                );
                number_str=String::from("");
                char_start = 0;
                char_end = 0 ;
            }
        },
        c if c.is_numeric() => {
            number_str.push(c);
            if char_start == 0 {char_start = i+1}
            char_end = i+1;
        },
        _ => {
            if number_str.len()>0 {
                engine_line.part_number.push(
                    (
                        Range{start: char_start-1, end: char_end+1},
                        number_str.parse().unwrap()
                    )
                );
                number_str=String::from("");
                char_start = 0;
                char_end = 0 ;
            }
            engine_line.symbols.push((i+1, c))
        },
      }
    }
    if number_str.len()>0 {
        engine_line.part_number.push(
            (
                Range{start: char_start-1, end: char_end+1},
                number_str.parse().unwrap()
            )
        );
    }
    return engine_line;
}

fn evaluate_engine(engine_line: &EngineSchematic, adjacent_line: &EngineSchematic) -> u32 {
    let mut sum:u32 = 0;
    for engine in &engine_line.part_number {
        for symbol in &adjacent_line.symbols {
            if symbol.0 >= engine.0.start   && symbol.0 <= engine.0.end {
                sum += engine.1
            }
        }
    }

    return sum;
}

fn evaluate_engine2(symbol_line: &EngineSchematic, adjacent_line1: &EngineSchematic, adjacent_line2: &EngineSchematic,) -> u32 {
    let mut sum:u32 = 0;
    for symbol in &symbol_line.symbols {
        if symbol.1 == '*' {
            for engine1 in &adjacent_line1.part_number {
                if symbol.0 >= engine1.0.start   && symbol.0 <= engine1.0.end  {
                    for engine2 in &adjacent_line2.part_number {
                        if symbol.0 >= engine2.0.start && symbol.0 <= engine2.0.end
                           && (adjacent_line1.line_number != adjacent_line2.line_number || engine1.0.start < engine2.0.start)
                        {
                          sum += engine1.1 * engine2.1
                        }
                    }
                }
            }

        }
    }

    return sum;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
