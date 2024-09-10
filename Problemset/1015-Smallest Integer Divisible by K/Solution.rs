impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut rem = 0;

        for n in 1..=k {
            rem = (rem * 10 % k + 1 % k) % k;

            if rem == 0 {
                return n;
            }
        }

        -1
    }
}
