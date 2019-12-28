impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        for i in 1..4 {
            for j in i..a.len() {
                if a[j - i] == a[j] {
                    return a[j];
                }
            }
        }

        -1
    }
}
