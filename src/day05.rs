use serde_scan::scan;

type Data = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

pub fn parse(input: &str) -> Data {
    let inputs = input.split("\n\n").collect::<Vec<_>>();

    let drawing = inputs[0].lines().rev().collect::<Vec<_>>();
    let mut stacks: Vec<Vec<char>> = vec![];
    for line in drawing.iter().skip(1) {
        let mut i = 0;
        let mut chars = line.chars().peekable();
        loop {
            // skip [ or space
            if chars.next().is_none() {
                break;
            }
            let crate_name = chars.next().unwrap();
            // skip ]
            chars.next();
            // skip space between
            chars.next();

            if crate_name.is_alphabetic() {
                if stacks.get(i).is_none() {
                    stacks.push(vec![]);
                }
                stacks[i].push(crate_name);
            }
            i += 1;
        }
    }

    let procedures = inputs[1]
        .lines()
        .map(|l| scan!("move {} from {} to {}" <- l).expect("Failed to parse input"))
        .collect::<Vec<(usize, usize, usize)>>();

    (stacks, procedures)
}

pub fn part_1(input: &Data) -> String {
    let mut stacks = input.0.clone();
    let procedures = input.1.clone();
    for (a, b, c) in procedures {
        for _ in 0..a {
            let val = stacks[b - 1].pop().unwrap();
            stacks[c - 1].push(val);
        }
    }
    stacks
        .iter()
        .flat_map(|stack| stack.last())
        .collect::<String>()
}

pub fn part_2(input: &Data) -> String {
    let mut stacks = input.0.clone();
    let procedures = input.1.clone();
    for (a, b, c) in procedures {
        let mut temp = vec![];
        for _ in 0..a {
            let val = stacks[b - 1].pop().unwrap();
            temp.push(val);
        }
        temp.reverse();
        stacks[c - 1].extend(temp);
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
        println!("{input:?}");
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
