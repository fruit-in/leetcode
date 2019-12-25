impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut flag = 0;

        for i in 1..a.len() {
            if a[i - 1] != a[i] {
                if flag * (a[i - 1] - a[i]) >= 0 {
                    flag = a[i - 1] - a[i];
                } else {
                    return false;
                }
            }
        }

        true
    }
}
