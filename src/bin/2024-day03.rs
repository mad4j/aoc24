use regex::Regex;

const INPUT_STR: &str = include_str!("../../inputs/2024-day03.txt");

fn solve_part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let re = Regex::new(r"(?<mul>mul)\((?<a>\d+),(?<b>\d+)\)|(?<do>do)\(\)|(?<dont>don\'t)\(\)")
        .unwrap();

    let mut result = 0;
    let mut is_enabled = true;

    for c in re.captures_iter(input).into_iter().collect::<Vec<_>>() {
        //println!("{:?}", c);

        if c.name("mul").is_some() {
            if is_enabled {
                let a: u32 = c.name("a").unwrap().as_str().parse().unwrap();
                let b: u32 = c.name("b").unwrap().as_str().parse().unwrap();

                result += a * b;
            }
        }

        if c.name("do").is_some() {
            is_enabled = true;
        }

        if c.name("dont").is_some() {
            is_enabled = false;
        }
    }

    result
}

fn main() {
    println!("Part1 solution: {}", solve_part1(INPUT_STR));
    println!("Part2 solution: {}", solve_part2(INPUT_STR));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final() {
        assert_eq!(solve_part1(INPUT_STR), 173785482);
        assert_eq!(solve_part2(INPUT_STR), 83158140);
    }
}
