pub fn part1(input: &str) -> u32 {
    const SET: Set = [12, 13, 14];

    let parse = |line| {
        let game = Game::parse(line)?;
        game.is_possible(SET).then_some(game.id)
    };

    input.lines().filter_map(parse).sum()
}

pub fn part2(input: &str) -> u32 {
    let parse = |line| {
        let game = Game::parse(line)?;
        let [r, g, b] = game.minimal_set();
        Some(r * g * b)
    };

    input.lines().filter_map(parse).sum()
}

type Set = [u32; 3];

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn parse(s: &str) -> Option<Self> {
        let (game, sets) = s.trim().split_once(':')?;
        let id = game.strip_prefix("Game ")?.parse().ok()?;
        let set = |s: &str| {
            let mut r @ mut g @ mut b = 0;
            for pair in s.split(',') {
                let (number, color) = pair.trim().split_once(' ')?;
                let number: u32 = number.parse().ok()?;
                match color {
                    "red" => r += number,
                    "green" => g += number,
                    "blue" => b += number,
                    _ => return None,
                }
            }

            Some([r, g, b])
        };

        let sets: Option<_> = sets.split(';').map(set).collect();
        Some(Self { id, sets: sets? })
    }

    fn is_possible(&self, [sr, sg, sb]: Set) -> bool {
        self.sets
            .iter()
            .all(|&[r, g, b]| r <= sr && g <= sg && b <= sb)
    }

    fn minimal_set(&self) -> Set {
        let max = |[ar, ag, ab]: Set, [br, bg, bb]: Set| {
            [u32::max(br, ar), u32::max(bg, ag), u32::max(bb, ab)]
        };

        self.sets.iter().copied().reduce(max).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            Game::parse(s),
            Some(Game {
                id: 1,
                sets: vec![[4, 0, 3], [1, 2, 6], [0, 2, 0]],
            })
        );
    }

    #[test]
    fn test_part1() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let output = part1(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn test_part2() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let output = part2(input);
        assert_eq!(output, 2286);
    }
}
