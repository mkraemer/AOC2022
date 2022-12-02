use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Hand {
    fn get_beating_hand(&self, outcome: Outcome) -> Self {
        Hand::from_int((self.int_value() + outcome.int_value()) % 3).unwrap()
    }

    fn from_char(s: char) -> Self {
        match s {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum Outcome {
    Draw = 0,
    Win = 1,
    Lose = 2,
}

impl Outcome {
    fn from_char(s: char) -> Self {
        match s {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!()
        }
    }

    fn get_score(&self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Hand,
    us: Hand,
}

impl Round {
    fn from_opponend_and_outcome(s: &str) -> Self {
        let opponent = Hand::from_char(s.as_bytes()[0] as char);
        let outcome = Outcome::from_char(s.as_bytes()[2] as char);

        let chosen_hand = opponent.get_beating_hand(outcome);

        Round {
            opponent,
            us: chosen_hand,
        }
    }

    fn from_opponent_and_us(s: &str) -> Self {
        Round {
            opponent: Hand::from_char(s.as_bytes()[0] as char),
            us: Hand::from_char(s.as_bytes()[2] as char)
        }
    }

    fn get_total_score(&self) -> usize {
        self.outcome().get_score() + self.get_hand_score()
    }

    fn outcome(&self) -> Outcome {
        Outcome::from_int((3 + self.us as usize - self.opponent as usize) % 3).unwrap()
    }

    fn get_hand_score(&self) -> usize {
        self.us as usize + 1
    }
}

fn main() {
    let input_str = include_str!("../input");

    let part1: usize = input_str
        .lines()
        .map(Round::from_opponent_and_us)
        .map(|r| r.get_total_score())
        .sum();

    let part2: usize = input_str
        .lines()
        .map(Round::from_opponend_and_outcome)
        .map(|r| r.get_total_score())
        .sum();
    
    println!("Part1: {}\nPart2: {}", part1, part2);
    assert!(part1 == 13221);
    assert!(part2 == 13131);
}
