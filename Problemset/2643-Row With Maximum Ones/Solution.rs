impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![0, 0];

        for i in 0..mat.len() {
            let ones = mat[i].iter().sum::<i32>();

            if ones > ret[1] {
                ret = vec![i as i32, ones];
            }
        }

        ret
    }
}
