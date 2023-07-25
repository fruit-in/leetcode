impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = pref;

        for i in (1..arr.len()).rev() {
            arr[i] ^= arr[i - 1];
        }

        arr
    }
}
