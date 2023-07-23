impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut k = (k as i64 % chalk.iter().map(|&x| x as i64).sum::<i64>()) as i32;

        for i in 0..chalk.len() {
            if k < chalk[i] {
                return i as i32;
            }

            k -= chalk[i];
        }

        unreachable!()
    }
}
