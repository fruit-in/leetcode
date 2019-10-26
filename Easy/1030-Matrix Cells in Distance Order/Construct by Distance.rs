impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut max_dis = r0.max(r - 1 - r0) + c0.max(c - 1 - c0);
        let mut cells = Vec::new();

        for dis in 0..=max_dis {
            for r_diff in (-dis).max(-r0)..=dis.min(r - 1 - r0) {
                let c_diff = dis - r_diff.abs();

                if c_diff == 0 {
                    cells.push(vec![r0 + r_diff, c0]);
                } else {
                    if c0 - c_diff > -1 {
                        cells.push(vec![r0 + r_diff, c0 - c_diff]);
                    }
                    if c0 + c_diff < c {
                        cells.push(vec![r0 + r_diff, c0 + c_diff]);
                    }
                }
            }
        }

        cells
    }
}
