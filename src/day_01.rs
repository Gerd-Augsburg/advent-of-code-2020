use std::collections::HashMap;

use crate::util::{read_lines, Triplets};

pub fn solve() -> (i32, i32) {
    let mut int_map = HashMap::new();
    if let Ok(lines) = read_lines("./res/day_01/input") {
        for line in lines {
            if let Ok(Ok(int)) = line.map(|s| s.parse()) {
                if let Some(value) = int_map.get(&int) {
                    return (int, *value);
                } else {
                    int_map.insert(2020 - int, int);
                }
            }
        }
    }
    (0, 0)
}

pub fn solve2() -> Option<i32> {
    let input: Vec<i32> = crate::util::parse_lines("./res/day_01/input");

    for [first, second, third] in Triplets::new(input.len()) {
        if input[first] + input[second] + input[third] == 2020 {
            return Some(input[first] * input[second] * input[third]);
        }
    }
    None
}

