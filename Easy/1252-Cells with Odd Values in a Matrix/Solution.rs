impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![false; m as usize]; n as usize];

        for index in indices {
            let ri = index[0] as usize;
            let ci = index[1] as usize;

            for c in 0..(m as usize) {
                matrix[ri][c] = !matrix[ri][c];
            }
            for r in 0..(n as usize) {
                matrix[r][ci] = !matrix[r][ci];
            }
        }

        matrix.iter().map(|r| r.iter().filter(|&&c| c).count() as i32).sum()
    }
}
