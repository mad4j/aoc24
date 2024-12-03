const INPUT_STR: &str = include_str!("../../inputs/2024-day02.txt");

fn parse_values(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn is_strictly_ordered(vec: &Vec<u32>) -> bool {
    vec.windows(2).all(|v| v[0] < v[1]) || vec.windows(2).all(|v| v[0] > v[1])
    /*
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
    */
}

fn max_adjacent_difference(vec: &Vec<u32>) -> u32 {
    //use map_windows() when possible
    vec.windows(2)
        .map(|v| v[0].abs_diff(v[1]))
        .max()
        .unwrap_or_default()
    /*
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
    */
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

/*
fn check_single(vec: &Vec<u32>) -> Option<&Vec<u32>> {
    (is_strictly_ordered(&vec) && max_adjacent_difference(&vec) <= 3).then_some(vec)
}

fn check_multiple(vec: &Vec<u32>) -> Option<&Vec<u32>> {
    remove_each_element(vec).iter().any(|v| check_single(v)).then_some(vec)
    /*
    for v in remove_each_element(vec) {
        if is_strictly_ordered(&v) && max_adjacent_difference(&v) <= 3 {
            return true;
        }
    }

    return false;
    */
}
*/

trait Check {
    fn check_single(&self) -> bool;
    fn check_multiple(&self) -> bool;
}

impl Check for Vec<u32> {
    fn check_single(&self) -> bool {
        is_strictly_ordered(self) && max_adjacent_difference(self) <= 3
    }

    fn check_multiple(&self) -> bool {
        //remove_each_element(self).iter().any(|v| check_multiple(v)).then_some(self)
        remove_each_element(self)
            .iter()
            .any(|v| v.check_single())
    }
}

fn solve_part1(input: &str) -> u32 {
    // maybe filter_map() more concise?
    input
        .lines()
        .map(|l| parse_values(l))
        .filter(|v| v.check_single())
        .count() as u32
}

fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| parse_values(l))
        .filter(|v| v.check_multiple())
        .count() as u32
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
        assert_eq!(solve_part1(INPUT_STR), 236);
        assert_eq!(solve_part2(INPUT_STR), 308);
    }

    #[test]
    fn test01() {
        let v = vec![1, 3, 2, 4, 5];
        assert_eq!(v.check_multiple(), true);
    }

    #[test]
    fn test02() {
        let v = vec![8, 6, 4, 4, 1];
        assert_eq!(v.check_multiple(), true);
    }

    #[test]
    fn test03() {
        let v = vec![71, 74, 73, 76, 82];
        assert_eq!(v.check_multiple(), false);
    }
}
