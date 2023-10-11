use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut cells = vec![];
        let mut pacific = HashSet::new();
        let mut atlantic = HashSet::new();

        for r in 0..m {
            cells.push((r, 0));
            cells.push((r, n - 1));
            pacific.insert((r, 0));
            atlantic.insert((r, n - 1));
        }
        for c in 0..n {
            cells.push((0, c));
            cells.push((m - 1, c));
            pacific.insert((0, c));
            atlantic.insert((m - 1, c));
        }

        while let Some((r0, c0)) = cells.pop() {
            for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let r1 = ((r0 as i32 + x).max(0) as usize).min(m - 1);
                let c1 = ((c0 as i32 + y).max(0) as usize).min(n - 1);

                if heights[r0][c0] > heights[r1][c1] {
                    continue;
                }
                if pacific.contains(&(r0, c0)) && pacific.insert((r1, c1)) {
                    cells.push((r1, c1));
                }
                if atlantic.contains(&(r0, c0)) && atlantic.insert((r1, c1)) {
                    cells.push((r1, c1));
                }
            }
        }

        pacific
            .intersection(&atlantic)
            .map(|&(r, c)| vec![r as i32, c as i32])
            .collect()
    }
}
