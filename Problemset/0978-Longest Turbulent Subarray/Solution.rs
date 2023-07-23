impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut count0 = 1;
        let mut count1 = 1;
        let mut ret = 1;

        for k in 0..arr.len() - 1 {
            if (k % 2 == 1 && arr[k] > arr[k + 1]) || (k % 2 == 0 && arr[k] < arr[k + 1]) {
                count0 += 1;
            } else {
                ret = ret.max(count0);
                count0 = 1;
            }
            if (k % 2 == 0 && arr[k] > arr[k + 1]) || (k % 2 == 1 && arr[k] < arr[k + 1]) {
                count1 += 1;
            } else {
                ret = ret.max(count1);
                count1 = 1;
            }
        }

        ret.max(count0).max(count1)
    }
}
