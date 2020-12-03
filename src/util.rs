use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn parse_lines<R, P>(filename: P) -> Vec<R>
    where P: AsRef<Path>, R: FromStr {
    if let Ok(lines) = read_lines(filename) {
        lines.filter_map(Result::ok)
            .map(|s| s.parse())
            .filter_map(Result::ok)
            .collect()
    } else {
        Vec::new()
    }
}

pub struct Triplets {
    max: usize,
    pos: [usize; 3]
}
impl Triplets {
    pub fn new(max: usize) -> Triplets {
        Triplets {
            max,
            pos: [0,1,2],
        }
    }
}

impl Iterator for Triplets {
    type Item = [usize; 3];
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.iter().any(|i| *i >= self.max) {
            return None;
        }
        let next = Some(self.pos);
        let [mut first, mut second, mut third] = self.pos;
        third += 1;
        if third >= self.max {
            second += 1;
            if second >= self.max - 1 {
                first += 1;
                second = first + 1;
            }
            third = second + 1;
        }
        self.pos = [first, second, third];
        next
    }
}