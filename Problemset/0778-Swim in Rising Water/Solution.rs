use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut heap = BinaryHeap::from([(-grid[0][0], 0, 0)]);
        let mut visited = HashSet::new();

        while let Some((t, i, j)) = heap.pop() {
            if i == n - 1 && j == n - 1 {
                return -t;
            }

            if visited.contains(&(i, j)) {
                continue;
            }

            visited.insert((i, j));

            if i > 0 && !visited.contains(&(i - 1, j)) {
                heap.push((t.min(-grid[i - 1][j]), i - 1, j));
            }
            if i < n - 1 && !visited.contains(&(i + 1, j)) {
                heap.push((t.min(-grid[i + 1][j]), i + 1, j));
            }
            if j > 0 && !visited.contains(&(i, j - 1)) {
                heap.push((t.min(-grid[i][j - 1]), i, j - 1));
            }
            if j < n - 1 && !visited.contains(&(i, j + 1)) {
                heap.push((t.min(-grid[i][j + 1]), i, j + 1));
            }
        }

        unreachable!()
    }
}
