impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                for j in ((i + 1)..arr.len()).rev() {
                    arr[j] = arr[j - 1];
                }
                i += 1;
            }
            i += 1;
        }
    }
}
