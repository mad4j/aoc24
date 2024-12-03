use regex::Regex;

const INPUT_STR: &str = include_str!("../../inputs/2024-day03.txt");

fn solve_part1(input: &str) -> u32 {

    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let mut result = 0;

    for c in re.captures_iter(input) {

        let a: u32 = c.name("a").unwrap().as_str().parse().unwrap();
        let b: u32 = c.name("b").unwrap().as_str().parse().unwrap();

        result += a*b;
    }
    
    result
}


fn solve_part2(input: &str) -> u32 {
    0
}

fn main() {

    println!("Part1 solution: {}", solve_part1(INPUT_STR));
    println!("Part2 solution: {}", solve_part2(INPUT_STR));
}