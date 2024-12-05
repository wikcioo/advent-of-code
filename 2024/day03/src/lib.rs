use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for line in input.trim().split('\n') {
        for (_, [n1, n2]) in re.captures_iter(line).map(|c| c.extract()) {
            sum += n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap();
        }
    }
    sum
}

pub fn part_b(input: &str) -> i64 {
    let re = Regex::new(r"(do)\(\)|(don't)\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    let mut can_parse = true;
    for line in input.trim().split('\n') {
        for captures in re.captures_iter(line) {
            if let Some(_) = captures.get(1) {
                can_parse = true;
            } else if let Some(_) = captures.get(2) {
                can_parse = false;
            } else if let (Some(n1), Some(n2)) = (captures.get(3), captures.get(4)) {
                if can_parse {
                    let num1 = n1.as_str().parse::<i64>().unwrap();
                    let num2 = n2.as_str().parse::<i64>().unwrap();
                    sum += num1 * num2;
                }
            }
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
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\n"
            ),
            161
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 162813399);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 53783319);
    }
}
