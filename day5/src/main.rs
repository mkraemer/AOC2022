use std::collections::HashMap;

fn build_initial_stacks(s: &str) -> HashMap<i8, Vec<char>> {
    let mut lines: Vec<&str> = s.lines().collect();

    let indices = lines
        .pop().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i8>().unwrap());

    let mut stacks: HashMap<i8, Vec<char>> = HashMap::new();
    for index in indices {
        stacks.insert(index, Vec::new());
    }

    for line in lines.iter().rev() {
        for index in 1..stacks.len() + 1 {
            let position = ((index - 1) * 4) + 1;
            let crate_content = line.chars().nth(position).unwrap();

            match crate_content {
                'A'..='Z' => {
                    let stack = stacks.get_mut(&(index as i8)).unwrap();
                    stack.push(crate_content);
                },
                ' ' => (),
                _ => unreachable!(),
            }


        }
    }

    stacks
}

#[derive(Debug)]
struct Move {
    from: i8,
    to: i8,
    quantity: i32,
}

impl Move {
    fn from_string(s: &str) -> Self {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        Move {
            quantity: tokens[1].parse::<i32>().unwrap(),
            from: tokens[3].parse::<i8>().unwrap(),
            to: tokens[5].parse::<i8>().unwrap(),
        }
    }
}

fn apply_move(stacks: &mut HashMap<i8, Vec<char>>, mov: Move) {
    for _i in 0..mov.quantity {
        
        let crate_content = stacks.get_mut(&(mov.from)).unwrap().pop().unwrap();
        stacks.get_mut(&(mov.to)).unwrap().push(crate_content);
    }
}

fn main() {
    let input_str = include_str!("../input");

    if let Some((stack_desc, move_desc)) = input_str.split_once("\n\n") {
        let mut stacks = build_initial_stacks(stack_desc);

        let moves = move_desc.lines().map(Move::from_string);

        for mov in moves {
            apply_move(&mut stacks, mov);
        }

        let mut part1 = "".to_owned();
        for index in 1..stacks.len() + 1 {
            let crates = stacks.get(&(index as i8)).unwrap();
            part1.push(crates[crates.len() - 1]);

        }
        
        println!("{}", part1);
        assert!(part1 == "CFFHVVHNC");
    }
}
