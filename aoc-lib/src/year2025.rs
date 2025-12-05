mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub const DAYS: &[(&str, fn() -> anyhow::Result<()>)] = &[
    ("1", day01::solve),
    ("2", day02::solve),
    ("3", day03::solve),
    ("4", day04::solve),
    ("5", day05::solve),
];
