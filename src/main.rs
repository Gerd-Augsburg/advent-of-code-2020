#![allow(dead_code)]
#[macro_use] extern crate lazy_static;
mod util;
mod day_01;
mod day_02;
mod day_03;

fn main() {
    let solution = day_03::solve_01();
    println!("You hit {} trees", solution);

    let solution = day_03::solve_02();
    println!("Product of all trees you hit: {}", solution);
}

fn day_02() {
    let n = day_02::solve_01();
    println!("{} entries are valid.", n);

    let n = day_02::solve_02();
    println!("In second Rule are {} valid.", n);
}

fn day_01() {
    let solution = day_01::solve();
    println!("Adding up to 2020 = {} + {}", solution.0, solution.1);
    println!("Product is : {}", solution.0 * solution.1);

    if let Some(second_solution) = day_01::solve2() {
        println!("Second Solution: {}", second_solution);
    } else {
        println!("No Solution Found.")
    }
}
