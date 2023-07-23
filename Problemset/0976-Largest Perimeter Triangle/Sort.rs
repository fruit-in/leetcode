impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let mut a = a;
        a.sort_unstable_by(|a, b| b.cmp(a));

        for i in 2..a.len() {
            if a[i] + a[i - 1] > a[i - 2] {
                return a[i] + a[i - 1] + a[i - 2];
            }
        }

        0
    }
}
