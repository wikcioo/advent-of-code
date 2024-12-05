use std::collections::HashMap;

fn get_data(input: &str) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let mut split = input.trim().split("\n\n");
    let rules_str = split.next().unwrap();
    let updates_str = split.next().unwrap();

    let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();

    rules_str
        .split('\n')
        .map(|n| {
            let p: Vec<&str> = n.split('|').collect();
            (p[0], p[1])
        })
        .for_each(|(x, y)| {
            rules
                .entry(x.parse::<i64>().unwrap())
                .or_insert_with(Vec::new)
                .push(y.parse::<i64>().unwrap());
        });

    let updates: Vec<Vec<i64>> = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect()
        })
        .collect();

    (rules, updates)
}

pub fn part_a(input: &str) -> i64 {
    let (rules, updates) = get_data(input);

    let mut sum = 0;
    for update in updates {
        let mut correct = true;
        for i in 1..update.len() {
            if let Some(rule) = rules.get(&update[i - 1]) {
                if !rule.contains(&update[i]) {
                    correct = false;
                    break;
                }
            } else if let Some(rule) = rules.get(&update[i]) {
                if rule.contains(&update[i - 1]) {
                    correct = false;
                    break;
                }
            }
        }

        if correct {
            sum += update[update.len() / 2];
        }
    }

    sum
}

pub fn part_b(input: &str) -> i64 {
    let (rules, updates) = get_data(input);

    let mut sum = 0;
    for update in updates {
        let mut correct = true;
        for i in 1..update.len() {
            if let Some(rule) = rules.get(&update[i - 1]) {
                if !rule.contains(&update[i]) {
                    correct = false;
                    break;
                }
            } else if let Some(rule) = rules.get(&update[i]) {
                if rule.contains(&update[i - 1]) {
                    correct = false;
                    break;
                }
            }
        }

        if !correct {
            let mut new_update = Vec::with_capacity(update.len());
            for page in &update {
                if let Some(rule) = rules.get(&page) {
                    if let Some(index) = new_update.iter().position(|&x| rule.contains(&x)) {
                        new_update.insert(index, *page);
                    } else {
                        new_update.push(*page);
                    }
                } else {
                    new_update.push(*page);
                }
            }

            if new_update.len() > 0 {
                sum += new_update[new_update.len() / 2];
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
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
\n"
            ),
            143
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
\n"
            ),
            123
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 7365);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 5770);
    }
}
