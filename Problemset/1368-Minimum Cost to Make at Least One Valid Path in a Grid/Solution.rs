impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let m = grid.len();
        let n = grid[0].len();
        let mut seen = HashSet::new();
        let mut heap = BinaryHeap::from([(0, 0, 0)]);

        while let Some((cost, i0, j0)) = heap.pop() {
            if i0 < 0 || j0 < 0 || i0 >= m || j0 >= n {
                continue;
            }

            if i0 == m - 1 && j0 == n - 1 {
                return -cost;
            }

            seen.insert((i0, j0));

            for (i1, j1, g) in [
                (i0, j0 + 1, 1),
                (i0, j0 - 1, 2),
                (i0 + 1, j0, 3),
                (i0 - 1, j0, 4),
            ] {
                if !seen.contains(&(i1, j1)) {
                    if g == grid[i0 as usize][j0 as usize] {
                        heap.push((cost, i1, j1));
                    } else {
                        heap.push((cost - 1, i1, j1));
                    }
                }
            }
        }

        unreachable!()
    }
}
