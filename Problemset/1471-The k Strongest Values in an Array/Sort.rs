impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort_unstable();
        let m = (arr.len() - 1) / 2;
        let mut i = 0;
        let mut j = arr.len() - 1;
        let mut ret = vec![];

        while ret.len() < k as usize {
            if (arr[i] - arr[m]).abs() > (arr[j] - arr[m]).abs() {
                ret.push(arr[i]);
                i += 1;
            } else {
                ret.push(arr[j]);
                j -= 1;
            }
        }

        ret
    }
}
