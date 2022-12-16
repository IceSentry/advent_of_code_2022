use std::collections::VecDeque;
use std::fmt::Write;

type Data = Vec<(Packet, Packet)>;

pub fn parse(input: &str) -> Data {
    input
        .split("\n\n")
        .map(|l| l.lines().map(parse_packet_string).collect::<Vec<_>>())
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect()
}

#[derive(Debug, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

#[allow(unused)]
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

fn parse_packet_string(s: &str) -> Packet {
    let mut c = s.chars().collect::<VecDeque<_>>();
    parse_packet(&mut c)[0].clone()
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

fn compare(left: &Packet, right: &Packet) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    use Packet::*;
    match (left, right) {
        (Int(l), Int(r)) => l.cmp(r),
        (List(l), List(r)) => {
            let mut i = 0;
            while i < l.len() && i < r.len() {
                match compare(&l[i], &r[i]) {
                    cmp @ (Less | Greater) => return cmp,
                    Equal => i += 1,
                }
            }
            l.len().cmp(&r.len())
        }
        (list, Int(int)) => compare(list, &List(vec![Int(*int)])),
        (Int(int), list) => compare(&List(vec![Int(*int)]), list),
    }
}

pub fn part_1(input: &Data) -> usize {
    let mut sum = 0;
    for (i, (left, right)) in input.iter().enumerate() {
        if let std::cmp::Ordering::Less = compare(left, right) {
            sum += i + 1;
        }
    }
    sum
}

pub fn part_2(input: &Data) -> usize {
    let mut packets = vec![];
    let divider_2 = parse_packet_string("[[2]]");
    let divider_6 = parse_packet_string("[[6]]");
    packets.push(divider_2.clone());
    packets.push(divider_6.clone());
    for (left, right) in input.clone() {
        packets.push(left);
        packets.push(right);
    }
    packets.sort_by(compare);
    let mut product = 0;
    for (i, p) in packets.iter().enumerate() {
        if compare(p, &divider_2).is_eq() {
            product = i + 1;
        } else if compare(p, &divider_6).is_eq() {
            product *= i + 1;
        }
    }
    product
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

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
        assert_eq!(result, 140);
    }
}
