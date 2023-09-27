impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut mat = mat;
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    let mut min_count = i32::MAX;

                    if j > 0 {
                        mat[i][j] += mat[i][j - 1];
                    }

                    for k in (0..=i).rev() {
                        if mat[k][j] == 0 {
                            break;
                        }

                        min_count = min_count.min(mat[k][j]);
                        ret += min_count;
                    }
                }
            }
        }

        ret
    }
}
