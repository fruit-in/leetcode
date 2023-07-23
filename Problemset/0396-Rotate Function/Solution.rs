impl Solution {
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        let mut f = 0;
        let mut sum = 0;

        for i in 0..a.len() {
            f += i as i32 * a[i];
            sum += a[i];
        }

        let mut ret = f;

        for i in 0..a.len() {
            f += a.len() as i32 * a[i] - sum;
            ret = ret.max(f);
        }

        ret
    }
}
