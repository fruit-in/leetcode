impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut greatest = -1;

        for i in (0..arr.len()).rev() {
            let tmp = greatest;
            greatest = greatest.max(arr[i]);
            arr[i] = tmp;
        }

        arr
    }
}
