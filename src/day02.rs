use serde_scan::scan;
use strum::EnumString;

type Data = Vec<(String, String)>;

#[derive(Debug, PartialEq, EnumString, Copy, Clone)]
enum Hand {
    #[strum(serialize = "A", serialize = "X")]
    Rock,
    #[strum(serialize = "B", serialize = "Y")]
    Paper,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors,
}

#[derive(Debug, PartialEq, EnumString, Copy, Clone)]
enum Move {
    #[strum(serialize = "X")]
    Lose,
    #[strum(serialize = "Y")]
    Draw,
    #[strum(serialize = "Z")]
    Win,
}

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| scan!("{} {}" <- line).expect("Failed to parse input"))
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    let input: Vec<(Hand, Hand)> = input
        .iter()
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let mut total = 0;
    for (opponent, you) in input {
        total += score_hand(&you) + score_game((opponent, you));
    }
    total
}

pub fn part_2(input: &Data) -> usize {
    use Hand::*;
    use Move::*;

    let input: Vec<(Hand, Move)> = input
        .iter()
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let mut total = 0;
    for (opponent, needs_to_end) in input {
        let you = match (needs_to_end, opponent) {
            // lose
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            // win
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            // draw
            (Draw, _) => opponent,
        };
        total += score_hand(&you) + score_game((opponent, you));
    }
    total
}

fn score_hand(hand: &Hand) -> usize {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn score_game((opponent, you): (Hand, Hand)) -> usize {
    use Hand::*;
    match (opponent, you) {
        // win
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        // draw
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        // lose
        _ => 0,
    }
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
