impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut ret = 0;

        for i in 0..arr.len() {
            x ^= 1 << arr[i];
            if x == (2 << i) - 1 {
                ret += 1;
            }
        }

        ret
    }
}
