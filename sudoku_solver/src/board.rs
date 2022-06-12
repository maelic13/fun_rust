pub struct Board {
    pub current_board: Vec<usize>,
    pub move_history: Vec<(usize, usize)>,
    pub original_board: Vec<usize>,
    pub playable_moves: Vec<Vec<usize>>,
}

impl Board {
    pub fn from_vector(vector: Vec<usize>) -> Board {
        if vector.len() != 81 {
            panic!("Wrong board size!");
        }

        let mut possible_moves = vec![];
        for i in 0..vector.len() {
            if vector[i] == 0 {
                possible_moves.push((1..10).collect());
            } else {
                possible_moves.push(vec![])
            }
        }

        return Board { current_board: vector.to_vec(), original_board: vector.to_vec(),
            playable_moves: possible_moves, move_history: vec![] };
    }

    pub fn valid(&self) -> bool {
        return true;
    }

    pub fn solved(&self) -> bool { return  self.valid() && self.filled(); }

    pub fn print(&self) {
        for i in 0..9 {
            print!("|");
            for j in 0..9 {
                let string = if self.current_board[i * 9 + j] != 0 {
                    self.current_board[i * 9 + j].to_string()
                } else {
                    " ".to_string()
                }.to_string();
                print!(" {} |", string);
            }
            print!("\n");
        }
    }

    pub fn make_move(&mut self, valid_move: (usize, usize)) -> bool {
        if self.current_board[valid_move.0] != 0 {
            return false;
        }

        self.current_board[valid_move.0] = valid_move.1;
        self.playable_moves[valid_move.0] = vec![];
        self.move_history.push(valid_move);

        return true;
    }

    pub fn unmake_last_move(&mut self) -> bool {
        if self.move_history.is_empty() {
            return false;
        }

        let valid_move = self.move_history.pop().expect("Move history was empty.");
        self.current_board[valid_move.0] = 0;
        self.playable_moves[valid_move.0] = (1..10).collect();

        return true;
    }

    pub fn valid_moves(&mut self) -> Vec<(usize, usize)> {
        let mut valid_moves = vec![];

        for possible_move in self.possible_moves() {
            self.make_move(possible_move);
            if self.valid() {
                valid_moves.push(possible_move.to_owned());
            }
            self.unmake_last_move();
        }

        return valid_moves;
    }

    fn filled(&self) -> bool {
        return self.unfilled_count() == 0;
    }

    pub fn unfilled_count(&self) -> usize {
        let mut count = 0;
        for position in self.current_board.to_vec() {
            if position == 0 {
                count += 1;
            }
        }

        return count;
    }

    fn possible_moves(&self) -> Vec<(usize, usize)> {
        let mut possible_moves = vec![];

        for i in 0..self.playable_moves.len() {
            for possible_move in self.playable_moves[i].to_vec() {
                possible_moves.push((i, possible_move));
            }
        }

        return possible_moves;
    }
}
