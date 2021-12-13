pub fn day13() {
    println!("Result 13-1: {}", part1());
    println!("Result 13-2: {}", part2());
}

pub fn part1() -> usize {
    let (board, folds) = get_board_and_folds();
    let board = fold(&board, folds[0]);
    let mut result = 0;
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if board[x][y] {
                result += 1;
            }
        }
    }
    result
}

pub fn part2() -> usize {
    let (mut board, folds) = get_board_and_folds();
    folds.iter().for_each(|f| board = fold(&board, *f));
    board.iter().for_each(|row| {
        let mut str = String::new();
        row.iter()
            .for_each(|b| str.push(if *b { '#' } else { ' ' }));
        println!("{:?}", str);
    });
    0
}

fn fold(board: &[Vec<bool>], fold: Fold) -> Vec<Vec<bool>> {
    match fold {
        Fold::X(column) => {
            let mut output = Vec::with_capacity(board.len());
            board.iter().for_each(|values| {
                let row_values = values[..column]
                    .iter()
                    .zip(values[column + 1..].iter().rev())
                    .map(|(f, l)| *f || *l)
                    .collect();
                output.push(row_values);
            });
            output
        }
        Fold::Y(row) => {
            let mut output = board[0..row].to_owned();
            board
                .iter()
                .skip(row + 1)
                .rev()
                .enumerate()
                .for_each(|(ri, values)| {
                    values
                        .iter()
                        .enumerate()
                        .for_each(|(ci, value)| output[ri][ci] |= *value);
                });
            output
        }
    }
}

fn get_board_and_folds() -> (Vec<Vec<bool>>, Vec<Fold>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut coordinates = Vec::with_capacity(100);
    let mut folds = Vec::with_capacity(10);
    raw_input().lines().for_each(|line| {
        if line.starts_with("fold") {
            // instruction
            let mut relevant = line.chars().skip("fold along ".len());
            let axis = relevant.next().unwrap();
            relevant.next().unwrap();
            let value = {
                let mut multiplier = 1;
                let mut value = 0;
                relevant
                    .collect::<Vec<char>>()
                    .into_iter()
                    .rev()
                    .for_each(|c| {
                        value += super::util::char_to_u8(c).unwrap() as usize * multiplier;
                        multiplier *= 10;
                    });
                value
            };
            if axis == 'x' {
                folds.push(Fold::X(value));
            } else {
                folds.push(Fold::Y(value));
            }
        } else if !line.is_empty() {
            let mut parts = line.split(',');
            let y = parts.next().map(|s| s.parse::<usize>().unwrap()).unwrap();
            let x = parts.next().map(|s| s.parse::<usize>().unwrap()).unwrap();
            max_y = std::cmp::max(max_y, y);
            max_x = std::cmp::max(max_x, x);
            coordinates.push((x, y));
        }
    });
    let mut board = vec![vec![false; max_y + 1]; max_x + 1];
    coordinates
        .into_iter()
        .for_each(|(x, y)| board[x][y] = true);
    (board, folds)
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(usize),
    Y(usize),
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
        assert_eq!(17, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2());
    }
}
