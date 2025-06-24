impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut count = vec![0; m + 1];
        let mut primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        primes.retain(|&x| x <= m);
        let n = primes.len();
        let mut dp = vec![0; 1 << n];
        dp[0] = 1_i64;
        let mut ret = 0;

        for &x in &nums {
            if x % 4 != 0 && x % 9 != 0 && x % 25 != 0 {
                count[x as usize] += 1;
            }
        }

        for x in 2..=m {
            if count[x] == 0 {
                continue;
            }

            let mut tmp = dp.clone();
            let mut mask = 0;

            for i in 0..n {
                if x % primes[i] == 0 {
                    mask |= 1 << i;
                }
            }
            for i in 0..1 << n {
                if mask & i == 0 {
                    dp[mask | i] = (dp[mask | i] + tmp[i] * count[x]) % 1_000_000_007;
                }
            }
        }

        for i in 1..1 << n {
            ret = (ret + dp[i]) % 1_000_000_007;
        }
        for _ in 0..count[1] {
            ret = (ret * 2) % 1_000_000_007;
        }

        ret as i32
    }
}
