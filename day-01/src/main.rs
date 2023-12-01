fn main() {
  let input = include_str!("./input.txt");
  part1(input);
  part2(input);
}

fn part1(input: &str) {
  let mut lines = input.lines();
  let mut total = 0;

  while let Some(line) = lines.next() {
    let mut chars = line.chars();
    let mut first_digit = chars.next().unwrap();
    while !first_digit.is_digit(10) {
      first_digit = chars.next().unwrap();
    }

    chars = line.chars();
    let mut last_digit = chars.next_back().unwrap();
    while !last_digit.is_digit(10) {
      last_digit = chars.next_back().unwrap();
    }

    let mut num = String::new();
    num.push(first_digit);
    num.push(last_digit);

    let num: i32 = num.parse().unwrap();
    total += num;
  }
  println!("{}", total);
}

struct Digit {
  pub word: &'static str,
  pub value: char,
}

const DIGITS: [Digit; 9] = [
  Digit { word: "one", value: '1' },
  Digit { word: "two", value: '2' },
  Digit { word: "three", value: '3' },
  Digit { word: "four", value: '4' },
  Digit { word: "five", value: '5' },
  Digit { word: "six", value: '6' },
  Digit { word: "seven", value: '7' },
  Digit { word: "eight", value: '8' },
  Digit { word: "nine", value: '9' },
];

fn part2(input: &str) {
  let mut lines = input.lines();
  let mut total = 0;

  while let Some(line) = lines.next() {
    let mut num = String::new();

    let mut chars = line.chars().enumerate();
    while let Some((i, c)) = chars.next() {
      if c.is_digit(10) {
        num.push(c);
        break;
      }
      let mut found = false;
      for digit in DIGITS.iter() {
        let test: String = line.chars().skip(i).take(digit.word.len()).collect();
        if test == digit.word {
          num.push(digit.value);
          found = true;
          break;
        }
      }
      if found {
        break;
      }
    }

    let mut chars_rev = line.chars().rev().enumerate();
    while let Some((i, c)) = chars_rev.next() {
      if c.is_digit(10) {
        num.push(c);
        break;
      }
      let mut found = false;
      for digit in DIGITS.iter() {
        let test: String = line.chars().rev().skip(i).take(digit.word.len()).collect();
        let test_rev: String = test.chars().rev().collect();
        if test_rev == digit.word {
          num.push(digit.value);
          found = true;
          break;
        }
      }
      if found {
        break;
      }
    }
    
    let num: i32 = num.parse().unwrap();
    total += num;
  }
  println!("{}", total);
}
