use std::collections::HashSet;

// O(N^2)
fn part1_n_squared(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == target {
                return Some(numbers[i] * numbers[j]);
            }
        }
    }
    return None;
}

// O(NlogN)
fn part1_n_logn(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let mut numbers = numbers.clone();
    numbers.sort();
    let mut i = 0;
    let mut j = numbers.len() - 1;
    while i < j {
        if numbers[i] + numbers[j] < target {
            i += 1;
        } else if numbers[i] + numbers[j] > target {
            j -= 1;
        } else {
            return Some(numbers[i] * numbers[j]);
        }
    }
    return None;
}

// O(N)
fn part1_n(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let mut hash = HashSet::new();
    for &x in numbers {
        let y = target - x;
        if hash.contains(&y) {
            return Some(x * y);
        } else {
            hash.insert(x);
        }
    }
    return None;
}

fn main() {
    let input = std::fs::read_to_string("src/day1/input.txt").unwrap();
    let numbers = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let result = part1_n_squared(&numbers, 2020).unwrap();
    println!("{}", result);

    let result = part1_n_logn(&numbers, 2020).unwrap();
    println!("{}", result);

    let result = part1_n(&numbers, 2020).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = std::fs::read_to_string("src/day1/sample.txt").unwrap();
        let numbers = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let result = part1_n_squared(&numbers, 2020);
        assert_eq!(result, Some(514579));

        let result = part1_n_logn(&numbers, 2020);
        assert_eq!(result, Some(514579));

        let result = part1_n(&numbers, 2020);
        assert_eq!(result, Some(514579));
    }
}
