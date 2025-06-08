impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let (mut x, mut y) = (arr[0], 0);
        let mut ret = x;

        for i in 1..arr.len() {
            (x, y) = (arr[i].max(x + arr[i]), x.max(y + arr[i]));
            ret = ret.max(x).max(y);
        }

        ret
    }
}
