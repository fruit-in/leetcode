impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut i = 1;
        while i + 1 < a.len() && a[i] <= a[i + 1] {
            i += 1;
        }
        i as i32
    }
}
