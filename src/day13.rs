use std::collections::VecDeque;
use std::fmt::Write;

type Data = Vec<(Packet, Packet)>;

pub fn parse(input: &str) -> Data {
    input
        .split("\n\n")
        .map(|l| {
            l.split('\n')
                .map(|l| l.chars().collect::<VecDeque<_>>())
                .collect::<Vec<_>>()
        })
        .map(|pair| {
            (
                parse_packet(&mut pair[0].clone())[0].clone(),
                parse_packet(&mut pair[1].clone())[0].clone(),
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

fn packet_string(packet: &Packet) -> String {
    let mut s = String::from("");
    match packet {
        Packet::List(l) => write!(
            s,
            "[{}]",
            l.iter().map(packet_string).collect::<Vec<_>>().join(",")
        )
        .unwrap(),
        Packet::Int(i) => write!(s, "{i}").unwrap(),
    };
    s
}

fn parse_packet(raw_packet: &mut VecDeque<char>) -> Vec<Packet> {
    let mut number = String::from("");
    let mut list = vec![];
    while let Some(c) = raw_packet.pop_front() {
        match c {
            '[' => {
                list.push(Packet::List(parse_packet(raw_packet)));
            }
            ']' => {
                if !number.is_empty() {
                    list.push(Packet::Int(number.parse().unwrap()));
                }
                break;
            }
            ',' => {
                if !number.is_empty() {
                    list.push(Packet::Int(number.parse().unwrap()));
                    number = String::from("");
                }
            }
            c if c.is_ascii_digit() => {
                number = format!("{number}{c}");
            }
            _ => unreachable!(),
        };
    }
    list
}

pub fn part_1(input: &Data) -> usize {
    let mut sum = 0;
    for (i, (left, right)) in input.iter().enumerate() {
        println!("\n== Pair {} ==", i + 1);
        match compare(left, right, 0) {
            std::cmp::Ordering::Less => {
                println!("Left side is smaller, so inputs are in the right order");
                sum += i + 1;
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                println!("Right side is smaller, so inputs are not in the right order");
            }
        }
        println!();
    }
    sum
}

fn print_depth(depth: usize) {
    for _ in 0..depth {
        print!(" ");
    }
}

fn compare(left: &Packet, right: &Packet, depth: usize) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    use Packet::*;
    print_depth(depth);
    println!(
        "- Compare: {} vs {}",
        packet_string(left),
        packet_string(right)
    );
    match (left, right) {
        (Int(l), Int(r)) => l.cmp(r),
        (List(l), List(r)) => {
            let mut i = 0;
            while i < l.len() && i < r.len() {
                let cmp = compare(&l[i], &r[i], depth + 1);
                if matches!(cmp, Less | Greater) {
                    return cmp;
                }
                i += 1;
            }

            l.len().cmp(&r.len())
        }
        (list, Int(int)) => {
            let right = List(vec![Int(*int)]);
            print_depth(depth + 1);
            println!(
                "- Mixed types; convert right to {} and retry comparison",
                packet_string(&right)
            );
            compare(list, &right, depth + 1)
        }
        (Int(int), list) => {
            let left = List(vec![Int(*int)]);
            print_depth(depth + 1);
            println!(
                "Mixed types; convert left to {} and retry comparison",
                packet_string(&left)
            );
            compare(&left, list, depth + 1)
        }
    }
}

pub fn part_2(input: &Data) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::packet_string;

    const INPUTS: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 0);
    }

    #[test]
    pub fn test_parse() {
        const INPUTS: &str = indoc! {"
            [[1],[2,3,4]]
            [[1],4]
        "};
        let input = super::parse(INPUTS);
        for (left, right) in input {
            println!("{:?}", left);
            println!("{}", packet_string(&left));
            println!("{:?}", right);
            println!("{}", packet_string(&right));
        }
    }
}
