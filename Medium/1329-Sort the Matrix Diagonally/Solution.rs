impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ret = vec![vec![0; n]; m];

        for i in 0..(m + n) {
            let mut row = (m - 1).saturating_sub(i);
            let mut col = i.saturating_sub(m - 1);
            let mut arr = vec![];

            for j in 0..(m - row).min(n - col) {
                arr.push(mat[row + j][col + j]);
            }

            arr.sort_unstable();

            for j in 0..(m - row).min(n - col) {
                ret[row + j][col + j] = arr[j];
            }
        }

        ret
    }
}
