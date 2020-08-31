impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut i = 0;

        for n in 1..2001 {
            if i >= arr.len() || n < arr[i] {
                k -= 1;
            } else {
                i += 1;
            }
            if k == 0 {
                return n;
            }
        }

        0
    }
}
