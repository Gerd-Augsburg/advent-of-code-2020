use regex::Regex;
use std::str::FromStr;
use std::ops::RangeInclusive;
use crate::util::parse_lines;

pub fn solve_01() -> usize {
    let lines: Vec<DatabaseEntry<Numbered>> = parse_lines("./res/day_02/data.txt");
    lines.iter().filter(|d| d.is_valid()).count()
}

pub fn solve_02() -> usize {
    let lines: Vec<DatabaseEntry<Position>> = parse_lines("./res/day_02/data.txt");
    lines.iter().filter(|d| d.is_valid()).count()
}

struct DatabaseEntry<T> where T: Rule + Sized {
    rule: T,
    password: String,
}

impl<T> DatabaseEntry<T> where T: Rule + Sized {
    fn is_valid(&self) -> bool {
        self.rule.applies_to(self.password.as_str())
    }
}

trait Rule {
    fn applies_to(&self, s: &str) -> bool;
}

struct Numbered {
    range: RangeInclusive<usize>,
    character: char,
}

struct Position {
    first: usize,
    second: usize,
    character: char,
}

impl Rule for Numbered {
    fn applies_to(&self, s: &str) -> bool {
        let n = s.chars().filter(|c| c == &self.character).count();
        self.range.contains(&n)
    }
}

impl Rule for Position {
    fn applies_to(&self, s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        [self.first, self.second].iter()
            .flat_map(|&i| chars.get(i - 1))
            .map(|&c| c == self.character)
            .fold(false, |a, b| a ^ b)
    }
}

impl FromStr for DatabaseEntry<Numbered> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<from>\d+)-(?P<to>\d+)\s+(?P<letter>[[:alpha:]]):\s+(?P<password>[[:alpha:]]+)").unwrap();
        }
        if let Some(cap) = RE.captures(s) {
            let from = cap["from"].parse().unwrap();
            let to = cap["to"].parse().unwrap();
            let letter = cap["letter"].chars().next().unwrap();
            let rule = Numbered {
                range: from..=to,
                character: letter,
            };
            let password = String::from(&cap["password"]);
            return Ok(DatabaseEntry {
                rule,
                password,
            });
        } else {
            Err("Invalid Format!")
        }
    }
}

impl FromStr for DatabaseEntry<Position> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<from>\d+)-(?P<to>\d+)\s+(?P<letter>[[:alpha:]]):\s+(?P<password>[[:alpha:]]+)").unwrap();
        }
        if let Some(cap) = RE.captures(s) {
            let first = cap["from"].parse().unwrap();
            let second = cap["to"].parse().unwrap();
            let letter = cap["letter"].chars().next().unwrap();
            let rule = Position {
                first,
                second,
                character: letter,
            };
            let password = String::from(&cap["password"]);
            return Ok(DatabaseEntry {
                rule,
                password,
            });
        } else {
            Err("Invalid Format!")
        }
    }
}
