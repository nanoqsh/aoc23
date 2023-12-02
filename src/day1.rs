pub fn p1(input: &str) -> u32 {
    let parse = |line| {
        let x = parse_line(Next(line), digit);
        let y = parse_line(Prev(line), digit);
        x * 10 + y
    };

    input.lines().map(parse).sum()
}

pub fn p2(input: &str) -> u32 {
    let parse = |line| {
        let x = parse_line(Next(line), |cx| digit(cx).or_else(|| literal(cx)));
        let y = parse_line(Prev(line), |cx| digit(cx).or_else(|| literal(cx)));
        x * 10 + y
    };

    input.lines().map(parse).sum()
}

fn parse_line<C, F>(mut cx: C, f: F) -> u32
where
    C: Context,
    F: Fn(C) -> Option<u32>,
{
    loop {
        if let Some(n) = f(cx) {
            break n;
        }

        let Some(new) = cx.strip() else {
            break 0;
        };

        cx = new;
    }
}

fn digit<C>(cx: C) -> Option<u32>
where
    C: Context,
{
    cx.next()?.to_digit(10)
}

fn literal<C>(cx: C) -> Option<u32>
where
    C: Context,
{
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    NUMS.iter().position(|n| cx.with(n)).map(|n| (n + 1) as _)
}

trait Context: Copy {
    fn next(self) -> Option<char>;
    fn with(self, s: &str) -> bool;
    fn strip(self) -> Option<Self>;
}

#[derive(Clone, Copy)]
struct Next<'a>(&'a str);

impl Context for Next<'_> {
    fn next(self) -> Option<char> {
        self.0.chars().next()
    }

    fn with(self, s: &str) -> bool {
        self.0.starts_with(s)
    }

    fn strip(self) -> Option<Self> {
        self.0.strip_prefix(|_| true).map(Self)
    }
}

#[derive(Clone, Copy)]
struct Prev<'a>(&'a str);

impl Context for Prev<'_> {
    fn next(self) -> Option<char> {
        self.0.chars().next_back()
    }

    fn with(self, s: &str) -> bool {
        self.0.ends_with(s)
    }

    fn strip(self) -> Option<Self> {
        self.0.strip_suffix(|_| true).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";

        let output = p1(input);
        assert_eq!(output, 142);
    }

    #[test]
    fn test_p2() {
        let input = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";

        let output = p2(input);
        assert_eq!(output, 281);
    }
}
