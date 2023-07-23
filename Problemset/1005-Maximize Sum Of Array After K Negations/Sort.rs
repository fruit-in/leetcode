impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut a = a;
        a.sort_unstable();

        for i in 0..a.len() {
            if a[i] < 0 && k > 0 {
                a[i] = -a[i];
                k -= 1;
            } else if k % 2 == 0 {
                break;
            } else if i > 0 && a[i] > a[i - 1] {
                a[i - 1] = -a[i - 1];
                break;
            } else {
                a[i] = -a[i];
                break;
            }
        }

        a.iter().sum()
    }
}
