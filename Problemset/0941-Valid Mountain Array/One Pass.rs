impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let length = a.len();
        let mut i = 0;
        while i + 1 < length && a[i] < a[i + 1] {
            i += 1;
        }
        if i == 0 || i + 1 == length {
            return false;
        }
        while i + 1 < length && a[i] > a[i + 1] {
            i += 1;
        }
        i + 1 == length
    }
}
