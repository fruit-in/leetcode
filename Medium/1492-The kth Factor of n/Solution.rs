impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut factor = 1;
        let mut i = 0;

        while factor * factor <= n {
            if n % factor == 0 {
                i += 1;
                if i == k {
                    return factor;
                }
            }
            factor += 1;
        }

        factor -= 1;
        if factor * factor == n {
            factor -= 1;
        }

        while factor > 0 {
            if n % factor == 0 {
                i += 1;
                if i == k {
                    return n / factor;
                }
            }
            factor -= 1;
        }

        -1
    }
}
