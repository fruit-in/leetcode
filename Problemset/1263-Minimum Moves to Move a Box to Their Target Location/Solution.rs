use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited_player = HashSet::new();
        let mut stack_player = vec![];
        let mut visited_box = HashSet::new();
        let mut deque_box = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 'S' {
                    visited_player.insert((i, j));
                    stack_player.push((i, j));
                    break;
                }
            }
        }

        while let Some((i, j)) = stack_player.pop() {
            if i > 0 && grid[i - 1][j] == 'B' && !visited_box.contains(&(i - 1, j, 'U')) {
                visited_box.insert((i - 1, j, 'U'));
                deque_box.push_back((i - 1, j, 'U', 0));
            }
            if i < m - 1 && grid[i + 1][j] == 'B' && !visited_box.contains(&(i + 1, j, 'D')) {
                visited_box.insert((i + 1, j, 'D'));
                deque_box.push_back((i + 1, j, 'D', 0));
            }
            if j > 0 && grid[i][j - 1] == 'B' && !visited_box.contains(&(i, j - 1, 'L')) {
                visited_box.insert((i, j - 1, 'L'));
                deque_box.push_back((i, j - 1, 'L', 0));
            }
            if j < n - 1 && grid[i][j + 1] == 'B' && !visited_box.contains(&(i, j + 1, 'R')) {
                visited_box.insert((i, j + 1, 'R'));
                deque_box.push_back((i, j + 1, 'R', 0));
            }

            if i > 0
                && grid[i - 1][j] != '#'
                && grid[i - 1][j] != 'B'
                && !visited_player.contains(&(i - 1, j))
            {
                visited_player.insert((i - 1, j));
                stack_player.push((i - 1, j));
            }
            if i < m - 1
                && grid[i + 1][j] != '#'
                && grid[i + 1][j] != 'B'
                && !visited_player.contains(&(i + 1, j))
            {
                visited_player.insert((i + 1, j));
                stack_player.push((i + 1, j));
            }
            if j > 0
                && grid[i][j - 1] != '#'
                && grid[i][j - 1] != 'B'
                && !visited_player.contains(&(i, j - 1))
            {
                visited_player.insert((i, j - 1));
                stack_player.push((i, j - 1));
            }
            if j < n - 1
                && grid[i][j + 1] != '#'
                && grid[i][j + 1] != 'B'
                && !visited_player.contains(&(i, j + 1))
            {
                visited_player.insert((i, j + 1));
                stack_player.push((i, j + 1));
            }
        }

        while let Some((i, j, s, pushes)) = deque_box.pop_front() {
            if grid[i][j] == 'T' {
                return pushes;
            }

            let mut bi = i;
            let mut bj = j;

            if s == 'U' && i > 0 && grid[i - 1][j] != '#' {
                bi -= 1;
            } else if s == 'D' && i < m - 1 && grid[i + 1][j] != '#' {
                bi += 1;
            } else if s == 'L' && j > 0 && grid[i][j - 1] != '#' {
                bj -= 1;
            } else if s == 'R' && j < n - 1 && grid[i][j + 1] != '#' {
                bj += 1;
            } else {
                continue;
            }

            visited_player = HashSet::from([(i, j)]);
            stack_player = vec![(i, j)];

            while let Some((i, j)) = stack_player.pop() {
                if i > 0 && i - 1 == bi && j == bj && !visited_box.contains(&(bi, bj, 'U')) {
                    visited_box.insert((bi, bj, 'U'));
                    deque_box.push_back((bi, bj, 'U', pushes + 1));
                }
                if i < m - 1 && i + 1 == bi && j == bj && !visited_box.contains(&(bi, bj, 'D')) {
                    visited_box.insert((bi, bj, 'D'));
                    deque_box.push_back((bi, bj, 'D', pushes + 1));
                }
                if j > 0 && i == bi && j - 1 == bj && !visited_box.contains(&(bi, bj, 'L')) {
                    visited_box.insert((bi, bj, 'L'));
                    deque_box.push_back((bi, bj, 'L', pushes + 1));
                }
                if j < n - 1 && i == bi && j + 1 == bj && !visited_box.contains(&(bi, bj, 'R')) {
                    visited_box.insert((bi, bj, 'R'));
                    deque_box.push_back((bi, bj, 'R', pushes + 1));
                }

                if i > 0
                    && grid[i - 1][j] != '#'
                    && (i - 1 != bi || j != bj)
                    && !visited_player.contains(&(i - 1, j))
                {
                    visited_player.insert((i - 1, j));
                    stack_player.push((i - 1, j));
                }
                if i < m - 1
                    && grid[i + 1][j] != '#'
                    && (i + 1 != bi || j != bj)
                    && !visited_player.contains(&(i + 1, j))
                {
                    visited_player.insert((i + 1, j));
                    stack_player.push((i + 1, j));
                }
                if j > 0
                    && grid[i][j - 1] != '#'
                    && (i != bi || j - 1 != bj)
                    && !visited_player.contains(&(i, j - 1))
                {
                    visited_player.insert((i, j - 1));
                    stack_player.push((i, j - 1));
                }
                if j < n - 1
                    && grid[i][j + 1] != '#'
                    && (i != bi || j + 1 != bj)
                    && !visited_player.contains(&(i, j + 1))
                {
                    visited_player.insert((i, j + 1));
                    stack_player.push((i, j + 1));
                }
            }
        }

        -1
    }
}
