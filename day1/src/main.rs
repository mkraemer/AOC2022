use itertools::Itertools;

struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    fn from_string (s: &str) -> Self {
        Elf {calories: s.lines().map(|l| l.parse::<i32>().unwrap()).collect()}
    }
}

fn get_calories_of_nth_most_packed_elves (elves: &[Elf], num: usize) -> i32 {
    elves
        .iter()
        .map(|elf| elf.calories.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(num)
        .sum()
}

fn main() {
    let input_str = include_str!("../input");

    let elves: Vec<Elf> = input_str.split("\n\n")
        .map(Elf::from_string)
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        get_calories_of_nth_most_packed_elves(&elves, 1),
        get_calories_of_nth_most_packed_elves(&elves, 3)
        );
}
