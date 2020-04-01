impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let mut clicks = vec![click];
        let h = board.len() as i32;
        let w = board[0].len() as i32;

        while let Some(click) = clicks.pop() {
            let r = click[0];
            let c = click[1];

            match board[r as usize][c as usize] {
                'M' => board[r as usize][c as usize] = 'X',
                'E' => {
                    let mut mines = 0;
                    let mut empties = Vec::new();

                    for i in (r - 1).max(0)..(r + 2).min(h) {
                        for j in (c - 1).max(0)..(c + 2).min(w) {
                            if (i != r || j != c) {
                                match board[i as usize][j as usize] {
                                    'M' => mines += 1,
                                    'E' => empties.push(vec![i, j]),
                                    _ => (),
                                }
                            }
                        }
                    }

                    board[r as usize][c as usize] = match mines {
                        0 => {
                            clicks.append(&mut empties);
                            'B'
                        },
                        m => (m as u8 + b'0') as char,
                    }
                },
                _ => (),
            }
        }

        board
    }
}
