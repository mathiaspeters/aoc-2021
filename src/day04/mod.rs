pub fn day4() {
    println!("Result  4-1: {}", part1());
    println!("Result  4-2: {}", part2());
}

pub fn part1() -> usize {
    let (draws, mut boards) = input();
    draws
        .iter()
        .find_map(|draw| boards.iter_mut().find_map(|board| board.mark_number(*draw)))
        .unwrap() as usize
}

pub fn part2() -> usize {
    let (draws, mut boards) = input();
    let mut draws = draws.iter();
    while boards.len() > 1 {
        let next_number = *draws.next().unwrap();
        boards = boards
            .iter_mut()
            .filter_map(|board| {
                if board.mark_number(next_number).is_none() {
                    Some(*board)
                } else {
                    None
                }
            })
            .collect::<Vec<BingoBoard>>();
    }
    let mut last_board = boards[0];
    draws
        .find_map(|draw| last_board.mark_number(*draw))
        .unwrap() as usize
}

fn input() -> (Vec<u32>, Vec<BingoBoard>) {
    let input = raw_input()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let draws = input[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let boards = input[1..]
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .chunks(25)
        .map(|chunk| BingoBoard::new(chunk))
        .collect::<Vec<_>>();
    (draws, boards)
}

#[derive(Copy, Clone, Debug)]
struct Number {
    value: u32,
    is_marked: bool,
}

impl Number {
    pub fn new() -> Self {
        Self {
            value: 0,
            is_marked: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct BingoBoard {
    rows: [[Number; 5]; 5],
}

impl BingoBoard {
    pub fn new(numbers: &[u32]) -> Self {
        let mut rows = [[Number::new(); 5]; 5];
        let mut row = 0;
        let mut col = 0;
        for i in 0..25 {
            rows[row][col].value = numbers[i];
            col += 1;
            if col >= 5 {
                row += 1;
                col = 0;
            }
        }
        Self { rows }
    }

    pub fn mark_number(&mut self, value: u32) -> Option<u32> {
        'outer: for row in 0..5 {
            for col in 0..5 {
                if self.rows[row][col].value == value {
                    self.rows[row][col].is_marked = true;
                    break 'outer;
                }
            }
        }
        self.bingo().map(|sum| sum * value)
    }

    fn bingo(&self) -> Option<u32> {
        let mut unmarked = vec![];
        let mut bingo = false;
        for i in 0..5 {
            let mut marked_in_row = 0;
            let mut marked_in_col = 0;
            for j in 0..5 {
                if !self.rows[i][j].is_marked {
                    unmarked.push(self.rows[i][j].value);
                }
                if !bingo {
                    if self.rows[i][j].is_marked {
                        marked_in_row += 1;
                    }
                    if self.rows[j][i].is_marked {
                        marked_in_col += 1;
                    }
                }
            }
            if marked_in_row >= 5 || marked_in_col >= 5 {
                bingo = true;
            }
        }
        if bingo {
            Some(unmarked.iter().sum())
        } else {
            None
        }
    }
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
        assert_eq!(4512, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1924, part2());
    }
}
