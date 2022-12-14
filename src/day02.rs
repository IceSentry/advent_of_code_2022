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

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
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
    input
        .iter()
        .map(|(a, b)| (a.parse::<Hand>().unwrap(), b.parse::<Hand>().unwrap()))
        .map(|(opponent, you)| you.score() + score_game((opponent, you)))
        .sum()
}

pub fn part_2(input: &Data) -> usize {
    use Hand::*;
    use Move::*;
    input
        .iter()
        .map(|(a, b)| (a.parse::<Hand>().unwrap(), b.parse::<Move>().unwrap()))
        .map(|(opponent, end_move)| {
            let you = match (opponent, end_move) {
                (Rock, Lose) => Scissors,
                (Rock, Win) => Paper,
                (Paper, Lose) => Rock,
                (Paper, Win) => Scissors,
                (Scissors, Lose) => Paper,
                (Scissors, Win) => Rock,
                (_, Draw) => opponent,
            };
            you.score() + score_game((opponent, you))
        })
        .sum()
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
