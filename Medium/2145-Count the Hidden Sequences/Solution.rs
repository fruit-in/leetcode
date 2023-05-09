impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prefix_sum = 0;
        let mut max_num = 0;
        let mut min_num = 0;

        for &x in &differences {
            prefix_sum += x as i64;
            max_num = max_num.max(prefix_sum);
            min_num = min_num.min(prefix_sum);
        }

        ((upper - lower) as i64 + min_num - max_num + 1).max(0) as i32
    }
}
