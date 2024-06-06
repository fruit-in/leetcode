impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut less = vec![(0, 0); arr.len()];
        let mut ret = 0;

        for i in 0..arr.len() {
            while stack.last().unwrap_or(&(0, 0)).1 >= arr[i] {
                stack.pop();
            }

            less[i].0 = i as i64 - stack.last().unwrap_or(&(-1, 0)).0;
            stack.push((i as i64, arr[i]));
        }

        stack.clear();

        for i in (0..arr.len()).rev() {
            while stack.last().unwrap_or(&(0, 0)).1 > arr[i] {
                stack.pop();
            }

            less[i].1 = stack.last().unwrap_or(&(arr.len() as i64, 0)).0 - i as i64;
            ret = (ret + arr[i] as i64 * less[i].0 * less[i].1) % 1_000_000_007;
            stack.push((i as i64, arr[i]));
        }

        ret as i32
    }
}
