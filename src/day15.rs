use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde_scan::scan;

type Data = Vec<((i32, i32), (i32, i32))>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| scan!("Sensor at x={}, y={}: closest beacon is at x={}, y={}" <- line).unwrap())
        .collect()
}

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn intervals_in_row(input: &Data, row_target: i32) -> Vec<(i32, i32)> {
    let mut intervals = vec![];
    for (sensor, beacon) in input {
        let d = dist(*sensor, *beacon);
        let dx = d - (sensor.1 - row_target).abs();
        if dx <= 0 {
            continue;
        }
        intervals.push((sensor.0 - dx, sensor.0 + dx));
    }
    merge_intervals(intervals)
}

fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|x| x.0);

    let mut result = vec![];
    let mut current = intervals[0];
    for (start, end) in intervals.iter().skip(1) {
        if *start <= current.1 {
            current.1 = (*end).max(current.1);
            continue;
        } else {
            result.push(current);
            current = (*start, *end);
        }
    }
    result.push(current);
    result
}

fn sum_intervals(intervals: Vec<(i32, i32)>) -> i32 {
    intervals.iter().map(|(start, end)| end - start).sum()
}

fn check_area(input: &Data, max: i32) -> usize {
    let (intervals, y) = (0..=max)
        .into_par_iter()
        .map(|y| (intervals_in_row(input, y), y))
        // if len is > 1 it means there's a gap
        .find_any(|(interval, _)| interval.len() > 1)
        .unwrap();
    let x = intervals[0].1 + 1;
    x as usize * 4000000 + y as usize
}

pub fn part_1(input: &Data) -> usize {
    let intervals = intervals_in_row(input, 2000000);
    sum_intervals(intervals) as usize
}

pub fn part_2(input: &Data) -> usize {
    check_area(input, 4000000)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let intervals = super::intervals_in_row(&input, 10);
        let sum = super::sum_intervals(intervals);
        assert_eq!(sum, 26);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::check_area(&input, 20);
        assert_eq!(result, 56000011);
    }
}
