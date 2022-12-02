const WIN: usize = 6;
const LOSS: usize = 0;
const DRAW: usize = 3;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn get_score(&self) -> usize {
        match self {
            Self::Rock => 0,
            Self::Paper => 1,
            Self::Scissors => 2,
        }
    }

    fn from_char(s: char) -> Option<Self> {
        match s {
            'A' | 'X' => Some(Self::Rock),
            'B' | 'Y' => Some(Self::Paper),
            'C' | 'Z' => Some(Self::Scissors),
            _ => None,
        }

    }
}

struct Round {
    opponent: Hand,
    us: Hand,
}

impl Round {
    fn from_string(s: &str) -> Self {
        Round {
            opponent: Hand::from_char(s.as_bytes()[0] as char).unwrap(),
            us: Hand::from_char(s.as_bytes()[2] as char).unwrap()
        }
    }

    fn get_total_score(&self) -> usize {
        self.get_outcome_score() + self.get_hand_score()
    }

    fn get_outcome_score(&self) -> usize {
        match (3 + self.us.get_score() - self.opponent.get_score()) % 3{
            0 => DRAW,
            1 => WIN,
            2 => LOSS,
            _ => unreachable!()
        }
    }

    fn get_hand_score(&self) -> usize {
        self.us.get_score() + 1
    }
}

fn main() {
    let input_str = include_str!("../input");

    let part1: usize = input_str
        .lines()
        .map(Round::from_string)
        .map(|r| r.get_total_score())
        .sum();
    
    println!("Part1: {}", part1);
    assert!(part1 == 13221);
}
