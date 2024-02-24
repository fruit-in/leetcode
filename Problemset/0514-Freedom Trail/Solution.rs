impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let ring = ring.as_bytes();
        let key = key.as_bytes();
        let mut dp = vec![None; n];
        dp[0] = Some(0);

        for i in 0..key.len() {
            let mut tmp: Vec<Option<i32>> = vec![None; n];

            for j in 0..n {
                if ring[j] != key[i] {
                    continue;
                }

                for k in 0..n {
                    if let Some(x) = dp[k] {
                        let y = tmp[j].unwrap_or(i32::MAX);
                        let a = j.min(k) as i32;
                        let b = j.max(k) as i32;
                        tmp[j] = Some(y.min(x + b - a + 1).min(x + n as i32 - b + a + 1));
                    }
                }
            }

            dp = tmp;
        }

        dp.iter().map(|&x| x.unwrap_or(i32::MAX)).min().unwrap()
    }
}
