pub fn part1(input: &str) -> u32 {
    let parse = |line| {
        let card = Card::parse(line)?;
        let count = card.count().checked_sub(1)?;
        Some(1 << count)
    };

    input.lines().filter_map(parse).sum()
}

pub fn part2(input: &str) -> u32 {
    let parse = |line| {
        let card = Card::parse(line)?;
        Some(card.count())
    };

    let cards: Vec<_> = input.lines().filter_map(parse).collect();
    let mut stack: Vec<_> = (0..cards.len()).collect();
    let mut count = 0;
    while let Some(idx) = stack.pop() {
        stack.extend(idx + 1..=idx + cards[idx]);
        count += 1;
    }

    count
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    nums: Vec<u32>,
    wins: Vec<u32>,
}

impl Card {
    fn parse(s: &str) -> Option<Self> {
        let (card, sets) = s.trim().split_once(':')?;
        let id = card.strip_prefix("Card")?.trim().parse().ok()?;
        let (nums, wins) = sets.split_once('|')?;
        let nums: Option<_> = nums.split_whitespace().map(|n| n.parse().ok()).collect();
        let wins: Option<_> = wins.split_whitespace().map(|n| n.parse().ok()).collect();
        Some(Self {
            id,
            nums: nums?,
            wins: wins?,
        })
    }

    fn count(&self) -> usize {
        self.nums.iter().filter(|n| self.wins.contains(n)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_card() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            Card::parse(s),
            Some(Card {
                id: 1,
                nums: vec![41, 48, 83, 86, 17],
                wins: vec![83, 86, 6, 31, 17, 9, 48, 53],
            })
        );

        let s = "Card   2: 1 2 3 | 1 2 3";
        assert_eq!(
            Card::parse(s),
            Some(Card {
                id: 2,
                nums: vec![1, 2, 3],
                wins: vec![1, 2, 3],
            })
        );
    }

    #[test]
    fn test_part1() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        let output = part1(input);
        assert_eq!(output, 13);
    }

    #[test]
    fn test_part2() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        let output = part2(input);
        assert_eq!(output, 30);
    }
}
