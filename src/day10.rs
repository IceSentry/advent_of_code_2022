use serde_scan::scan;

type Data = Vec<(String, String)>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| match line {
            _ if line.starts_with("noop") => ("noop".into(), "".into()),
            _ => scan!("{} {}" <- line).expect("Failed to parse input"),
        })
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    let mut cycle_count = 0;
    let mut signal_strength = 0;
    let mut x = 1;
    for (instruction, value) in input {
        cycle(&mut cycle_count, &mut signal_strength, x);
        match instruction.as_str() {
            "noop" => {}
            "addx" => {
                cycle(&mut cycle_count, &mut signal_strength, x);
                x += value.parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }
    signal_strength
}

pub fn part_2(input: &Data) -> usize {
    let mut cycle_count = 0;
    let mut signal_strength = 0;
    let mut x = 1;
    for (instruction, value) in input {
        draw(cycle_count, x);
        cycle(&mut cycle_count, &mut signal_strength, x);
        match instruction.as_str() {
            "noop" => {}
            "addx" => {
                draw(cycle_count, x);
                cycle(&mut cycle_count, &mut signal_strength, x);
                x += value.parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }
    println!();
    0
}

fn draw(cycle: usize, x: i32) {
    if cycle % 40 == 0 {
        println!();
    }
    if (x - cycle as i32 % 40).abs() <= 1 {
        print!("#");
    } else {
        print!(" ");
    }
}

fn cycle(cycle: &mut usize, signal_strength: &mut usize, register_x: i32) {
    *cycle += 1;
    if *cycle == 20 || (*cycle + 20) % 40 == 0 {
        *signal_strength += *cycle * register_x as usize;
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 13140);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 0);
    }
}
