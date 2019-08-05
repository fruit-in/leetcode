use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cnt_row;
        let mut set_row = HashSet::new();
        let mut cnt_col = vec![0; 9];
        let mut set_col = vec![HashSet::new(); 9];
        let mut cnt_sub = vec![0; 3];
        let mut set_sub = vec![HashSet::new(); 3];

        for row in 0..9 {
            cnt_row = 0;
            set_row.clear();
            if row % 3 == 0 {
                for i in 0..3 {
                    cnt_sub[i] = 0;
                    set_sub[i].clear();
                }
            }

            for col in 0..9 {
                if board[row][col] != '.' {
                    cnt_row += 1;
                    set_row.insert(board[row][col]);
                    cnt_col[col] += 1;
                    set_col[col].insert(board[row][col]);
                    cnt_sub[col / 3] += 1;
                    set_sub[col / 3].insert(board[row][col]);
                }

                if col % 3 == 2 {
                    if cnt_sub[col / 3] != set_sub[col / 3].len() {
                        return false;
                    }
                }
                if row == 8 {
                    if cnt_col[col] != set_col[col].len() {
                        return false;
                    }
                }
            }
            if cnt_row != set_row.len() {
                return false;
            }
        }
        true
    }
}
