use board::Board;

mod board;

fn main() {
    let board = Board::from_vector(vec![
        6, 0, 0, 0, 7, 1, 4, 0, 0, 1, 8, 5, 0, 0, 9, 2, 0, 0, 0, 4, 0, 2, 5, 0, 9, 0, 8, 5, 0, 8,
        0, 3, 0, 0, 0, 4, 0, 7, 3, 0, 0, 0, 6, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 9, 1, 0,
        0, 7, 6, 8, 0, 0, 0, 0, 7, 1, 0, 9, 7, 1, 0, 6, 8, 3, 0, 0, 0,
    ]);
    board.print();
    println!();

    let result = solve_board(board);
    result.print();
    println!("Validity: {}.", result.valid())
}

fn solve_board(mut board: Board) -> Board {
    for valid_move in board.valid_moves() {
        board.make_move(valid_move);
        board = solve_board(board);
        if board.solved() {
            return board;
        }
        board.unmake_last_move();
    }

    board
}
