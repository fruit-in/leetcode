impl Solution {
    pub fn total_strength(strength: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = strength.len();
        let mut stack = vec![];
        let mut left_le = vec![0; n];
        let mut right_lt = vec![0; n];
        let mut prefix_sum = vec![0; n + 1];
        let mut preprefix_sum = vec![0; n + 2];
        let mut suffix_sum = vec![0; n + 1];
        let mut sufsuffix_sum = vec![0; n + 2];
        let mut ret = 0;

        for i in 0..n {
            while stack.last().unwrap_or(&(0, i32::MIN)).1 > strength[i] {
                stack.pop();
            }

            left_le[i] = i as i64 - stack.last().unwrap_or(&(-1, 0)).0;
            stack.push((i as i64, strength[i]));
            prefix_sum[i + 1] = (prefix_sum[i] + strength[i] as i64) % MOD;
            preprefix_sum[i + 1] = (preprefix_sum[i] + prefix_sum[i]) % MOD;
        }
        preprefix_sum[n + 1] = (preprefix_sum[n] + prefix_sum[n]) % MOD;

        stack = vec![];
        for i in (0..n).rev() {
            while stack.last().unwrap_or(&(0, i32::MIN)).1 >= strength[i] {
                stack.pop();
            }

            right_lt[i] = stack.last().unwrap_or(&(n as i64, 0)).0 - i as i64;
            stack.push((i as i64, strength[i]));
            suffix_sum[i] = (suffix_sum[i + 1] + strength[i] as i64) % MOD;
            sufsuffix_sum[i + 1] = (sufsuffix_sum[i + 2] + suffix_sum[i + 1]) % MOD;
        }
        sufsuffix_sum[0] = (sufsuffix_sum[1] + suffix_sum[0]) % MOD;

        for i in 0..n {
            let j = i.wrapping_sub(left_le[i] as usize);
            let k = i + right_lt[i] as usize;
            let mut tmp = strength[i] as i64 * left_le[i] % MOD * right_lt[i] % MOD;
            tmp = (tmp + right_lt[i] * prefix_sum[i] % MOD * (left_le[i] - 1) % MOD) % MOD;
            tmp = (tmp
                + right_lt[i]
                    * (preprefix_sum[j.wrapping_add(1)] - preprefix_sum[i]).rem_euclid(MOD)
                    % MOD)
                % MOD;
            tmp = (tmp + left_le[i] * suffix_sum[i + 1] % MOD * (right_lt[i] - 1) % MOD) % MOD;
            tmp = (tmp
                + left_le[i] * (sufsuffix_sum[k + 1] - sufsuffix_sum[i + 2]).rem_euclid(MOD) % MOD)
                % MOD;
            tmp = tmp * strength[i] as i64 % MOD;
            ret = (ret + tmp) % MOD;
        }

        ret as i32
    }
}
