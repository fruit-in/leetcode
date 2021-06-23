impl Solution {
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut ret = 0;

        while n > 0 {
            ret += n % k;
            n /= k;
        }

        ret
    }
}
