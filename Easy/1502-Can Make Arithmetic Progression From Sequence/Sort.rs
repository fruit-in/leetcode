impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();

        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] != arr[i - 1] - arr[i - 2] {
                return false;
            }
        }

        true
    }
}
