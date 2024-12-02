const INPUT_STR: &str = include_str!("../../inputs/day02-24.txt");

fn parse_values(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn is_strictly_ordered(vec: &Vec<u32>) -> bool {
    if vec.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..vec.len() - 1 {
        if vec[i] >= vec[i + 1] {
            increasing = false;
        }
        if vec[i] <= vec[i + 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn max_adjacent_difference(vec: &Vec<u32>) -> u32 {
    if vec.len() < 2 {
        return 0;
    }

    let mut max_diff = 0;
    for i in 0..vec.len() - 1 {
        let diff = if vec[i] > vec[i + 1] {
            vec[i] - vec[i + 1]
        } else {
            vec[i + 1] - vec[i]
        };
        if diff > max_diff {
            max_diff = diff;
        }
    }

    max_diff
}

fn remove_each_element(vec: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    result.push(vec.clone());
    for i in 0..vec.len() {
        let mut new_vec = vec.clone();
        new_vec.remove(i);
        result.push(new_vec);
    }
    result
}

fn check_multiple(vec: &Vec<u32>) -> bool {

    for v in remove_each_element(vec) {

        if is_strictly_ordered(&v) && max_adjacent_difference(&v) <= 3 {
            return true;
        }
    }

    return false;

}

fn solve_part1(input: &str) -> i32 {
    let mut result = 0;

    for line in input.split('\n') {
        let v = parse_values(line);

        if is_strictly_ordered(&v) && max_adjacent_difference(&v) <= 3 {
            result += 1;
        }
    }

    result
}

fn solve_part2(input: &str) -> i32 {
    let mut result = 0;

    for line in input.split('\n') {
        let v = parse_values(line);

        if check_multiple(&v) {
            result += 1;
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
    fn test01() {

        let v = vec![1, 3, 2, 4, 5];
        assert_eq!(check_multiple(&v), true);
    }

    #[test]
    fn test02() {

        let v = vec![8, 6, 4, 4, 1];
        assert_eq!(check_multiple(&v), true);
    }

    #[test]
    fn test03() {

        let v = vec![71, 74, 73, 76, 82];
        assert_eq!(check_multiple(&v), false);
    }

}

