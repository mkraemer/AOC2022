#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn get_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
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

#[derive(Debug)]
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
        match self.us {
            Hand::Rock => {
                match self.opponent {
                    Hand::Rock => 3,
                    Hand::Paper => 0,
                    Hand::Scissors => 6,
                }
            },
            Hand::Paper => {
                match self.opponent {
                    Hand::Rock => 6,
                    Hand::Paper => 3,
                    Hand::Scissors => 0,
                }
            },
            Hand::Scissors => {
                match self.opponent {
                    Hand::Rock => 0,
                    Hand::Paper => 6,
                    Hand::Scissors => 3,
                }
            },
        }
    }

    fn get_hand_score(&self) -> usize {
        self.us.get_score()
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
