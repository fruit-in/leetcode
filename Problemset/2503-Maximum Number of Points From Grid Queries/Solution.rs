impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let (m, n) = (grid.len(), grid[0].len());
        let mut heap = BinaryHeap::from([(-grid[0][0], 0, 0)]);
        let mut visited = HashSet::from([(0, 0)]);
        let mut indices = (0..queries.len()).collect::<Vec<_>>();
        let mut points = 0;
        let mut answer = vec![0; queries.len()];

        indices.sort_unstable_by_key(|&i| queries[i]);

        for &i in &indices {
            while queries[i] > -heap.peek().unwrap_or(&(-1000000, 0, 0)).0 {
                let (_, i, j) = heap.pop().unwrap();

                if i > 0 && !visited.contains(&(i - 1, j)) {
                    heap.push((-grid[i - 1][j], i - 1, j));
                    visited.insert((i - 1, j));
                }
                if i < m - 1 && !visited.contains(&(i + 1, j)) {
                    heap.push((-grid[i + 1][j], i + 1, j));
                    visited.insert((i + 1, j));
                }
                if j > 0 && !visited.contains(&(i, j - 1)) {
                    heap.push((-grid[i][j - 1], i, j - 1));
                    visited.insert((i, j - 1));
                }
                if j < n - 1 && !visited.contains(&(i, j + 1)) {
                    heap.push((-grid[i][j + 1], i, j + 1));
                    visited.insert((i, j + 1));
                }

                points += 1;
            }

            answer[i] = points;
        }

        answer
    }
}
