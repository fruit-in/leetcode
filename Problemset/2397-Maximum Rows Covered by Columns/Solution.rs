impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut comb = (1_usize..2 << n)
            .filter(|x| x.count_ones() as i32 == num_select)
            .collect::<Vec<_>>();
        let mut ret = 0;

        for &s in &comb {
            let mut covered = 0;

            for row in 0..m {
                if (0..n)
                    .filter(|&col| matrix[row][col] == 1)
                    .all(|col| s & (1 << col) != 0)
                {
                    covered += 1;
                }
            }

            ret = ret.max(covered);
        }

        ret
    }
}
