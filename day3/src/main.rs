use byte_set::ByteSet;

#[derive(Debug)]
struct Rucksack {
    first_compartment: ByteSet,
    second_compartment: ByteSet,
}

impl Rucksack {
    fn from_string(s: &str) -> Self {
        let (first_compartment, second_compartment) = s.split_at(s.len() / 2);

        Rucksack {
            first_compartment: str_to_set(first_compartment),
            second_compartment: str_to_set(second_compartment),
        }
    }

    fn score(&self) -> u8 {
        let shared = self.first_compartment.intersection(self.second_compartment).first().unwrap();

        match shared {
            b'a'..=b'z' => (shared - b'a') + 1,
            b'A'..=b'Z' => (shared - b'A') + 1 + 26,
            _ => panic!("Only a-z and A-Z allowed")
        }
    }
}

fn main() {
    let input_str = include_str!("../input");

    let part1: u32 = input_str
        .lines()
        .map(Rucksack::from_string)
        .map(|r| r.score() as u32)
        .sum()
        ;

    println!("{:?}", part1);
    assert!(part1 == 8298);
}

fn str_to_set(s: &str) -> ByteSet {
    let mut set = ByteSet::new();
    for b in s.as_bytes() {
        set.insert(*b);
    }

    set
}
