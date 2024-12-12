use std::collections::HashSet;

fn find_trails(
    grid: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    n: &mut HashSet<(usize, usize)>,
    m: &mut u32,
) {
    let current = grid[y][x];
    if current == 9 {
        n.insert((x, y));
        *m += 1;
        return;
    }

    if x > 0 && grid[y][x - 1] == current + 1 {
        find_trails(grid, x - 1, y, width, height, n, m);
    }
    if x < width - 1 && grid[y][x + 1] == current + 1 {
        find_trails(grid, x + 1, y, width, height, n, m);
    }
    if y > 0 && grid[y - 1][x] == current + 1 {
        find_trails(grid, x, y - 1, width, height, n, m);
    }
    if y < height - 1 && grid[y + 1][x] == current + 1 {
        find_trails(grid, x, y + 1, width, height, n, m);
    }
}

pub fn part_a(input: &str) -> i64 {
    let grid: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                let mut n: HashSet<(usize, usize)> = HashSet::new();
                let mut m: u32 = 0;
                find_trails(&grid, x, y, width, height, &mut n, &mut m);
                sum += n.len();
            }
        }
    }

    sum as i64
}

pub fn part_b(input: &str) -> i64 {
    let grid: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                let mut n: HashSet<(usize, usize)> = HashSet::new();
                let mut m: u32 = 0;
                find_trails(&grid, x, y, width, height, &mut n, &mut m);
                sum += m;
            }
        }
    }

    sum as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "1190119
1111198
1112117
6543456
7651987
8761111
9871111
\n"
            ),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
\n"
            ),
            81
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 776);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1657);
    }
}
