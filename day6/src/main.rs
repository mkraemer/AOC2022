fn all_different(v: Vec<char>) -> bool {
    let mut check: Vec<char> = Vec::new();

    for c in &v {
        if check.contains(c) {
            return false
        };

        check.push(*c);
    }

    check.len() == v.len()
}

fn find_end_of_n_distinct_chars(v: &[char], n: usize) -> Option<usize> {
    for (i, window) in v.windows(n).enumerate() {
        if all_different(window.to_vec()) {
            return Some(i + n);
        }
    }

    None
}

fn main() {
    let input = include_str!("../input");

    let chars = input.chars().collect::<Vec<char>>();

    let part1 = find_end_of_n_distinct_chars(&chars, 4).unwrap();
    let part2 = find_end_of_n_distinct_chars(&chars, 14).unwrap();

    println!("Part1: {}\nPart2: {}", part1, part2);
    assert!(part1 == 1757);
    assert!(part2 == 2950);
}
