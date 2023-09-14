use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..grid.len() {
            count
                .entry(grid[i].clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for i in 0..grid.len() {
            let col = (0..grid.len()).map(|j| grid[j][i]).collect::<Vec<_>>();

            ret += count.get(&col).unwrap_or(&0);
        }

        ret
    }
}
