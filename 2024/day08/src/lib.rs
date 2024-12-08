use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Vec2 {
    x: i64,
    y: i64,
}

pub fn part_a(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width: i64 = grid[0].len().try_into().unwrap();
    let height: i64 = grid.len().try_into().unwrap();

    let mut map: HashMap<char, Vec<Vec2>> = HashMap::new();
    let mut antennas: HashSet<Vec2> = HashSet::new();

    let mut check_and_insert = |x, y| {
        if (0..width).contains(&x) && (0..height).contains(&y) && !antennas.contains(&Vec2 { x, y })
        {
            antennas.insert(Vec2 { x, y });
        }
    };

    for y in 0..height {
        for x in 0..width {
            let c = grid[y as usize][x as usize];
            if c == '.' {
                continue;
            }

            let entry = map.entry(c).or_insert(Vec::new());

            if entry.len() > 0 {
                for pos in entry.iter() {
                    let (dx, dy) = (x - pos.x, y - pos.y);
                    let (xx1, yy1) = (pos.x - dx, pos.y - dy);
                    let (xx2, yy2) = (x + dx, y + dy);
                    check_and_insert(xx1, yy1);
                    check_and_insert(xx2, yy2);
                }
            }

            entry.push(Vec2 { x, y });
        }
    }

    antennas.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width: i64 = grid[0].len().try_into().unwrap();
    let height: i64 = grid.len().try_into().unwrap();

    let mut map: HashMap<char, Vec<Vec2>> = HashMap::new();
    let mut antennas: HashSet<Vec2> = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            let c = grid[y as usize][x as usize];
            if c == '.' {
                continue;
            }

            let entry = map.entry(c).or_insert(Vec::new());
            antennas.insert(Vec2 { x, y });

            if entry.len() > 0 {
                for pos in entry.iter() {
                    let (dx, dy) = (x - pos.x, y - pos.y);

                    for i in 1.. {
                        let (xx1, yy1) = (pos.x - (dx * i), pos.y - (dy * i));
                        if !(0..width).contains(&xx1) || !(0..height).contains(&yy1) {
                            break;
                        }

                        if !antennas.contains(&Vec2 { x: xx1, y: yy1 }) {
                            antennas.insert(Vec2 { x: xx1, y: yy1 });
                        }
                    }

                    for i in 1.. {
                        let (xx2, yy2) = (pos.x + (dx * i), pos.y + (dy * i));
                        if !(0..width).contains(&xx2) || !(0..height).contains(&yy2) {
                            break;
                        }

                        if !antennas.contains(&Vec2 { x: xx2, y: yy2 }) {
                            antennas.insert(Vec2 { x: xx2, y: yy2 });
                        }
                    }
                }
            }

            entry.push(Vec2 { x, y });
        }
    }

    antennas.len() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
\n"
            ),
            14
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
\n"
            ),
            34
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 320);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1157);
    }
}
