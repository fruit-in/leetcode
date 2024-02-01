use std::collections::BTreeSet;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut tree_set = BTreeSet::new();
        let mut ret = vec![];

        for i in 0..m {
            for j in 0..n {
                tree_set.insert(grid[i][j]);

                for k in 1..=i.min(j).min(m - 1 - i).min(n - 1 - j) {
                    let mut rhombus_sum = 0;

                    for a in 0..k {
                        rhombus_sum += grid[i - a][j - k + a];
                        rhombus_sum += grid[i - k + a][j + a];
                        rhombus_sum += grid[i + a][j + k - a];
                        rhombus_sum += grid[i + k - a][j - a];
                    }

                    tree_set.insert(rhombus_sum);
                }
            }
        }

        while ret.len() < 3 && !tree_set.is_empty() {
            ret.push(tree_set.pop_last().unwrap());
        }

        ret
    }
}
