use serde_scan::scan;

type Data = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

pub fn parse(input: &str) -> Data {
    let inputs = input.split("\n\n").collect::<Vec<_>>();

    let drawing = inputs[0]
        .lines()
        .rev()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut stacks = vec![];
    for line in drawing.iter().skip(1) {
        for (i, chunk) in line.chunks(4).enumerate() {
            let crate_name = chunk[1];
            if crate_name.is_alphabetic() {
                if stacks.get(i).is_none() {
                    stacks.push(vec![]);
                }
                stacks[i].push(crate_name);
            }
        }
    }

    let procedures = inputs[1]
        .lines()
        .map(|l| scan!("move {} from {} to {}" <- l).expect("Failed to parse input"))
        .collect::<Vec<(usize, usize, usize)>>();

    (stacks, procedures)
}

pub fn part_1((stacks, procedures): &Data) -> String {
    let mut stacks = stacks.clone();
    for (a, b, c) in procedures {
        let stack = (0..*a)
            .flat_map(|_| stacks[b - 1].pop())
            .collect::<Vec<_>>();
        stacks[c - 1].extend(stack);
    }
    stacks
        .iter()
        .flat_map(|stack| stack.last())
        .collect::<String>()
}

pub fn part_2((stacks, procedures): &Data) -> String {
    let mut stacks = stacks.clone();
    for (a, b, c) in procedures {
        let mut stack = (0..*a)
            .flat_map(|_| stacks[b - 1].pop())
            .collect::<Vec<_>>();
        stack.reverse();
        stacks[c - 1].extend(stack);
    }
    stacks
        .iter()
        .flat_map(|stack| stack.last())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[rustfmt::skip]
    const INPUTS: &str = indoc! {"
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, "CMZ");
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, "MCD");
    }
}
