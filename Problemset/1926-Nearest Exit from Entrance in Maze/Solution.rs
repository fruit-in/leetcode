use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len() as i32;
        let n = maze[0].len() as i32;
        let mut maze = maze;
        let mut cells = VecDeque::from([(entrance[0], entrance[1], 0)]);
        maze[cells[0].0 as usize][cells[0].1 as usize] = '+';

        while let Some((row, col, steps)) = cells.pop_front() {
            if (row != entrance[0] || col != entrance[1])
                && (row == 0 || col == 0 || row == m - 1 || col == n - 1)
            {
                return steps;
            }

            for (r, c) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let row = row + r;
                let col = col + c;

                if row >= 0
                    && row < m
                    && col >= 0
                    && col < n
                    && maze[row as usize][col as usize] == '.'
                {
                    cells.push_back((row, col, steps + 1));
                    maze[row as usize][col as usize] = '+';
                }
            }
        }

        -1
    }
}
