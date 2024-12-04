fn check_safe(numbers: &Vec<i64>) -> bool {
    let is_asc = numbers[0] < numbers[numbers.len() - 1];

    for i in 0..(numbers.len() - 1) {
        let diff = (numbers[i] - numbers[i + 1]).abs();

        if (diff < 1 || diff > 3)
            || (numbers[i] < numbers[i + 1] && !is_asc)
            || (numbers[i] > numbers[i + 1] && is_asc)
        {
            return false;
        }
    }

    return true;
}

pub fn part_a(input: &str) -> i64 {
    let mut safe_count: i64 = 0;
    for line in input.trim().split('\n') {
        if line.len() < 2 {
            safe_count += 1;
            continue;
        }

        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        if check_safe(&numbers) {
            safe_count += 1;
        }
    }

    safe_count
}

pub fn part_b(input: &str) -> i64 {
    let mut safe_count: i64 = 0;
    for line in input.trim().split('\n') {
        if line.len() < 2 {
            safe_count += 1;
            continue;
        }

        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        if check_safe(&numbers) {
            safe_count += 1;
        } else {
            for i in 0..numbers.len() {
                let mut num_copy: Vec<i64> = numbers.clone();
                num_copy.remove(i);
                if check_safe(&num_copy) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    safe_count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\n"
            ),
            2
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 383);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 436);
    }
}
