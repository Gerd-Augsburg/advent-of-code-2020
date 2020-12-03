use crate::util::parse_lines;
use crate::day_03::Field::{Open, Tree};

pub fn solve_01() -> i32 {
    let input: Vec<String> = parse_lines("./res/day_03/input.txt");
    let chars: Vec<Vec<char>> = input.iter().skip(1).map(|s| s.chars().collect()).collect();
    let mut counter = 0;
    let mut slope = 0;
    for line in chars {
        slope = (slope + 3) % line.len();
        if line[slope] == '#' {
            counter += 1;
        }
    }
    counter
}

pub fn solve_02() -> usize {
    let input: Vec<String> = parse_lines("./res/day_03/input.txt");
    let hill = Hill::from(&input);
    let slopes = [
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2)
    ];
    slopes.iter().map(|slope| {
        let mut down = 0;
        let mut right = 0;
        let mut hits = 0;
        while let Some(field) = hill.get(down, right) {
            if *field == Field::Tree {
                hits += 1;
            }
            down += slope.down;
            right += slope.right;
        }
        hits
    }).fold(1, |a, b| a * b)
}

#[derive(Eq, PartialEq, Debug)]
enum Field {
    Tree,
    Open,
}

impl Field {
    pub fn from(c: &char) -> Field {
        match *c {
            '.' => Open,
            '#' => Tree,
            _ => panic!("Invalid Format")
        }
    }
}

type Matrix<T> = Vec<Vec<T>>;

struct Hill {
    terrain: Matrix<Field>,
}

impl Hill {
    pub fn from(data: &Vec<String>) -> Hill {
        let terrain = data.iter()
            .map(|s| s.chars().map(|c| Field::from(&c)).collect())
            .collect();
        Hill {
            terrain
        }
    }

    pub fn get(&self, down: usize, right: usize) -> Option<&Field> {
        let row = self.terrain.get(down)?;
        row.get((right) % row.len())
    }
}

struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    pub fn new(right: usize, down: usize) -> Slope {
        Slope {
            right,
            down,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn field_should_be_constructed_by_char() {
        assert_eq!(Field::from(&'.'), Field::Open);
        assert_eq!(Field::from(&'#'), Field::Tree);
    }

    #[test]
    fn should_create_hill_from_vec_of_strings() {
        let data = vec![".#.#.#".into()];
        let hill = Hill::from(&data);
        assert_eq!(hill.terrain, vec![vec![Open, Tree, Open, Tree, Open, Tree]]);
    }
}