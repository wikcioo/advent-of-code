use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut after = Vec::new();
        for num in &numbers {
            if *num == 0 {
                after.push(1);
                continue;
            }
            let num_str = num.to_string();
            if num_str.len() % 2 == 0 {
                let (left, right) = num_str.split_at(num_str.len() / 2);
                after.push(left.parse::<u64>().unwrap());
                after.push(right.parse::<u64>().unwrap());
                continue;
            }
            after.push(*num * 2024);
        }
        numbers = after;
    }

    numbers.len() as i64
}

fn blink(x: u64, n: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if n == 0 {
        return 1;
    }

    if !cache.contains_key(&(x, n)) {
        let mut res = 0;
        if x == 0 {
            res = blink(1, n - 1, cache);
        } else if x.to_string().len() % 2 == 0 {
            let x_str = x.to_string();
            let (l, r) = x_str.split_at(x.to_string().len() / 2);
            res += blink(l.parse::<u64>().unwrap(), n - 1, cache);
            res += blink(r.parse::<u64>().unwrap(), n - 1, cache);
        } else {
            res = blink(x * 2024, n - 1, cache);
        }
        cache.insert((x, n), res);
    }

    return *cache.get(&(x, n)).unwrap();
}

pub fn part_b(input: &str) -> i64 {
    let numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut cache = HashMap::<(u64, u64), u64>::new();
    let mut ans = 0;
    for n in &numbers {
        ans += blink(*n, 75, &mut cache);
    }

    ans as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("125 17\n"), 55312);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("\n"), 0);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 175006);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 207961583799296);
    }
}
