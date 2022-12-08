use hashbrown::HashSet;

type Data = Vec<char>;

pub fn parse(input: &str) -> Data {
    input.chars().collect()
}

pub fn part_1(input: &Data) -> usize {
    find_marker(input, 4)
}

pub fn part_2(input: &Data) -> usize {
    find_marker(input, 14)
}

fn find_marker(data: &Data, size: usize) -> usize {
    data.windows(size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == size)
        .unwrap()
        + size
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn part_1() {
        let inputs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (input, expected) in inputs {
            let parsed = super::parse(input);
            let result = super::part_1(&parsed);
            assert_eq!(result, expected);
        }
    }

    #[test]
    pub fn part_2() {
        let inputs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (input, expected) in inputs {
            let parsed = super::parse(input);
            let result = super::part_2(&parsed);
            assert_eq!(result, expected);
        }
    }
}
