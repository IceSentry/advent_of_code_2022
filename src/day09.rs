use glam::Vec2;
use hashbrown::HashSet;
use serde_scan::scan;

type Data = Vec<(Vec2, i32)>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| scan!("{} {}" <- line).expect("Failed to parse input"))
        .map(|(dir, amount)| {
            let dir = match dir {
                'U' => Vec2::NEG_Y,
                'D' => Vec2::Y,
                'L' => Vec2::NEG_X,
                'R' => Vec2::X,
                _ => unreachable!(),
            };
            (dir, amount)
        })
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    let mut head = Vec2::ZERO;
    let mut tail = Vec2::ZERO;
    let mut tail_cache = HashSet::new();
    for (dir, amount) in input {
        for _ in 0..*amount {
            head += *dir;
            tail = follow(tail, head);
            tail_cache.insert((tail.x as i32, tail.y as i32));
        }
    }
    tail_cache.len()
}

pub fn part_2(input: &Data) -> usize {
    let mut head = Vec2::ZERO;
    let mut tail_cache = HashSet::new();
    let mut rope = vec![Vec2::ZERO; 9];
    for (dir, amount) in input {
        for _ in 0..*amount {
            head += *dir;
            rope[0] = follow(rope[0], head);
            for i in 1..rope.len() {
                rope[i] = follow(rope[i], rope[i - 1]);
            }
            let tail = rope.last().unwrap();
            tail_cache.insert((tail.x as i32, tail.y as i32));
        }
    }
    tail_cache.len()
}

fn follow(tail: Vec2, head: Vec2) -> Vec2 {
    let dir = head - tail;
    if dir.length() == 2.0 {
        tail + dir.normalize()
    } else if dir.length() > 2.0 {
        tail + dir.signum()
    } else {
        tail
    }
}

#[allow(unused)]
fn print_cache(size: i32, cache: &HashSet<(i32, i32)>) {
    for y in -size..size {
        for x in -size..size {
            if (x, y) == (0, 0) {
                print!("s");
            } else if cache.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(unused)]
fn print_step(size: i32, head: (i32, i32), rope: &[(i32, i32)]) {
    for y in -size..size {
        for x in -size..size {
            match (x, y) {
                (0, 0) => print!("s"),
                pos if pos == head => print!("H"),
                pos => match rope.iter().position(|i| i == &pos) {
                    Some(i) => print!("{}", i + 1),
                    None => print!("."),
                },
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
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
        assert_eq!(result, 1);

        let large_input = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "};
        let input = super::parse(large_input);
        let result = super::part_2(&input);
        assert_eq!(result, 36);
    }
}
