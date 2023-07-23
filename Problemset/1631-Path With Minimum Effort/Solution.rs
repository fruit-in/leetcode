impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut min_efforts = vec![vec![i32::MAX; heights[0].len()]; heights.len()];
        let mut cells = vec![(0, 0)];
        min_efforts[0][0] = 0;

        while let Some((i, j)) = cells.pop() {
            if i > 0 {
                let effort = (heights[i][j] - heights[i - 1][j])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i - 1][j] {
                    min_efforts[i - 1][j] = effort;
                    cells.push((i - 1, j));
                }
            }
            if i < heights.len() - 1 {
                let effort = (heights[i][j] - heights[i + 1][j])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i + 1][j] {
                    min_efforts[i + 1][j] = effort;
                    cells.push((i + 1, j));
                }
            }
            if j > 0 {
                let effort = (heights[i][j] - heights[i][j - 1])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i][j - 1] {
                    min_efforts[i][j - 1] = effort;
                    cells.push((i, j - 1));
                }
            }
            if j < heights[0].len() - 1 {
                let effort = (heights[i][j] - heights[i][j + 1])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i][j + 1] {
                    min_efforts[i][j + 1] = effort;
                    cells.push((i, j + 1));
                }
            }
        }

        *min_efforts.last().unwrap().last().unwrap()
    }
}
