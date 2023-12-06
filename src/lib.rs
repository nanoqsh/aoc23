pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let output = day1::part1(include_str!("day1"));
        assert_eq!(output, 53080);

        let output = day1::part2(include_str!("day1"));
        assert_eq!(output, 53268);
    }

    #[test]
    fn day2() {
        let output = day2::part1(include_str!("day2"));
        assert_eq!(output, 2256);

        let output = day2::part2(include_str!("day2"));
        assert_eq!(output, 74229);
    }

    #[test]
    fn day3() {
        let output = day3::part1(include_str!("day3"));
        assert_eq!(output, 556367);

        let output = day3::part2(include_str!("day3"));
        assert_eq!(output, 89471771);
    }

    #[test]
    fn day4() {
        let output = day4::part1(include_str!("day4"));
        assert_eq!(output, 21821);

        let output = day4::part2(include_str!("day4"));
        assert_eq!(output, 5539496);
    }
}
