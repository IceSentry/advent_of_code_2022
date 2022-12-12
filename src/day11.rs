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
    monkey.test_value = scan!("Test: divisible by {}" <- line).unwrap();

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
            // println!("Monkey {i}:");
            let monkey = monkeys[i].clone();

            if monkey.items.is_empty() {
                continue;
            }

            for item in monkey.items {
                let mut worry_level = item;
                // println!("  Monkey inspects an item with a worry level of {worry_level}.");

                let (operator, value) = &monkey.operation;
                if value != "old" {
                    let value = value.parse::<usize>().unwrap();
                    match operator.as_str() {
                        "*" => worry_level *= value,
                        "+" => worry_level += value,
                        _ => unreachable!(),
                    }
                    // println!("    Worry level is {operator} by {value} to {worry_level}.")
                } else {
                    match operator.as_str() {
                        "*" => worry_level *= worry_level,
                        "+" => worry_level += worry_level,
                        _ => unreachable!(),
                    }
                    // println!("    Worry level is {operator} by itself to {worry_level}.")
                }

                worry_level /= 3;
                // println!(
                //     "    Monkey gets bored with item. Worry level is divided by 3 to {worry_level}."
                // );

                let throw_index = if worry_level % monkey.test_value == 0 {
                    // println!(
                    //     "    Current worry level is divisible by {}.",
                    //     monkey.test_value
                    // );
                    monkey.test_true
                } else {
                    // println!(
                    //     "    Current worry level is not divisible by {}.",
                    //     monkey.test_value
                    // );
                    monkey.test_false
                };
                // println!(
                //     "    Item with worry level {worry_level} is thrown to monkey {throw_index}."
                // );
                monkeys[throw_index].items.push(worry_level);
            }

            monkeys[i].inspect_count += monkeys[i].items.len();
            monkeys[i].items.clear();
        }

        // println!();
        // for m in &monkeys {
        //     println!("Moneky: {} {:?}", m.index, m.items);
        // }
    }

    // println!();
    // for m in &monkeys {
    //     println!(
    //         "Moneky {} inspected items {:?} times.",
    //         m.index, m.inspect_count
    //     );
    // }

    monkeys.sort_by_key(|m| m.inspect_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspect_count)
        .product()
}

pub fn part_2(input: &Data) -> usize {
    0
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
        assert_eq!(result, 0);
    }
}
