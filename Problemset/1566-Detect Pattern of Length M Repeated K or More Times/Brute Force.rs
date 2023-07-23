impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;

        if arr.len() < m * k {
            return false;
        }

        for i in 0..=(arr.len() - m * k) {
            if (i..(i + m * (k - 1))).all(|j| arr[j] == arr[j + m]) {
                return true;
            }
        }

        false
    }
}
