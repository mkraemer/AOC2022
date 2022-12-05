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

fn apply_move_each(stacks: &mut HashMap<i8, Vec<char>>, mov: &Move) {
    for _i in 0..mov.quantity {
        let crate_content = stacks.get_mut(&(mov.from)).unwrap().pop().unwrap();
        stacks.get_mut(&(mov.to)).unwrap().push(crate_content);
    }
}

fn apply_move_multiple(stacks: &mut HashMap<i8, Vec<char>>, mov: &Move) {
    let mut moved_crates: Vec<char> = Vec::new();

    for _i in 0..mov.quantity {
        let crate_content = stacks.get_mut(&(mov.from)).unwrap().pop().unwrap();
        moved_crates.push(crate_content);
    }

    for _i in 0..mov.quantity {
        stacks.get_mut(&(mov.to)).unwrap().push(moved_crates.pop().unwrap());
    }
}

fn get_top_crates(stacks: &HashMap<i8, Vec<char>>) -> String {
    let mut s = "".to_owned();
    for index in 1..stacks.len() + 1 {
        let crates = stacks.get(&(index as i8)).unwrap();
        s.push(crates[crates.len() - 1]);
    }

    s
}

fn main() {
    let (stack_desc, move_desc) = include_str!("../input").split_once("\n\n").unwrap();
    let moves = move_desc.lines().map(Move::from_string);

    let mut stacks_part_1 = build_initial_stacks(stack_desc);
    let mut stacks_part_2 = stacks_part_1.clone();
    for mov in moves {
        apply_move_each(&mut stacks_part_1, &mov);
        apply_move_multiple(&mut stacks_part_2, &mov);
    }

    let part1 = get_top_crates(&stacks_part_1);
    let part2 = get_top_crates(&stacks_part_2);

    println!("{}\n{}", part1, part2);
    assert!(part1 == "CFFHVVHNC");
    assert!(part2 == "FSZWBPTBG");
}
