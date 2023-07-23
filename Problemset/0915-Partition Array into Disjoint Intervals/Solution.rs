impl Solution {
    pub fn partition_disjoint(a: Vec<i32>) -> i32 {
        let mut max_left = a[0];
        let mut max = a[0];
        let mut length = 1;

        for i in 1..a.len() {
            if a[i] < max_left {
                length = i + 1;
                max_left = max;
            } else if a[i] > max {
                max = a[i];
            }
        }

        length as i32
    }
}
