impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut ret = 0;

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X' &&
                (i + 1 == board.len() || board[i + 1][j] == '.') &&
                (j + 1 == board[0].len() || board[i][j + 1] != 'X') {
                    ret += 1;
                }
            }
        }

        ret
    }
}
