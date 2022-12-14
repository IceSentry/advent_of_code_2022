use hashbrown::HashSet;

type Data = (HashSet<(usize, usize)>, usize);

pub fn parse(input: &str) -> Data {
    let input: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .flat_map(|x| x.split_once(','))
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut map = HashSet::new();
    let mut max_y = usize::MIN;
    for line in input {
        for window in line.windows(2) {
            let [a, b] = window else { unreachable!(); };
            for y in a.1.min(b.1)..=a.1.max(b.1) {
                for x in a.0.min(b.0)..=a.0.max(b.0) {
                    map.insert((x, y));
                    max_y = max_y.max(y);
                }
            }
        }
    }
    (map, max_y)
}

fn simulate_sand(map: &HashSet<(usize, usize)>, max_y: usize) -> (usize, usize) {
    let sand_start = (500, 0);
    let mut sand = sand_start;
    while sand.1 <= max_y {
        if !map.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
        } else if !map.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
        } else if !map.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.0 += 1;
            sand.1 += 1;
        } else {
            break;
        }
    }
    sand
}

pub fn part_1((map, max_y): &Data) -> usize {
    let mut map = map.clone();
    let mut count = 0;
    loop {
        let sand = simulate_sand(&map, *max_y);
        if sand.1 > *max_y {
            break;
        }
        map.insert(sand);
        count += 1;
    }
    count
}

pub fn part_2((map, max_y): &Data) -> usize {
    let mut map = map.clone();
    let mut count = 0;
    loop {
        let sand = simulate_sand(&map, *max_y);
        map.insert(sand);
        count += 1;
        if sand == (500, 0) {
            break;
        }
    }
    count
}

// std::thread::sleep(std::time::Duration::from_millis(5));
// print_map(&map, *max_y);
#[allow(unused)]
fn print_map(map: &HashSet<(usize, usize)>, max_y: usize) {
    // clear screen and reset cursor at top left
    print!("\x1B[2J\x1B[1;1H");
    println!();
    for y in 0..=max_y + 2 {
        for x in 450..=550 {
            if !map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 24);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 93);
    }
}
