pub fn day23() {
    // 11608
    println!("Result 23-1: {}", part1());
    // 46754
    println!("Result 23-2: {}", part2());
}

pub fn part1() -> usize {
    process(raw_input_1())
}

pub fn part2() -> usize {
    let mut planner = Planner {
        best_score: usize::MAX,
    };
    planner.process(raw_input_2(), 0);
    planner.best_score
}

fn process(initial_board: Board) -> usize {
    let mut closed = vec![];
    let mut open = vec![(initial_board, 0)];
    while !open.is_empty() {
        let min = open
            .iter()
            .enumerate()
            .min_by_key(|(_, (_, s))| *s)
            .unwrap()
            .0;
        let (board, score) = open.remove(min);
        if board.check_done() {
            return score;
        }
        board.possible_moves().into_iter().for_each(|(src, dest)| {
            let mut nb = board.clone();
            let cost = nb.make_move(src, dest);
            let new_score = score + cost;
            if !closed.contains(&nb) {
                if open
                    .iter_mut()
                    .find_map(|(b, s)| {
                        if *b == nb {
                            if new_score < *s {
                                *s = new_score;
                            }
                            Some(())
                        } else {
                            None
                        }
                    })
                    .is_none()
                {
                    open.push((nb, new_score));
                }
            }
        });
        closed.push(board);
    }
    panic!("Couldn't find anything")
}

struct Planner {
    best_score: usize,
}

impl Planner {
    fn process(&mut self, board: Board, score: usize) {
        let moves = board.possible_moves();
        moves.into_iter().for_each(|(src, dest)| {
            let mut nb = board.clone();
            let new_score = score + nb.make_move(src, dest);
            if nb.check_done() {
                if new_score < self.best_score {
                    self.best_score = new_score;
                }
            } else {
                self.process(nb, new_score);
            }
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Board {
    corridor: Corridor,
    rooms: [Room; 4],
}

const ROOM_OFFSETS: [usize; 4] = [2, 4, 6, 8];

impl Board {
    fn check_done(&self) -> bool {
        (0..4).fold(true, |acc, i| {
            acc && self.rooms[i].amphipods[0..self.rooms[i].size]
                .iter()
                .all(|a| matches!(a, Some(j) if *j == i as u8))
        })
    }

    fn make_move(&mut self, src: (usize, usize), dest: (usize, usize)) -> usize {
        let a = if src.0 == 4 {
            let t = self.corridor.tiles[src.1];
            self.corridor.tiles[src.1] = None;
            t.unwrap()
        } else {
            let a = self.rooms[src.0].amphipods[src.1].unwrap();
            self.rooms[src.0].amphipods[src.1] = None;
            a
        };
        if dest.0 == 4 {
            self.corridor.tiles[dest.1] = Some(a);
        } else {
            self.rooms[dest.0].amphipods[dest.1] = Some(a);
        }
        fn abs_diff(a: usize, b: usize) -> usize {
            std::cmp::max(a, b) - std::cmp::min(a, b)
        }
        let steps = if src.0 < 4 && dest.0 < 4 {
            // room to room
            let src_room = src.1 + 1;
            let corridor = abs_diff(ROOM_OFFSETS[src.0], ROOM_OFFSETS[dest.0]);
            let dest_room = dest.1 + 1;
            src_room + corridor + dest_room
        } else if src.0 < 4 {
            // room to corridor
            let src_room = src.1 + 1;
            let corridor = abs_diff(ROOM_OFFSETS[src.0], dest.1);
            src_room + corridor
        } else {
            // corridor to room
            let corridor = abs_diff(src.1, ROOM_OFFSETS[dest.0]);
            let dest_room = dest.1 + 1;
            corridor + dest_room
        };
        let cost = match a {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1000,
            _ => panic!(""),
        };
        steps * cost
    }

    fn possible_moves(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut moves = vec![];
        moves.extend_from_slice(&self.possible_moves_from_corridor());
        (0..4).for_each(|room| {
            moves.extend_from_slice(&self.possible_moves_from_room(room));
        });
        moves
    }

    fn possible_moves_from_corridor(&self) -> Vec<((usize, usize), (usize, usize))> {
        self.corridor
            .occupied_spaces()
            .into_iter()
            .filter_map(|i| match self.corridor.tiles[i] {
                Some(a) if !ROOM_OFFSETS.contains(&i) => {
                    if self
                        .corridor
                        .free_spaces(i)
                        .contains(&ROOM_OFFSETS[a as usize])
                    {
                        match self.rooms[a as usize].next_fill_slot() {
                            Some(fs) => {
                                let src = (4, i);
                                let dest = (a as usize, fs);
                                Some((src, dest))
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    fn possible_moves_from_room(&self, room: usize) -> Vec<((usize, usize), (usize, usize))> {
        match self.rooms[room].next_mover() {
            Some(i) => {
                let a = self.rooms[room].amphipods[i].unwrap();
                let src = (room, i);
                let clear_path_to_room = self
                    .corridor
                    .free_spaces(ROOM_OFFSETS[room])
                    .contains(&ROOM_OFFSETS[a as usize]);
                match self.rooms[a as usize].next_fill_slot() {
                    Some(fs) if clear_path_to_room => {
                        let dest = (a as usize, fs);
                        vec![(src, dest)]
                    }
                    _ => self
                        .corridor
                        .free_spaces(ROOM_OFFSETS[room])
                        .into_iter()
                        .filter_map(|t| {
                            if ROOM_OFFSETS.contains(&t) {
                                None
                            } else {
                                Some((src, (4, t)))
                            }
                        })
                        .collect::<Vec<_>>(),
                }
            }
            None => vec![],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Room {
    room_num: usize,
    amphipods: [Option<u8>; 4],
    size: usize,
}

impl Room {
    fn new(amphis: &str, room_num: usize) -> Self {
        let mut amphipods = [None, None, None, None];
        amphis
            .chars()
            .enumerate()
            .for_each(|(i, c)| amphipods[i] = Some(Self::char_to_amphipod(c)));
        Self {
            room_num,
            amphipods,
            size: amphis.len(),
        }
    }

    fn next_fill_slot(&self) -> Option<usize> {
        if self.amphipods[0..self.size]
            .iter()
            .any(|a| matches!(a, Some(am) if *am as usize != self.room_num))
        {
            return None;
        }
        self.amphipods[0..self.size]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, a)| if a.is_none() { Some(i) } else { None })
    }

    fn next_mover(&self) -> Option<usize> {
        self.amphipods[0..self.size - self.correct()]
            .iter()
            .enumerate()
            .find_map(|(i, a)| a.map(|_| i))
    }

    fn correct(&self) -> usize {
        let mut result = 0;
        for i in (0..self.size).rev() {
            if matches!(self.amphipods[i], Some(am) if am == self.room_num as u8) {
                result += 1;
            } else {
                return result;
            }
        }
        result
    }

    fn char_to_amphipod(c: char) -> u8 {
        match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => panic!("Unknown amphipod"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Corridor {
    tiles: [Option<u8>; 11],
}

impl Corridor {
    fn new() -> Self {
        Self {
            tiles: [
                None, None, None, None, None, None, None, None, None, None, None,
            ],
        }
    }

    fn occupied_spaces(&self) -> Vec<usize> {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, t)| t.map(|_| i))
            .collect::<Vec<_>>()
    }

    fn free_spaces(&self, starting_point: usize) -> Vec<usize> {
        let mut spaces = vec![];
        let itiles = self.tiles.iter().enumerate().collect::<Vec<_>>();
        itiles[0..starting_point]
            .iter()
            .rev()
            .take_while(|(_, t)| t.is_none())
            .for_each(|(i, _)| {
                spaces.push(*i);
            });
        itiles[starting_point + 1..]
            .iter()
            .take_while(|(_, t)| t.is_none())
            .for_each(|(i, _)| {
                spaces.push(*i);
            });
        spaces
    }
}

#[cfg(not(test))]
fn raw_input_1() -> Board {
    init_board(["BC", "BA", "DD", "AC"])
}

#[cfg(test)]
fn raw_input_1() -> Board {
    init_board(["BA", "CD", "BC", "DA"])
}

#[cfg(not(test))]
fn raw_input_2() -> Board {
    init_board(["BDDC", "BCBA", "DBAD", "AACC"])
}

#[cfg(test)]
fn raw_input_2() -> Board {
    init_board(["BDDA", "CCBD", "BBAC", "DACA"])
}

fn init_board(in_rooms: [&str; 4]) -> Board {
    let corridor = Corridor::new();
    let rooms = [
        Room::new(in_rooms[0], 0),
        Room::new(in_rooms[1], 1),
        Room::new(in_rooms[2], 2),
        Room::new(in_rooms[3], 3),
    ];
    Board { corridor, rooms }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(12521, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(44169, part2());
    }
}
