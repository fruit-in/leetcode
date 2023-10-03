impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let r_move = r_move;
        let c_move = c_move;

        for direct in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let mut r = r_move + direct.0;
            let mut c = c_move + direct.1;

            if r < 0
                || r > 7
                || c < 0
                || c > 7
                || board[r as usize][c as usize] == '.'
                || board[r as usize][c as usize] == color
            {
                continue;
            }

            r += direct.0;
            c += direct.1;

            while r >= 0 && r < 8 && c >= 0 && c < 8 && board[r as usize][c as usize] != '.' {
                if board[r as usize][c as usize] == color {
                    return true;
                }

                r += direct.0;
                c += direct.1;
            }
        }

        false
    }
}
