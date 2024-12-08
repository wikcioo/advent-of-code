use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut av = Vec::new();
    let mut bv = Vec::new();

    for line in input.trim().split('\n') {
        let mut split = line.split_whitespace();
        av.push(split.next().unwrap().parse::<i64>().unwrap());
        bv.push(split.next().unwrap().parse::<i64>().unwrap());
    }

    av.sort();
    bv.sort();

    let mut diff = 0;
    for i in 0..av.len() {
        diff += (av[i] - bv[i]).abs();
    }

    diff
}

pub fn part_b(input: &str) -> i64 {
    let mut av = Vec::new();
    let mut b: HashMap<i64, i64> = HashMap::new();

    for line in input.trim().split('\n') {
        let mut split = line.split_whitespace();
        av.push(split.next().unwrap().parse::<i64>().unwrap());

        let entry = b
            .entry(split.next().unwrap().parse::<i64>().unwrap())
            .or_insert(0);
        *entry += 1;
    }

    let mut result = 0;
    for n in av {
        if let Some(freq) = b.get(&n) {
            result += n * freq;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "3   4
4   3
2   5
1   3
3   9
3   3
\n"
            ),
            11
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "3   4
4   3
2   5
1   3
3   9
3   3
\n"
            ),
            31
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1651298);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 21306195);
    }
}
