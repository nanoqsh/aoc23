pub fn part1(input: &str) -> u32 {
    let scheme = Scheme::parse(input);
    let mut sum = 0;
    scheme.numbers(|x| sum += x.num);
    sum
}

pub fn part2(input: &str) -> u32 {
    let scheme = Scheme::parse(input);
    let mut sum = 0;
    scheme.gears(|a, b| sum += a.num * b.num);
    sum
}

struct Scheme<'a>(Vec<&'a [u8]>);

impl<'a> Scheme<'a> {
    fn parse(s: &'a str) -> Self {
        Self(
            s.lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(str::as_bytes)
                .collect(),
        )
    }

    fn numbers<F>(&self, mut yeeld: F)
    where
        F: FnMut(Number),
    {
        let get = |x: isize, y: isize| {
            self.0
                .get(y as usize)
                .and_then(|row| row.get(x as usize))
                .copied()
                .unwrap_or(b'.')
        };

        let parse_digit = |ch: u8| ch.is_ascii_digit().then(|| u32::from(ch - b'0'));
        let mut number: Option<Number> = None;
        let mut symbol = false;
        for (y, &row) in (0..).zip(&self.0) {
            for (x, &ch) in (0..).zip(row) {
                match (number, parse_digit(ch)) {
                    (Some(n), Some(digit)) => {
                        const OFFSETS: [Position; 2] = [(0, -1), (0, 1)];

                        symbol = symbol
                            || OFFSETS
                                .into_iter()
                                .map(|(dx, dy)| get(x + dx, y + dy))
                                .any(|ch| ch != b'.');

                        number = Some(n.with(digit));
                    }
                    (Some(n), None) => {
                        const OFFSETS: [Position; 3] = [(0, -1), (0, 0), (0, 1)];

                        symbol = symbol
                            || OFFSETS
                                .into_iter()
                                .map(|(dx, dy)| get(x + dx, y + dy))
                                .any(|ch| ch != b'.');

                        if symbol {
                            yeeld(n);
                        }

                        number = None;
                        symbol = false;
                    }
                    (None, Some(digit)) => {
                        const OFFSETS: [Position; 5] =
                            [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1)];

                        symbol = symbol
                            || OFFSETS
                                .into_iter()
                                .map(|(dx, dy)| get(x + dx, y + dy))
                                .any(|ch| ch != b'.');

                        number = Some(Number::new(x, y).with(digit));
                    }
                    (None, None) => {}
                }
            }

            if let Some(n) = number {
                if symbol {
                    yeeld(n);
                }

                number = None;
                symbol = false;
            }
        }
    }

    fn gears<F>(&self, mut yeeld: F)
    where
        F: FnMut(Number, Number),
    {
        let nums = {
            let mut nums = vec![];
            self.numbers(|x| nums.push(x));
            nums
        };

        for (y, &row) in (0..).zip(&self.0) {
            for (x, &ch) in (0..).zip(row) {
                if ch != b'*' {
                    continue;
                }

                let ns = {
                    #[rustfmt::skip]
                    const OFFSETS: [Position; 8] = [
                        (-1, -1), (0, -1), (1, -1),
                        (-1,  0),          (1,  0),
                        (-1,  1), (0,  1), (1,  1),
                    ];

                    let find = |(dx, dy)| nums.iter().copied().find(|n| n.at(x + dx, y + dy));
                    OFFSETS.into_iter().filter_map(find)
                };

                let mut found = None;
                for n in ns {
                    match found {
                        None => found = Some(n),
                        Some(u) if u.pos != n.pos => {
                            yeeld(u, n);
                            break;
                        }
                        Some(_) => {}
                    }
                }
            }
        }
    }
}

type Position = (isize, isize);

#[derive(Clone, Copy)]
struct Number {
    num: u32,
    len: usize,
    pos: Position,
}

impl Number {
    fn new(x: isize, y: isize) -> Self {
        Self {
            num: 0,
            len: 0,
            pos: (x, y),
        }
    }

    fn with(mut self, d: u32) -> Self {
        self.num *= 10;
        self.num += d;
        self.len += 1;
        self
    }

    fn at(self, x: isize, y: isize) -> bool {
        let (nx, ny) = self.pos;
        (nx..nx + self.len as isize).contains(&x) && ny == y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";

        let output = part1(input);
        assert_eq!(output, 4361);
    }

    #[test]
    fn test_part2() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";

        let output = part2(input);
        assert_eq!(output, 467835);
    }
}
