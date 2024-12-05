pub fn part_a(input: &str) -> i64 {
    for line in input.trim().split('\n') {}
    0
}

pub fn part_b(input: &str) -> i64 {
    for line in input.trim().split('\n') {}
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("\n"), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("\n"), 0);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 0);
    }
}
