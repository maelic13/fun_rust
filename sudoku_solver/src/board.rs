use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Move {
    pub column: usize,
    pub row: usize,
    pub number: usize
}

impl Move {
    pub fn index(&self) -> usize {
        return self.row * 9 + self.column;
    }

    pub fn from_index(index: usize, number: usize) -> Self {
        return Move{ column: index % 9, row: index / 9, number }
    }

    pub fn print(&self) {
        println!("Move[{}, {}] = {}.", self.row, self.column, self.number);
    }
}

pub struct Board {
    pub board: Vec<usize>,
    pub move_history: Vec<Move>,
    pub solved: bool,
}

impl Board {
    pub fn new() -> Board {
        return Board { board: vec![0; 81], move_history: vec![], solved: false };
    }

    pub fn from_vector(vector: Vec<usize>) -> Board {
        return Board { board: vector, move_history: vec![], solved: false };
    }

    pub fn solved(&self) -> bool {
        return self.all_filled() && self.valid();
    }

    pub fn valid(&self) -> bool {
        return self.all_rows_valid() && self.all_columns_valid() && self.all_sectors_valid()
    }

    pub fn make_move(&mut self, sudoku_move: Move) -> bool {
        if self.board[sudoku_move.index()] != 0 {
            return false;
        }

        self.move_history.push(sudoku_move);
        self.board[sudoku_move.index()] = sudoku_move.number.to_owned();
        return true;
    }

    pub fn unmake_last_move(&mut self) -> bool {
        if self.move_history.is_empty() {
            return false;
        }

        let last_move = self.move_history.pop().expect("Move history was empty.");
        self.board[last_move.index()] = 0;
        return true;
    }

    // pub fn sorted_valid_moves(&mut self) -> Vec<Move> {
    //     let mut valid_moves = self.valid_moves();
    //     let sorted_valid_moves = vec![];
    //
    //     let mut moves_counter: Vec<usize> = vec![0; 81];
    //     for valid_move in valid_moves {
    //         moves_counter[valid_move.index()] += 1;
    //     }
    //     let mut sorted_moves_counter = moves_counter.to_vec();
    //     sorted_moves_counter.sort();
    //
    //     valid_moves.sort_by(|a, b| moves_counter[b.index()].cmp(&moves_counter[a.index()]));
    //
    //     return sorted_valid_moves;
    // }

    pub fn valid_moves(&mut self) -> Vec<Move> {
        let possible_moves = self.all_possible_moves();
        let mut valid_moves = vec![];

        for tested_move in possible_moves {
            self.make_move(tested_move);
            if self.valid() {
                valid_moves.push(tested_move);
            }
            self.unmake_last_move();
        }
        return valid_moves;
    }

    pub fn all_possible_moves(&self) -> Vec<Move> {
        let mut possible_moves = vec![];

        for index in 0..81 {
            if self.board[index] == 0 {
                continue;
            }
            for i in 1..10 {
                possible_moves.push(Move::from_index(index, i));
            }
        }
        return possible_moves;
    }

    pub fn count_unfilled(&self) -> usize {
        return self.board.iter().filter(|&n| *n == 0).count();
    }

    pub fn print(&self) {
        for i in 0..9 {
            print!("|");
            for j in 0..9 {
                let string = if self.board[i * 9 + j] != 0 {
                    self.board[i * 9 + j].to_string()
                } else {
                    " ".to_string()
                }.to_string();
                print!(" {} |", string);
            }
            print!("\n");
        }
    }

    fn all_filled(&self) -> bool {
        return self.count_unfilled() == 0;
    }

    fn part_valid(&self, part: Vec<usize>) -> bool {
        let mut part_dedup = part.to_vec();
        part_dedup.sort();
        part_dedup.dedup();
        return part.iter().filter(|&n| *n != 0).count()
            == part_dedup.iter().filter(|&n| *n != 0).count();
    }

    fn all_columns_valid(&self) -> bool {
        let mut valid = true;
        for i in 0..9 {
            valid = valid && self.part_valid(self.column(i));
        }
        return valid;
    }

    fn all_rows_valid(&self) -> bool {
        let mut valid = true;
        for i in 0..9 {
            valid = valid && self.part_valid(self.row(i));
        }
        return valid;
    }

    fn all_sectors_valid(&self) -> bool {
        let mut valid = true;
        for i in 0..9 {
            valid = valid && self.part_valid(self.sector(i));
        }
        return valid;
    }

    fn column(&self, column_index: usize) -> Vec<usize> {
        let mut column: Vec<usize> = Vec::new();
        for i in 0..9 {
            column.push(self.board[i * 9 + column_index]);
        }
        return column;
    }

    fn row(&self, row_index: usize) -> Vec<usize> {
        let mut row: Vec<usize> = Vec::new();
        for i in 0..9 {
            row.push(self.board[row_index * 9 + i]);
        }
        return row;
    }

    fn sector(&self, sector_index: usize) -> Vec<usize> {
        let mut sector = Vec::new();

        let row_x = sector_index / 3 * 3;
        let column_y = sector_index % 3 * 3;

        for i in row_x .. row_x + 3 {
            for j in column_y .. column_y + 3 {
                sector.push(self.board[i * 9 + j]);
            }
        }
        return sector;
    }
}
