type Data = Vec<Vec<usize>>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|l| {
            l.parse::<String>()
                .unwrap()
                .chars()
                .map(|c| c as usize - 48)
                .collect()
        })
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    let dimensions = (input[0].len(), input.len());
    let (width, height) = dimensions;
    let mut count = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if check_vis(input, &dimensions, (x, y), (-1, 0))
                || check_vis(input, &dimensions, (x, y), (1, 0))
                || check_vis(input, &dimensions, (x, y), (0, -1))
                || check_vis(input, &dimensions, (x, y), (0, 1))
            {
                count += 1;
                continue;
            }
        }
    }
    count + (((width - 2) + (height - 2)) * 2 + 4)
}

pub fn part_2(input: &Data) -> usize {
    let dimensions = (input[0].len(), input.len());
    let (width, height) = dimensions;
    let mut max_score = 0;
    for y in 0..height {
        for x in 0..width {
            let score = scenic_score(input, &dimensions, (x, y), (0, -1))
                * scenic_score(input, &dimensions, (x, y), (-1, 0))
                * scenic_score(input, &dimensions, (x, y), (0, 1))
                * scenic_score(input, &dimensions, (x, y), (1, 0));
            max_score = max_score.max(score);
        }
    }
    max_score
}

fn check_vis(
    input: &Data,
    (width, height): &(usize, usize),
    (x, y): (usize, usize),
    dir: (i32, i32),
) -> bool {
    let curr_tree = input[y][x];
    let mut x = x as i32;
    let mut y = y as i32;
    let mut visible = true;
    loop {
        x += dir.0;
        y += dir.1;

        if x == -1 || x >= *width as i32 {
            break;
        }
        if y == -1 || y >= *height as i32 {
            break;
        }

        if input[y as usize][x as usize] >= curr_tree {
            visible = false;
            break;
        }
    }
    visible
}

fn scenic_score(
    input: &Data,
    (width, height): &(usize, usize),
    (x, y): (usize, usize),
    dir: (i32, i32),
) -> usize {
    let curr_tree = input[y][x];
    let mut x = x as i32;
    let mut y = y as i32;
    let mut score = 0;
    loop {
        x += dir.0;
        y += dir.1;

        if x == -1 || x >= *width as i32 {
            break;
        }
        if y == -1 || y >= *height as i32 {
            break;
        }

        let check = input[y as usize][x as usize];
        if check < curr_tree {
            score += 1;
        }

        if check >= curr_tree {
            score += 1;
            break;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 8);
    }
}
