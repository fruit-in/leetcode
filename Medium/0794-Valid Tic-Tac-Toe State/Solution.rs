impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let board = board.concat().into_bytes();
        let x_count = board.iter().filter(|&&c| c == b'X').count();
        let o_count = board.iter().filter(|&&c| c == b'O').count();

        (x_count == o_count + 1 && !Self::is_win(b'O', &board))
            || (x_count == o_count && !Self::is_win(b'X', &board))
    }

    fn is_win(player: u8, board: &[u8]) -> bool {
        [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ]
        .iter()
        .any(|&(x, y, z)| [board[x], board[y], board[z]] == [player, player, player])
    }
}
