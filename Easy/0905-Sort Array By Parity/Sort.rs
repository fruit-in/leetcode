impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        a.sort_unstable_by_key(|k| k % 2 == 1);
        a
    }
}
