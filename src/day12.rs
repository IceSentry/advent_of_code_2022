use std::collections::BinaryHeap;

use hashbrown::{HashMap, HashSet};

// (map, start, end)
type Data = (HashMap<(i32, i32), i32>, (i32, i32), (i32, i32));

pub fn parse(input: &str) -> Data {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, chars) in chars.iter().enumerate() {
        for (x, c) in chars.iter().enumerate() {
            let point = (x as i32, y as i32);
            let val = match c {
                'S' => {
                    start = point;
                    'a'
                }
                'E' => {
                    end = point;
                    'z'
                }
                c => *c,
            };
            map.insert(point, val as i32 - 'a' as i32);
        }
    }
    (map, start, end)
}

// Djikstra algorithm
fn shortest_path(
    map: &HashMap<(i32, i32), i32>,
    start: (i32, i32),
    end: (i32, i32),
) -> Option<i32> {
    let mut visited = HashSet::new();
    // This is a max heap, but we want a min heap, so steps should be negative
    let mut heap = BinaryHeap::new();
    heap.push((0, start));
    while let Some((steps, (x, y))) = heap.pop() {
        if !visited.insert((x, y)) {
            continue;
        }
        if (x, y) == end {
            return Some(-steps);
        }
        let curr_height = map.get(&(x, y)).unwrap();
        let neighbours = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (n_x, n_y) in neighbours {
            let n = (x + n_x, y + n_y);
            let Some(n_height) = map.get(&n) else {
                continue;
            };
            if *n_height <= curr_height + 1 {
                heap.push((steps - 1, n));
            }
        }
    }
    None
}

pub fn part_1((map, start, end): &Data) -> usize {
    shortest_path(map, *start, *end).unwrap() as usize
}

pub fn part_2((map, _start, end): &Data) -> usize {
    map.iter()
        .filter(|(_, value)| **value == 0)
        .flat_map(|(point, _)| shortest_path(map, *point, *end))
        .min()
        .unwrap() as usize
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 31);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 29);
    }
}
