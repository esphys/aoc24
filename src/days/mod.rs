use std::str::Lines;

pub mod day_01;
pub mod day_02;

pub type Solution = fn(input: Lines<'_>);

pub fn get_registry() -> Vec<(&'static str, Solution)> {
    vec![
        ("day-01", day_01::solution as Solution),
        ("day-02", day_02::solution as Solution),
    ]
}
