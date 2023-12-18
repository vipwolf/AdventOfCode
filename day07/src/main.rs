use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use std::env;

#[derive(Ord, Eq)]
struct Deck {
  bid: u64,
  cards: Vec<(Card, usize)>,
  deck_type: DeckTypes,
  deck_str: String
}

//A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
#[derive(Clone, Copy)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
#[derive(Debug)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    NINE,
    EIGHT,
    SEVEN,
    SIX,
    FIVE,
    FOUR,
    THREE,
    TWO,
    JOKER
}

impl Card {
  pub fn from(c: char) -> Result<Card, ()> {
      match c {
          'A' => Ok(Card::A),
          'K' => Ok(Card::K),
          'Q' => Ok(Card::Q),
          'J' => Ok(Card::J),
          'T' => Ok(Card::T),
          '9' => Ok(Card::NINE),
          '8' => Ok(Card::EIGHT),
          '7' => Ok(Card::SEVEN),
          '6' => Ok(Card::SIX),
          '5' => Ok(Card::FIVE),
          '4' => Ok(Card::FOUR),
          '3' => Ok(Card::THREE),
          '2' => Ok(Card::TWO),
          '1' => Ok(Card::JOKER),
          _ => Err(())
      }
  }
}

impl PartialOrd for Deck {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.deck_type == other.deck_type {
      let mut c_char = self.deck_str.chars();
      let mut other_c_char = other.deck_str.chars();
      while let Some(c) = c_char.next(){
        if let Some(other_c) = other_c_char.next(){
          if c != other_c { return Some( Card::from(c).cmp(&Card::from(other_c))) }
        }
      }
      Some(Ordering::Equal)
    } else {
      self.deck_type.partial_cmp(&other.deck_type)
    }
  }
}

impl PartialEq for Deck {
  fn eq(&self, other: &Self) -> bool {
      if self.deck_type == other.deck_type {
        self.deck_str == other.deck_str
      } else {
        false
      }
  }
}

#[derive(Debug)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum DeckTypes{
  FiveOfAKind,
  FourOfAKind,
  FullHouse,
  ThreeOfAKind,
  TwoPair,
  OnePair,
  HighCard
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      println!("Missing argument for input file");
      exit(1)
    }

    if let Ok(mut lines) = read_lines(&args[1]) {
      let mut decks = create_cards(&mut lines);
      decks.iter_mut().for_each(evaluate_deck);
      decks.sort_unstable();
      decks.reverse();
      let mut line_number = 0;

      println!("res: {}", decks.iter().map(|line| { line_number += 1; line_number * line.bid }).sum::<u64>());
      decks.iter_mut().for_each(evaluate_deck2);
      decks.sort_unstable();
      decks.reverse();
      line_number = 0;
      println!("res: {}", decks.iter().map(|line| { line_number += 1; line_number * line.bid }).sum::<u64>());
    }
}

fn evaluate_deck(deck: &mut Deck) {
  match deck.cards.len(){
    1 => deck.deck_type=DeckTypes::FiveOfAKind,
    2 => {if deck.cards.iter().any(|x| x.1 == 4 ) {deck.deck_type=DeckTypes::FourOfAKind} else {deck.deck_type=DeckTypes::FullHouse}}
    3 => {if deck.cards.iter().any(|x| x.1 == 3 ) {deck.deck_type=DeckTypes::ThreeOfAKind} else {deck.deck_type=DeckTypes::TwoPair}}
    4 => deck.deck_type=DeckTypes::OnePair,
    _ => {}
  }
}

fn evaluate_deck2(deck: &mut Deck) {
  match deck.cards.len(){
    1 => deck.deck_type=DeckTypes::FiveOfAKind,
    2 => {
      if deck.deck_str.contains("J") {
        deck.deck_type=DeckTypes::FiveOfAKind
      } else if deck.cards.iter().any(|x| x.1 == 4 ) {
        deck.deck_type=DeckTypes::FourOfAKind
      } else {
        deck.deck_type=DeckTypes::FullHouse
      }
    }
    3 => {
      if deck.cards.iter().any(|x| x.1 == 3 ) {
        if deck.deck_str.contains("J") {
          deck.deck_type=DeckTypes::FourOfAKind
        } else {
          deck.deck_type=DeckTypes::ThreeOfAKind
        }
      } else {
        if deck.deck_str.contains("J") {
          if deck.cards.iter().any(|x| x.1 == 2 && x.0 == Card::J ) {
            deck.deck_type=DeckTypes::FourOfAKind
          } else {
            deck.deck_type=DeckTypes::FullHouse
          }

        } else {
          deck.deck_type=DeckTypes::TwoPair
        }
      }
    }
    4 =>{
      if deck.deck_str.contains("J") {
        deck.deck_type=DeckTypes::ThreeOfAKind
      } else {
        deck.deck_type=DeckTypes::OnePair
      }
    },
    _ => {if deck.deck_str.contains("J") {deck.deck_type=DeckTypes::OnePair}}
  }
  deck.deck_str = deck.deck_str.replace("J", "1");
}

fn create_cards<'a>(lines: &mut io::Lines<io::BufReader<File>>) -> Vec<Deck> {
  let mut decks:Vec<Deck> = Vec::new();
  for line in lines {
    let deck_str:Vec<String> = line.unwrap().split_ascii_whitespace().map(|x| x.to_owned()).collect();
    let mut deck = Deck{bid: deck_str.get(1).unwrap().parse().unwrap(), cards: Vec::new(), deck_type: DeckTypes::HighCard, deck_str: deck_str.get(0).unwrap().clone()};
    for char in deck_str[0].chars(){
      let card = deck.cards.iter_mut().find(|x| x.0 == Card::from(char).unwrap());
      match card {
        Some(card_tuple) => card_tuple.1 += 1,
        None => deck.cards.push((Card::from(char).unwrap(),1))
      }
    };
    decks.push(deck);
  }
  return decks;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
