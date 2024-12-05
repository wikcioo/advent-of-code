pub fn part_a(input: &str) -> i64 {
    let width = input.trim().find('\n').unwrap();
    let height = input.trim().split('\n').count();
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let target = "XMAS";
    let reverse = &target.chars().rev().collect::<String>();
    let mut count = 0;

    // rows
    count += lines
        .iter()
        .map(|s| s.matches(target).count() + s.matches(reverse).count())
        .sum::<usize>();

    // columns
    count += (0..width)
        .map(|x| {
            (0..height)
                .map(|y| lines[y].chars().nth(x).unwrap())
                .collect::<String>()
        })
        .map(|s| s.matches(target).count() + s.matches(reverse).count())
        .sum::<usize>();

    // diagonals
    let window_len = 4;
    let data = input.replace("\n", "");
    for y in 0..=(height - window_len) as usize {
        for x in 0..=(width - window_len) as usize {
            let mut window: Vec<&str> = vec![];
            for yy in 0..window_len as usize {
                let i: usize = (yy + y) * width + x;
                window.push(&data[i..(i + window_len)]);
            }

            let diagonals = [
                (0..window_len)
                    .map(|i| window[i].chars().nth(i).unwrap())
                    .collect::<String>(),
                (0..window_len)
                    .map(|i| window[window_len - i - 1].chars().nth(i).unwrap())
                    .collect::<String>(),
            ];

            for d in &diagonals {
                if d.contains(target) || d.contains(reverse) {
                    count += 1;
                }
            }
        }
    }

    count as i64
}

pub fn part_b(input: &str) -> i64 {
    let width = input.trim().find('\n').unwrap();
    let height = input.trim().split('\n').count();
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let target = "MAS";
    let reverse = &target.chars().rev().collect::<String>();
    let mut count = 0;

    let window_len = 3;
    let data = input.replace("\n", "");
    for y in 0..=(height - window_len) as usize {
        for x in 0..=(width - window_len) as usize {
            let mut window: Vec<&str> = vec![];
            for yy in 0..window_len as usize {
                let i: usize = (yy + y) * width + x;
                window.push(&data[i..(i + window_len)]);
            }

            let diagonals = [
                (0..window_len)
                    .map(|i| window[i].chars().nth(i).unwrap())
                    .collect::<String>(),
                (0..window_len)
                    .map(|i| window[window_len - i - 1].chars().nth(i).unwrap())
                    .collect::<String>(),
            ];

            if (diagonals[0].contains(target) || diagonals[0].contains(reverse))
                && (diagonals[1].contains(target) || diagonals[1].contains(reverse))
            {
                count += 1;
            }
        }
    }

    count as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
\n"
            ),
            18
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
\n"
            ),
            9
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2344);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1815);
    }
}
