type Data = Vec<Vec<char>>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|l| l.parse::<String>().unwrap().chars().collect())
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    input
        .iter()
        .map(|sack| sack.split_at(sack.len() / 2))
        .map(|(a, b)| {
            let ac = a.iter().find(|ac| b.contains(ac)).unwrap();
            priority(*ac)
        })
        .sum::<u32>() as usize
}

pub fn part_2(input: &Data) -> usize {
    input
        .chunks(3)
        .flat_map(|group| {
            let [a, b, c] = group else {
                unreachable!()
            };
            a.iter().find(|ac| b.contains(ac) && c.contains(ac))
        })
        .map(|ac| priority(*ac))
        .sum::<u32>() as usize
}

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_uppercase() {
        c as u32 - 'A' as u32 + 26 + 1
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 157);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 70);
    }
}
