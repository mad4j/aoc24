#![feature(iter_intersperse)]

use regex::Regex;

const INPUT_STR: &str = include_str!("../../inputs/2024-day04_sample1.txt");

fn build_regex(label: &str, offset: usize) -> Regex {
    let s = format!(
        "(?s){}",
        label
            .chars()
            .map(|x| format!("{x}"))
            .intersperse(format!(".{{{offset}}}"))
            .collect::<String>()
    );
    Regex::new(s.as_str()).unwrap()
}

fn solve_part1(input: &str) -> usize {
    let input = input.replace("\r", "");
    let grid_width = input.find("\n").unwrap_or_default();

    let regs = [
        //build_regex("XMAS", 0),
        //build_regex("SAMX", 0),
        //build_regex("XMAS", grid_width),
        //build_regex("SAMX", grid_width),
        //build_regex("XMAS", grid_width + 1),
        //build_regex("SAMX", grid_width + 1),
        //build_regex("XMAS", grid_width - 1),
        build_regex("SAMX", grid_width - 1),
    ];

    println!("{regs:?}");

    //let result = regs.into_iter().fold(0, |s, r| {
    //    s + r.find_iter(&input).inspect(|x| println!("{x:?}")).count()
    //});

    let result = regs.into_iter().fold(0, |s, r| {
        s + (0..input.len())
            .filter_map(|i| r.is_match_at(&input, i).then_some(i))
            .count()
    });

    result
}

fn solve_part2(input: &str) -> usize {
    0
}

fn main() {
    println!("Part1 solution: {}", solve_part1(INPUT_STR));
    println!("Part2 solution: {}", solve_part2(INPUT_STR));
}
