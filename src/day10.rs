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

struct Cpu {
    cycle_count: usize,
    register_x: i32,
    signal_strength: usize,
    draw: bool,
}

impl Cpu {
    fn new(draw: bool) -> Self {
        Self {
            cycle_count: 0,
            register_x: 1,
            signal_strength: 0,
            draw,
        }
    }

    fn run(&mut self, input: &Data) {
        for (instruction, value) in input {
            self.cycle();
            match instruction.as_str() {
                "noop" => {}
                "addx" => {
                    self.cycle();
                    self.register_x += value.parse::<i32>().unwrap();
                }
                _ => unreachable!(),
            }
        }
    }

    fn cycle(&mut self) {
        self.draw();

        self.cycle_count += 1;
        if self.cycle_count == 20 || (self.cycle_count + 20) % 40 == 0 {
            self.signal_strength += self.cycle_count * self.register_x as usize;
        }
    }

    fn draw(&mut self) {
        if !self.draw {
            return;
        }
        if self.cycle_count % 40 == 0 {
            println!();
        }
        if (self.register_x - self.cycle_count as i32 % 40).abs() <= 1 {
            print!("#");
        } else {
            print!(" ");
        }
    }
}

pub fn part_1(input: &Data) -> usize {
    let mut cpu = Cpu::new(false);
    cpu.run(input);
    cpu.signal_strength
}

pub fn part_2(input: &Data) -> usize {
    let mut cpu = Cpu::new(true);
    cpu.run(input);
    println!();
    0
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
