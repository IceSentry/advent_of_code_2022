use hashbrown::HashMap;
use serde_scan::scan;

type Data = Vec<(char, char)>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| scan!("{} {}" <- line).expect("Failed to parse input"))
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    let mut scores = HashMap::new();
    scores.insert('X', 1);
    scores.insert('Y', 2);
    scores.insert('Z', 3);

    let mut total = 0;
    for (opponent, you) in input {
        let game_score = match (opponent, you) {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // win
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // draw
            _ => 0,                                    // loss
        };
        total += scores[you] + game_score;
    }
    total
}

// X -> lose
// Y -> draw
// Z -> win
pub fn part_2(input: &Data) -> usize {
    let mut scores = HashMap::new();
    scores.insert('A', 1);
    scores.insert('B', 2);
    scores.insert('C', 3);
    let mut total = 0;
    for (opponent, needs_to_end) in input {
        let you = match (needs_to_end, opponent) {
            // lose
            ('X', 'A') => 'C',
            ('X', 'B') => 'A',
            ('X', 'C') => 'B',
            // win
            ('Z', 'A') => 'B',
            ('Z', 'B') => 'C',
            ('Z', 'C') => 'A',
            // draw
            ('Y', _) => *opponent,
            _ => unreachable!(),
        };

        let game_score = match (opponent, you) {
            ('A', 'B') | ('B', 'C') | ('C', 'A') => 6, // win
            ('A', 'A') | ('B', 'B') | ('C', 'C') => 3, // draw
            _ => 0,                                    // loss
        };
        total += scores[&you] + game_score;
    }
    total
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 15);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 12);
    }
}
