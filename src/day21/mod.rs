pub fn day21() {
    println!("Result 21-1: {}", part1());
    println!("Result 21-2: {}", part2());
}

pub fn part1() -> usize {
    let (mut player1, mut player2) = starting_positions();
    let mut player1_score = 0;
    let mut player2_score = 0;
    let mut dice = DeterministicDice::default();

    loop {
        player1 += dice.roll();
        while player1 > 10 {
            player1 -= 10;
        }
        player1_score += player1;
        if player1_score >= 1000 {
            return dice.roll_count() * player2_score;
        }

        player2 += dice.roll();
        while player2 > 10 {
            player2 -= 10;
        }
        player2_score += player2;
        if player2_score >= 1000 {
            return dice.roll_count() * player1_score;
        }
    }
}

#[derive(Default)]
struct DeterministicDice {
    num_rolls: usize,
    current_value: usize,
}

impl DeterministicDice {
    fn roll(&mut self) -> usize {
        let total_roll: usize = (0..3)
            .map(|_| {
                self.current_value += 1;
                self.current_value
            })
            .sum();
        self.num_rolls += 3;
        total_roll
    }

    fn roll_count(&self) -> usize {
        self.num_rolls
    }
}

pub fn part2() -> u128 {
    let (player1, player2) = starting_positions();
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    process(
        1,
        1,
        player1 as u8,
        player2 as u8,
        0,
        0,
        &mut player1_wins,
        &mut player2_wins,
    );
    std::cmp::max(player1_wins, player2_wins)
}

fn process(
    turn: u8,
    acc_mul: usize,
    position1: u8,
    position2: u8,
    score1: u8,
    score2: u8,
    p1: &mut u128,
    p2: &mut u128,
) {
    roll_options().into_iter().for_each(|(val, mul)| {
        let acc_mul = acc_mul * mul as usize;
        if turn == 1 {
            let mut position = position1 + val;
            if position > 10 {
                position -= 10;
            }
            let score = score1 + position;
            if score >= 21 {
                *p1 += acc_mul as u128;
            } else {
                process(2, acc_mul, position, position2, score, score2, p1, p2);
            }
        } else {
            let mut position = position2 + val;
            if position > 10 {
                position -= 10;
            }
            let score = score2 + position;
            if score >= 21 {
                *p2 += acc_mul as u128;
            } else {
                process(1, acc_mul, position1, position, score1, score, p1, p2);
            }
        }
    });
}

const fn roll_options() -> [(u8, u32); 7] {
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
}

fn starting_positions() -> (usize, usize) {
    let positions = raw_input()
        .lines()
        .map(|line| line.split(' ').last().unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (positions[0], positions[1])
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
        assert_eq!(739785, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(444356092776315, part2());
    }
}
