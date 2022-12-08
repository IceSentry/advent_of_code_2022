use hashbrown::HashSet;

type Data = Vec<(Sections, Sections)>;

#[derive(Debug)]
pub struct Sections {
    min: usize,
    max: usize,
}

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|l| {
            let sections = l.split(',').collect::<Vec<_>>();
            let a = sections[0].split('-').collect::<Vec<_>>();
            let b = sections[1].split('-').collect::<Vec<_>>();
            (
                Sections {
                    min: a[0].parse().unwrap(),
                    max: a[1].parse().unwrap(),
                },
                Sections {
                    min: b[0].parse().unwrap(),
                    max: b[1].parse().unwrap(),
                },
            )
        })
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    input
        .iter()
        .filter(|(a, b)| a.min >= b.min && a.max <= b.max || b.min >= a.min && b.max <= a.max)
        .count()
}

pub fn part_2(input: &Data) -> usize {
    input
        .iter()
        .map(|(a, b)| {
            (
                (a.min..=a.max).collect::<HashSet<_>>(),
                (b.min..=b.max).collect::<HashSet<_>>(),
            )
        })
        .filter(|(a, b)| a.intersection(b).count() > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 4);
    }
}
