impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut upcount = 0;
        let mut downcount = 0;
        let mut ret = 0;

        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                upcount = 0;
                downcount = 0;
            } else if arr[i] > arr[i - 1] && i > 1 && arr[i - 1] < arr[i - 2] {
                upcount = 1;
                downcount = 0;
            } else if arr[i] > arr[i - 1] {
                upcount += 1;
            } else {
                downcount += 1;
            }

            if upcount > 0 && downcount > 0 {
                ret = ret.max(upcount + downcount + 1);
            }
        }

        ret
    }
}
