impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        fn f(x: i64) -> i32 {
            let mut pow5 = 5;
            let mut ret = 0;

            while pow5 <= x {
                ret += x / pow5;
                pow5 *= 5;
            }

            ret as i32
        }

        let mut lo = 0;
        let mut hi = 800000004;

        while lo < hi {
            let mid = (lo + hi) / 2;
            let zeros = f(mid * 5);

            if zeros <= k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        5 * (f((hi - 1) * 5) == k) as i32
    }
}
