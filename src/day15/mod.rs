pub fn day15() {
    println!("Result 15-1: {}", part1());
    println!("Result 15-2: {}", part2());
}

pub fn part1() -> usize {
    let weights = grid();
    astar(weights)
}

pub fn part2() -> usize {
    let weights = grid();
    let weights = expand_grid(weights);
    astar(weights)
}

fn astar(weights: Vec<Vec<u8>>) -> usize {
    let max_y = weights.len() - 1;
    let max_x = weights[0].len() - 1;

    let mut open_nodes = Vec::with_capacity(50);
    open_nodes.push((0, 0));

    let mut scores = vec![vec![usize::MAX; max_x + 1]; max_y + 1];
    scores[0][0] = 0;
    loop {
        let min = open_nodes
            .iter()
            .enumerate()
            .rev()
            .min_by_key(|(_, (x, y))| scores[*x][*y])
            .unwrap()
            .0;
        let head = open_nodes.remove(min);
        if head == (max_y, max_x) {
            return scores[head.0][head.1];
        }
        get_adjacent(head, max_x, max_y)
            .into_iter()
            .for_each(|point| {
                let new_score = scores[head.0][head.1] + weights[point.0][point.1] as usize;
                if scores[point.0][point.1] == usize::MAX {
                    open_nodes.push(point);
                }
                if new_score < scores[point.0][point.1] {
                    scores[point.0][point.1] = new_score;
                }
            });
    }
}

fn get_adjacent(point: (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut output = vec![];
    if point.0 > 0 {
        output.push((point.0 - 1, point.1));
    }
    if point.0 < max_x {
        output.push((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        output.push((point.0, point.1 - 1));
    }
    if point.1 < max_y {
        output.push((point.0, point.1 + 1));
    }
    output
}

fn expand_grid(original: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let height = original.len();
    let width = original[0].len();
    let new_height = height * 5;
    let new_width = width * 5;
    let mut output = vec![vec![0; new_width]; new_height];
    for i in 0..5 {
        for j in 0..5 {
            let value_offset = (i + j) as u8;
            for row in 0..height {
                for col in 0..width {
                    let mut new_value = original[row][col] + value_offset;
                    if new_value > 9 {
                        new_value -= 9;
                    }
                    output[row + (height * i)][col + (width * j)] = new_value;
                }
            }
        }
    }
    output
}

fn grid() -> Vec<Vec<u8>> {
    raw_input()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| super::util::char_to_u8(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(not(test))]
fn raw_input() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
fn raw_input() -> &'static str {
    include_str!("testinput")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(40, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(315, part2());
    }
}
