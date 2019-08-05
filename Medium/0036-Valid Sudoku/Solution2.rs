impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut ans = vec![vec![vec![false; 9]; 3] ;9];
        for row in 0..9 {
            for col in 0..9 {
                let sub = row / 3 * 3 + col / 3;
                if board[row][col] != '.' {
                    let n = board[row][col].to_digit(10).unwrap() as usize - 1;
                    if ans[n][0][row] || ans[n][1][col] || ans[n][2][sub] {
                        return false;
                    }
                    ans[n][0][row] = true;
                    ans[n][1][col] = true;
                    ans[n][2][sub] = true;
                }
            }
        }
        true
    }
}
