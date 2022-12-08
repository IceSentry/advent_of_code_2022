use hashbrown::HashSet;

type Data = Vec<char>;

pub fn parse(input: &str) -> Data {
    input.chars().collect()
}

pub fn part_1(input: &Data) -> usize {
    for (i, window) in input.windows(4).enumerate() {
        let set = window.iter().collect::<HashSet<_>>();
        if set.len() == 4 {
            println!("found it {i} {window:?}");
            return i + 4;
        }
    }
    unreachable!()
}

pub fn part_2(input: &Data) -> usize {
    for (i, window) in input.windows(14).enumerate() {
        let set = window.iter().collect::<HashSet<_>>();
        if set.len() == 14 {
            println!("found it {i} {window:?}");
            return i + 14;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 7);

        let input = super::parse("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let result = super::part_1(&input);
        assert_eq!(result, 5);
        let input = super::parse("nppdvjthqldpwncqszvftbrmjlhg");
        let result = super::part_1(&input);
        assert_eq!(result, 6);
        let input = super::parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let result = super::part_1(&input);
        assert_eq!(result, 10);
        let input = super::parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let result = super::part_1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 19);

        let input = super::parse("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let result = super::part_2(&input);
        assert_eq!(result, 23);
        let input = super::parse("nppdvjthqldpwncqszvftbrmjlhg");
        let result = super::part_2(&input);
        assert_eq!(result, 23);
        let input = super::parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let result = super::part_2(&input);
        assert_eq!(result, 29);
        let input = super::parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let result = super::part_2(&input);
        assert_eq!(result, 26);
    }
}
