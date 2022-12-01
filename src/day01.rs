type Data = Vec<usize>;

pub fn parse(input: &str) -> Data {
    input
        .split("\n\n")
        .into_iter()
        .map(|x| x.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    *input.iter().max().unwrap()
}

pub fn part_2(input: &Data) -> usize {
    let mut temp = input.clone();
    temp.sort();
    temp.reverse();
    temp.truncate(3);
    temp.iter().sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 24000);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 45000);
    }
}
