pub struct NewBoard {
    pub board: Vec<usize>,
    pub playable_moves: Vec<Vec<usize>>,
    pub move_history: Vec<(usize, usize)>
}

impl NewBoard {
    pub fn new() -> NewBoard {
        return NewBoard { board: vec![], playable_moves: vec![], move_history: vec![] };
    }

    pub fn from_vector(vector: Vec<usize>) -> NewBoard {
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

        return NewBoard { board: vector, playable_moves: possible_moves, move_history: vec![] };
    }

    pub fn board_valid(&mut self) -> bool {
        return true;
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

    pub fn make_move(&mut self, valid_move: (usize, usize)) -> bool {
        if self.board[valid_move.0] != 0 {
            return false;
        }

        self.board[valid_move.0] = valid_move.1;
        self.playable_moves[valid_move.0] = vec![];
        self.move_history.push(valid_move);

        return true;
    }

    pub fn unmake_last_move(&mut self) -> bool {
        if self.move_history.is_empty() {
            return false;
        }

        let valid_move = self.move_history.pop().expect("Move history was empty.");
        self.board[valid_move.0] = 0;
        self.playable_moves[valid_move.0] = (1..10).collect();

        return true;
    }

    pub fn valid_moves(&mut self) -> Vec<(usize, usize)> {
        let mut valid_moves = vec![];

        for possible_move in self.possible_moves() {
            self.make_move(possible_move);
            if self.board_valid() {
                valid_moves.push(possible_move.to_owned());
            }
            self.unmake_last_move();
        }

        return valid_moves;
    }

    fn possible_moves(&self) -> Vec<(usize, usize)> {
        let mut possible_moves = vec![];

        for i in 0..self.board.len() {
            for possible_move in self.playable_moves[i] {
                possible_moves.push((i, possible_move));
            }
        }

        return possible_moves;
    }
}
