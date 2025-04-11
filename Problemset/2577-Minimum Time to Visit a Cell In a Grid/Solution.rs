use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = HashSet::from([(0, 0)]);
        let mut heap = BinaryHeap::from([Reverse((0, 0, 0))]);

        while let Some(Reverse((t, i, j))) = heap.pop() {
            if i == m - 1 && j == n - 1 {
                return t;
            }

            if i > 0 && !visited.contains(&(i - 1, j)) {
                visited.insert((i - 1, j));
                if t + 1 >= grid[i - 1][j] {
                    heap.push(Reverse((t + 1, i - 1, j)));
                } else if t % 2 != grid[i - 1][j] % 2 {
                    heap.push(Reverse((grid[i - 1][j], i - 1, j)));
                } else {
                    heap.push(Reverse((grid[i - 1][j] + 1, i - 1, j)));
                }
            }
            if i < m - 1 && !visited.contains(&(i + 1, j)) {
                visited.insert((i + 1, j));
                if t + 1 >= grid[i + 1][j] {
                    heap.push(Reverse((t + 1, i + 1, j)));
                } else if t % 2 != grid[i + 1][j] % 2 {
                    heap.push(Reverse((grid[i + 1][j], i + 1, j)));
                } else {
                    heap.push(Reverse((grid[i + 1][j] + 1, i + 1, j)));
                }
            }
            if j > 0 && !visited.contains(&(i, j - 1)) {
                visited.insert((i, j - 1));
                if t + 1 >= grid[i][j - 1] {
                    heap.push(Reverse((t + 1, i, j - 1)));
                } else if t % 2 != grid[i][j - 1] % 2 {
                    heap.push(Reverse((grid[i][j - 1], i, j - 1)));
                } else {
                    heap.push(Reverse((grid[i][j - 1] + 1, i, j - 1)));
                }
            }
            if j < n - 1 && !visited.contains(&(i, j + 1)) {
                visited.insert((i, j + 1));
                if t + 1 >= grid[i][j + 1] {
                    heap.push(Reverse((t + 1, i, j + 1)));
                } else if t % 2 != grid[i][j + 1] % 2 {
                    heap.push(Reverse((grid[i][j + 1], i, j + 1)));
                } else {
                    heap.push(Reverse((grid[i][j + 1] + 1, i, j + 1)));
                }
            }
        }

        unreachable!()
    }
}
