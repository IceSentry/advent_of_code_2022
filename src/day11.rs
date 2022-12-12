use serde_scan::scan;

type Data = Vec<Monkey>;

pub fn parse(input: &str) -> Data {
    input.split("\n\n").map(parse_monkey).collect()
}

#[derive(Default, Debug, Clone)]
pub struct Monkey {
    index: usize,
    items: Vec<usize>,
    operation: (String, String),
    test_value: usize,
    test_true: usize,
    test_false: usize,
    inspect_count: usize,
}

fn parse_monkey(block: &str) -> Monkey {
    let mut monkey = Monkey::default();
    let mut lines = block.lines();

    let line = lines.next().unwrap();
    monkey.index = scan!("Monkey {}:" <- line).unwrap();

    let line = lines.next().unwrap().trim();
    let items: &str = scan!("Starting items: {}" <- line).unwrap();
    monkey.items = items
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let line = lines.next().unwrap().trim();
    monkey.operation = scan!("Operation: new = old {} {}" <- line).unwrap();

    let line = lines.next().unwrap().trim();
    monkey.test_value =
        scan!("Test: divisible by {}" <- line).unwrap_or_else(|_| panic!("Failed to parse {line}"));

    let line = lines.next().unwrap().trim();
    monkey.test_true = scan!("If true: throw to monkey {}" <- line).unwrap();

    let line = lines.next().unwrap().trim();
    monkey.test_false = scan!("If false: throw to monkey {}" <- line).unwrap();

    monkey
}

pub fn part_1(input: &Data) -> usize {
    let mut monkeys = input.clone();
    for _ in 0..20 {
        for i in 0..input.len() {
            let monkey = monkeys[i].clone();

            if monkey.items.is_empty() {
                continue;
            }

            for item in monkey.items {
                let mut worry_level = item;

                let (operator, value) = &monkey.operation;
                if value != "old" {
                    let value = value.parse::<usize>().unwrap();
                    match operator.as_str() {
                        "*" => worry_level *= value,
                        "+" => worry_level += value,
                        _ => unreachable!(),
                    }
                } else {
                    match operator.as_str() {
                        "*" => worry_level *= worry_level,
                        "+" => worry_level += worry_level,
                        _ => unreachable!(),
                    }
                }

                worry_level /= 3;

                let throw_index = if worry_level % monkey.test_value == 0 {
                    monkey.test_true
                } else {
                    monkey.test_false
                };
                monkeys[throw_index].items.push(worry_level);
            }

            monkeys[i].inspect_count += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by_key(|m| m.inspect_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspect_count)
        .product()
}

pub fn part_2(input: &Data) -> usize {
    let mut monkeys = input.clone();
    let modulo: usize = monkeys.iter().map(|m| m.test_value).product();

    for _ in 0..10000 {
        for i in 0..input.len() {
            let monkey = monkeys[i].clone();

            if monkey.items.is_empty() {
                continue;
            }

            for item in monkey.items {
                let mut worry_level = item;

                let (operator, value) = &monkey.operation;
                if value != "old" {
                    let value = value.parse::<usize>().unwrap();
                    match operator.as_str() {
                        "*" => worry_level *= value,
                        "+" => worry_level += value,
                        _ => unreachable!(),
                    }
                } else {
                    match operator.as_str() {
                        "*" => worry_level *= worry_level,
                        "+" => worry_level += worry_level,
                        _ => unreachable!(),
                    }
                }

                worry_level %= modulo;

                let throw_index = if worry_level % monkey.test_value == 0 {
                    monkey.test_true
                } else {
                    monkey.test_false
                };
                monkeys[throw_index].items.push(worry_level);
            }

            monkeys[i].inspect_count += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by_key(|m| m.inspect_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspect_count)
        .product()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        Monkey 0:
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

        Monkey 1:
            Starting items: 54, 65, 75, 74
            Operation: new = old + 6
            Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

        Monkey 2:
            Starting items: 79, 60, 97
            Operation: new = old * old
            Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

        Monkey 3:
            Starting items: 74
            Operation: new = old + 3
            Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 10605);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 2713310158);
    }
}
