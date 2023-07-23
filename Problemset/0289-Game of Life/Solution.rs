impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        let nbs = [(-1, -1), (-1, 0), (-1, 1), (0, -1),
            (0, 1), (1, -1), (1, 0), (1, 1)];

        for i in 0..m {
            for j in 0..n {
                let mut cnt_1 = 0;
                for (r, c) in &nbs {
                    if i >= -r && i + r < m && j >= -c && j + c < n {
                        cnt_1 += board[(i + r) as usize][(j + c) as usize] % 2;
                    }
                }

                match (board[i as usize][j as usize], cnt_1) {
                    (0, 3) => board[i as usize][j as usize] = 2,
                    (1, x) if x < 2 || x > 3 => board[i as usize][j as usize] = 3,
                    _ => (),
                }
            }
        }

        for i in 0..(m as usize) {
            for j in 0..(n as usize) {
                match board[i][j] {
                    2 => board[i][j] = 1,
                    3 => board[i][j] = 0,
                    _ => (),
                }
            }
        }
    }
}
