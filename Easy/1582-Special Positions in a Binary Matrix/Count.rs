impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let rows = mat.len();
        let cols = mat[0].len();
        let mut row1 = vec![0; rows];
        let mut col1 = vec![0; cols];
        let mut ret = 0;

        for r in 0..rows {
            for c in 0..cols {
                if mat[r][c] == 1 {
                    row1[r] += 1;
                    col1[c] += 1;
                }
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if mat[r][c] == 1 && row1[r] + col1[c] == 2 {
                    ret += 1;
                }
            }
        }

        ret
    }
}
