impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![];

        for i in 0..n {
            let max_num = (0..m).map(|j| grid[j][i]).max().unwrap();
            let min_num = (0..m).map(|j| grid[j][i]).min().unwrap();

            ans.push(max_num.to_string().len().max(min_num.to_string().len()) as i32);
        }

        ans
    }
}
