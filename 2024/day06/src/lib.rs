use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

const DIRS: [Vec2; 4] = [
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 0 },
];

fn get_data(input: &str) -> (usize, usize, Vec<Vec<char>>, Vec2, usize) {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut guard_pos = Vec2 { x: 0, y: 0 };
    let mut dirs_idx = 0;

    for y in 0..height {
        for x in 0..width {
            let c = grid[y][x];
            if c != '.' && c != '#' {
                let (xx, yy) = (x as i32, y as i32);
                guard_pos = Vec2 { x: xx, y: yy };
                dirs_idx = match c {
                    '^' => 0,
                    'v' => 2,
                    '<' => 3,
                    '>' => 1,
                    _ => panic!("did not find expected direction symbol"),
                }
            }
        }
    }

    (width, height, grid, guard_pos, dirs_idx)
}

pub fn part_a(input: &str) -> i64 {
    let (width, height, grid, mut guard_pos, mut dirs_idx) = get_data(input);

    let mut visited: HashMap<Vec2, i32> = HashMap::new();

    loop {
        let next_pos = Vec2 {
            x: guard_pos.x + DIRS[dirs_idx].x,
            y: guard_pos.y + DIRS[dirs_idx].y,
        };

        if !(0..width).contains(&(next_pos.x as usize))
            || !(0..height).contains(&(next_pos.y as usize))
        {
            break;
        }

        if grid[next_pos.y as usize][next_pos.x as usize] == '#' {
            dirs_idx = (dirs_idx + 1) % DIRS.len();
            continue;
        }

        visited.entry(guard_pos).or_insert(1);
        guard_pos = next_pos;
    }

    visited.keys().len() as i64 + 1
}

pub fn part_b(input: &str) -> i64 {
    let (width, height, grid, mut guard_pos, mut dirs_idx) = get_data(input);
    let guard_pos_original = guard_pos;
    let dirs_idx_original = dirs_idx;

    let mut visited: HashMap<Vec2, i32> = HashMap::new();

    loop {
        visited.entry(guard_pos).or_insert(1);

        let mut next_pos = Vec2 {
            x: guard_pos.x + DIRS[dirs_idx].x,
            y: guard_pos.y + DIRS[dirs_idx].y,
        };

        if !(0..width).contains(&(next_pos.x as usize))
            || !(0..height).contains(&(next_pos.y as usize))
        {
            break;
        }

        if grid[next_pos.y as usize][next_pos.x as usize] == '#' {
            dirs_idx = (dirs_idx + 1) % DIRS.len();
            next_pos.x = guard_pos.x + DIRS[dirs_idx].x;
            next_pos.y = guard_pos.y + DIRS[dirs_idx].y;
        }

        guard_pos = next_pos;
    }

    let mut count = 0;
    for pos in visited.keys() {
        guard_pos = guard_pos_original;
        dirs_idx = dirs_idx_original;

        if pos.x == guard_pos.x && pos.y == guard_pos.y {
            continue;
        }

        let mut grid_copy = grid.clone();
        grid_copy[pos.y as usize][pos.x as usize] = '#';

        let mut obstacles: HashSet<(Vec2, Vec2)> = HashSet::new();

        loop {
            let next_pos = Vec2 {
                x: guard_pos.x + DIRS[dirs_idx].x,
                y: guard_pos.y + DIRS[dirs_idx].y,
            };

            if !(0..width).contains(&(next_pos.x as usize))
                || !(0..height).contains(&(next_pos.y as usize))
            {
                break;
            }

            if grid_copy[next_pos.y as usize][next_pos.x as usize] == '#' {
                if obstacles.contains(&(next_pos, DIRS[dirs_idx])) {
                    count += 1;
                    break;
                }

                obstacles.insert((next_pos, DIRS[dirs_idx]));

                dirs_idx = (dirs_idx + 1) % DIRS.len();
                continue;
            }

            guard_pos = next_pos;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
\n"
            ),
            41
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
\n"
            ),
            6
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 5329);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2162);
    }
}
