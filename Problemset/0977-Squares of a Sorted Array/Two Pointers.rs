impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        let mut p = a.binary_search(&0).unwrap_or_else(|x| x);
        let mut n = p;
        while n > 0 || p < a.len() {
            if n == 0 || (p < a.len() && a[p] < -a[n - 1]) {
                ret.push(a[p] * a[p]);
                p += 1;
            } else {
                ret.push(a[n - 1] * a[n - 1]);
                n -= 1;
            }
        }

        ret
    }
}
