use std::collections::HashSet;

// O(N^2)
fn part1_n_squared(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let n = numbers.len();
    for i in 0..n {
        for j in i + 1..n {
            if numbers[i] + numbers[j] == target {
                return Some(numbers[i] * numbers[j]);
            }
        }
    }
    return None;
}

// O(NlogN)
fn part1_n_logn_by_binary_search(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let mut numbers = numbers.clone();
    numbers.sort();
    let n = numbers.len();
    for i in 0..n {
        let x = target - numbers[i];
        if let Ok(j) = numbers.binary_search(&x) {
            return Some(numbers[i] * numbers[j]);
        }
    }
    return None;
}

// O(NlogN)
fn part1_n_logn_by_two_pointers(numbers: &Vec<i32>, target: i32) -> Option<i32> {
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

// O(n^3)
fn part2_n_cubed(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let n = numbers.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if numbers[i] + numbers[j] + numbers[k] == target {
                    return Some(numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }
    return None;
}

// O(N^2 logN)
fn part2_n_squared_logn(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let mut numbers = numbers.clone();
    numbers.sort();
    let n = numbers.len();
    for i in 0..n {
        for j in i + 1..n {
            let x = target - numbers[i] - numbers[j];
            if let Ok(k) = numbers.binary_search(&x) {
                return Some(numbers[i] * numbers[j] * numbers[k]);
            }
        }
    }
    return None;
}

// O(N^2)
fn part2_n_squared_by_hash_table(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let mut hash = HashSet::new();
    for &x in numbers {
        hash.insert(x);
    }
    let n = numbers.len();
    for i in 0..n {
        for j in i + 1..n {
            let x = target - numbers[i] - numbers[j];
            if hash.contains(&x) {
                return Some(numbers[i] * numbers[j] * x);
            }
        }
    }
    return None;
}

// O(N^2)
fn part2_n_squared_by_two_pointers(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let mut numbers = numbers.clone();
    numbers.sort();
    let n = numbers.len();
    for i in 0..n {
        let x = target - numbers[i];
        let mut l = i + 1;
        let mut r = n - 1;
        while l < r {
            if numbers[l] + numbers[r] < x {
                l += 1;
            } else if numbers[l] + numbers[r] > x {
                r -= 1;
            } else {
                return Some(numbers[i] * numbers[l] * numbers[r]);
            }
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

    // part 1
    println!("Part 1");
    let result = part1_n_squared(&numbers, 2020).unwrap();
    println!("  O(N^2):    {}", result);

    let result = part1_n_logn_by_two_pointers(&numbers, 2020).unwrap();
    println!("  O(NlogN):  {}  (by two pointers)", result);

    let result = part1_n_logn_by_binary_search(&numbers, 2020).unwrap();
    println!("  O(NlogN):  {}  (by binary search)", result);

    let result = part1_n(&numbers, 2020).unwrap();
    println!("  O(N):      {}", result);

    // part 2
    println!("Part 2");
    let result = part2_n_cubed(&numbers, 2020).unwrap();
    println!("  O(N^3):       {}", result);

    let result = part2_n_squared_logn(&numbers, 2020).unwrap();
    println!("  O(N^2 logN):  {}", result);

    let result = part2_n_squared_by_hash_table(&numbers, 2020).unwrap();
    println!("  O(N^2):       {}  (by hash table)", result);

    let result = part2_n_squared_by_two_pointers(&numbers, 2020).unwrap();
    println!("  O(N^2):       {}  (by two pointers)", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let input = std::fs::read_to_string("src/day1/sample.txt").unwrap();
        let numbers = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let result = part1_n_squared(&numbers, 2020);
        assert_eq!(result, Some(514579));

        let result = part1_n_logn_by_two_pointers(&numbers, 2020);
        assert_eq!(result, Some(514579));

        let result = part1_n_logn_by_binary_search(&numbers, 2020);
        assert_eq!(result, Some(514579));

        let result = part1_n(&numbers, 2020);
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn it_works_part2() {
        let input = std::fs::read_to_string("src/day1/sample.txt").unwrap();
        let numbers = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let result = part2_n_cubed(&numbers, 2020);
        assert_eq!(result, Some(241861950));

        let result = part2_n_squared_logn(&numbers, 2020);
        assert_eq!(result, Some(241861950));

        let result = part2_n_squared_by_hash_table(&numbers, 2020);
        assert_eq!(result, Some(241861950));

        let result = part2_n_squared_by_two_pointers(&numbers, 2020);
        assert_eq!(result, Some(241861950));
    }
}
