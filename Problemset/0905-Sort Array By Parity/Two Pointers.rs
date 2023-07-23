impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut i = 0;
        let mut j = a.len() - 1;
        while i < j {
            while i < j && a[i] % 2 == 0 {
                i += 1;
            }
            while i < j && a[j] % 2 == 1 {
                j -= 1;
            }
            a.swap(i, j);
        }
        a
    }
}
