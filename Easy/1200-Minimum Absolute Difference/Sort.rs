impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();
        let mut min = std::i32::MAX;
        let mut ret = Vec::new();

        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] < min {
                min = arr[i] - arr[i - 1];
                ret.clear();
                ret.push(vec![arr[i - 1], arr[i]]);
            } else if arr[i] - arr[i - 1] == min {
                ret.push(vec![arr[i - 1], arr[i]]);
            }
        }

        ret
    }
}
