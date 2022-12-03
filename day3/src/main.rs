use byte_set::ByteSet;
use itertools::Itertools;

fn main() {
    let input_str = include_str!("../input");

    let part1: u32 = input_str
        .lines()
        .filter_map(|l| {
            let (s1, s2) = l.split_at(l.len() / 2);

            let bm1 = str_to_set(s1);
            let bm2 = str_to_set(s2);

            bm1.intersection(bm2).first()
        })
        .map(|r| score(r) as u32)
        .sum()
        ;


    let part2: u32 = input_str
        .lines()
        .map(str_to_set)
        .tuples()
        .filter_map(|(rucksack1, rucksack2, rucksack3)| {
            rucksack1.intersection(rucksack2).intersection(rucksack3).first()
        })
        .map(|r| score(r) as u32)
        .sum()
        ;

    println!("Part1: {:?}\nPart2: {}", part1, part2);
    assert!(part1 == 8298);
    assert!(part2 == 2708);
}

fn str_to_set(s: &str) -> ByteSet {
    let mut set = ByteSet::new();
    for b in s.as_bytes() {
        set.insert(*b);
    }

    set
}

fn score(b: u8) -> u8 {
    match b {
        b'a'..=b'z' => (b - b'a') + 1,
        b'A'..=b'Z' => (b - b'A') + 1 + 26,
        _ => panic!("Only a-z and A-Z allowed")
    }
}
