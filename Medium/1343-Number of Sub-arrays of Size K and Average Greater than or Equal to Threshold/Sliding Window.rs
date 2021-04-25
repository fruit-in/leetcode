impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut sum = arr[..k as usize].iter().sum::<i32>();
        let mut i = 0;
        let mut ret = (sum >= k * threshold) as i32;

        for j in k as usize..arr.len() {
            sum += arr[j] - arr[i];
            i += 1;
            if sum >= k * threshold {
                ret += 1;
            }
        }

        ret
    }
}
