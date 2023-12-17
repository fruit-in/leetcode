use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::from([[
            board[0][0],
            board[0][1],
            board[0][2],
            board[1][0],
            board[1][1],
            board[1][2],
        ]]);
        let mut moves_required = HashMap::from([(queue[0], 0)]);

        while let Some(board) = queue.pop_front() {
            if board == [1, 2, 3, 4, 5, 0] {
                return moves_required[&board];
            }

            let mut boards = vec![];
            let mut tmp = board;

            if board[0] == 0 {
                tmp.swap(0, 1);
                boards.push(tmp);
                tmp = board;
                tmp.swap(0, 3);
                boards.push(tmp);
            } else if board[1] == 0 {
                tmp.swap(1, 0);
                boards.push(tmp);
                tmp = board;
                tmp.swap(1, 2);
                boards.push(tmp);
                tmp = board;
                tmp.swap(1, 4);
                boards.push(tmp);
            } else if board[2] == 0 {
                tmp.swap(2, 1);
                boards.push(tmp);
                tmp = board;
                tmp.swap(2, 5);
                boards.push(tmp);
            } else if board[3] == 0 {
                tmp.swap(3, 0);
                boards.push(tmp);
                tmp = board;
                tmp.swap(3, 4);
                boards.push(tmp);
            } else if board[4] == 0 {
                tmp.swap(4, 1);
                boards.push(tmp);
                tmp = board;
                tmp.swap(4, 3);
                boards.push(tmp);
                tmp = board;
                tmp.swap(4, 5);
                boards.push(tmp);
            } else {
                tmp.swap(5, 2);
                boards.push(tmp);
                tmp = board;
                tmp.swap(5, 4);
                boards.push(tmp);
            }

            for i in 0..boards.len() {
                if !moves_required.contains_key(&boards[i]) {
                    queue.push_back(boards[i]);
                    moves_required.insert(boards[i], moves_required[&board] + 1);
                }
            }
        }

        -1
    }
}
