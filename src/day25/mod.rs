pub fn day25() {
    println!("Result 25-1: {}", part1());
    println!("Result 25-2: {}", part2());
}

pub fn part1() -> usize {
    let (mut board, mut easters, mut southers) = input();
    let mut steps = 0;
    let mut updates = vec![];
    loop {
        steps += 1;
        easters.iter_mut().for_each(|(row, col)| {
            let mut new_col = *col + 1;
            if new_col == board[*row].len() {
                new_col = 0;
            }
            if !board[*row][new_col] {
                updates.push(((*row, *col), false));
                updates.push(((*row, new_col), true));
                *col = new_col;
            }
        });
        updates.iter().for_each(|((row, col), occupied)| {
            board[*row][*col] = *occupied;
        });
        let east_updates = updates.len();
        updates.clear();
        southers.iter_mut().for_each(|(row, col)| {
            let mut new_row = *row + 1;
            if new_row == board.len() {
                new_row = 0;
            }
            if !board[new_row][*col] {
                updates.push(((*row, *col), false));
                updates.push(((new_row, *col), true));
                *row = new_row;
            }
        });
        updates.iter().for_each(|((row, col), occupied)| {
            board[*row][*col] = *occupied;
        });
        let south_updates = updates.len();
        updates.clear();
        if east_updates + south_updates == 0 {
            return steps;
        }
    }
}

pub fn part2() -> usize {
    raw_input().len()
}

fn input() -> (Vec<Vec<bool>>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut easters = vec![];
    let mut southers = vec![];
    let board = raw_input()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '.' {
                        false
                    } else {
                        if c == '>' {
                            easters.push((i, j));
                        } else {
                            southers.push((i, j));
                        }
                        true
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (board, easters, southers)
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
        assert_eq!(58, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2());
    }
}
