impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();

        let mut times = [false; 1001];
        let mut i = 0;
        let mut j = 1;
        while i < arr.len() {
            while j < arr.len() && arr[i] == arr[j] {
                j += 1;
            }
            if times[j - i] {
                return false;
            }
            times[j - i] = true;
            i = j;
        }

        true
    }
}
