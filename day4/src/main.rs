use byte_set::ByteSet;

fn build_range_from_start_end(start: u8, end: u8) -> ByteSet {
    let mut set = ByteSet::new();

    for n in start..end {
        set.insert(n);
    }

    set
}

fn fully_contains(s: &str) -> Option<()> {
    let parts: Vec<&str> = s.split(['-', ',']).collect();

    let first_start = parts[0].parse::<u8>().unwrap();
    let first_end = parts[1].parse::<u8>().unwrap();
    let second_start = parts[2].parse::<u8>().unwrap();
    let second_end = parts[3].parse::<u8>().unwrap();

    let first = build_range_from_start_end(first_start, first_end);
    let second = build_range_from_start_end(second_start, second_end);

    (first.is_subset(&second) || second.is_subset(&first)).then_some(())
}


fn main() {
    let input_str = include_str!("../input");

    let part1: usize = input_str
        .lines()
        .filter_map(fully_contains)
        //.map(|s| {
        //    println!("{:?}", s);
        //    s
        //})
        .count();

    println!("Part1: {:?}", part1);
}