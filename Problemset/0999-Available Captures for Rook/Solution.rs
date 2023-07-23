impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut rook = (0, 0);
        for i in 0..8 {
            if let Some(j) = board[i].iter().position(|&c| c == 'R') {
                rook = (i, j);
                break;
            }
        }

        let mut flag = [0, 0, 0, 0];
        for i in 0..rook.0 {
            match board[i][rook.1] {
                'p' => flag[0] = 1,
                'B' => flag[0] = 0,
                _ => (),
            }
        }
        for i in ((rook.0 + 1)..8).rev() {
            match board[i][rook.1] {
                'p' => flag[1] = 1,
                'B' => flag[1] = 0,
                _ => (),
            }
        }
        for i in 0..rook.1 {
            match board[rook.0][i] {
                'p' => flag[2] = 1,
                'B' => flag[2] = 0,
                _ => (),
            }
        }
        for i in ((rook.1 + 1)..8).rev() {
            match board[rook.0][i] {
                'p' => flag[3] = 1,
                'B' => flag[3] = 0,
                _ => (),
            }
        }

        flag.iter().sum()
    }
}
