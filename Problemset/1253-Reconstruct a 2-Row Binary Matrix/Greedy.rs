impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; colsum.len()]; 2];

        for i in 0..colsum.len() {
            if colsum[i] == 2 {
                ret[0][i] = 1;
                ret[1][i] = 1;
                upper -= 1;
                lower -= 1;
            } else if colsum[i] == 1 {
                if upper > lower {
                    ret[0][i] = 1;
                    upper -= 1;
                } else {
                    ret[1][i] = 1;
                    lower -= 1;
                }
            }
        }

        if upper | lower != 0 {
            vec![]
        } else {
            ret
        }
    }
}
