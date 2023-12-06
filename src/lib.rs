pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        const TASKS: [(u8, u8, fn(&str) -> u32, &str, u32); 8] = [
            (1, 1, day1::part1, include_str!("day1"), 53080),
            (1, 2, day1::part2, include_str!("day1"), 53268),
            (2, 1, day2::part1, include_str!("day2"), 2256),
            (2, 2, day2::part2, include_str!("day2"), 74229),
            (3, 1, day3::part1, include_str!("day3"), 556367),
            (3, 2, day3::part2, include_str!("day3"), 89471771),
            (4, 1, day4::part1, include_str!("day4"), 21821),
            (4, 2, day4::part2, include_str!("day4"), 5539496),
        ];

        for (day, part, task, input, expected) in TASKS {
            assert_eq!(task(input), expected, "day {day} part {part}");
        }
    }
}
