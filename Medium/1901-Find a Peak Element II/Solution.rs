impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut i = 0;
        let mut j = 0;

        loop {
            let mut peak_i = i;
            let mut peak_j = j;

            if i > 0 && mat[i - 1][j] > mat[peak_i][peak_j] {
                peak_i = i - 1;
                peak_j = j;
            }
            if i < mat.len() - 1 && mat[i + 1][j] > mat[peak_i][peak_j] {
                peak_i = i + 1;
                peak_j = j;
            }
            if j > 0 && mat[i][j - 1] > mat[peak_i][peak_j] {
                peak_i = i;
                peak_j = j - 1;
            }
            if j < mat[0].len() - 1 && mat[i][j + 1] > mat[peak_i][peak_j] {
                peak_i = i;
                peak_j = j + 1;
            }

            if peak_i == i && peak_j == j {
                break;
            }

            i = peak_i;
            j = peak_j;
        }

        vec![i as i32, j as i32]
    }
}
