fn is_valid1(target: i64, ns: &[i64]) -> bool {
    if ns.len() == 1 {
        return ns[0] == target;
    }

    let mut a = vec![ns[0] + ns[1]];
    a.extend_from_slice(&ns[2..]);
    if is_valid1(target, &a) {
        return true;
    }

    let mut b = vec![ns[0] * ns[1]];
    b.extend_from_slice(&ns[2..]);
    if is_valid1(target, &b) {
        return true;
    }

    return false;
}

fn is_valid2(target: i64, ns: &[i64]) -> bool {
    if ns.len() == 1 {
        return ns[0] == target;
    }

    let mut a = vec![ns[0] + ns[1]];
    a.extend_from_slice(&ns[2..]);
    if is_valid2(target, &a) {
        return true;
    }

    let mut b = vec![ns[0] * ns[1]];
    b.extend_from_slice(&ns[2..]);
    if is_valid2(target, &b) {
        return true;
    }

    let mut c = vec![(ns[0].to_string() + ns[1].to_string().as_str())
        .parse::<i64>()
        .unwrap()];
    c.extend_from_slice(&ns[2..]);
    if is_valid2(target, &c) {
        return true;
    }

    return false;
}

pub fn part_a(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let mut split = line.split(':');
        let target = split.next().unwrap().parse::<i64>().unwrap();
        let numbers: Vec<i64> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        if is_valid1(target, &numbers) {
            sum += target;
        }
    }
    sum
}

pub fn part_b(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let mut split = line.split(':');
        let target = split.next().unwrap().parse::<i64>().unwrap();
        let numbers: Vec<i64> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        if is_valid2(target, &numbers) {
            sum += target;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
\n"
            ),
            3749
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
\n"
            ),
            11387
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 267566105056);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 116094961956019);
    }
}
