# 2281. Sum of Total Strength of Wizards
As the ruler of a kingdom, you have an army of wizards at your command.

You are given a **0-indexed** integer array `strength`, where `strength[i]` denotes the strength of the <code>i<sup>th</sup></code> wizard. For a **contiguous** group of wizards (i.e. the wizards' strengths form a **subarray** of `strength`), the **total strength** is defined as the **product** of the following two values:
* The strength of the **weakest** wizard in the group.
* The **total** of all the individual strengths of the wizards in the group.

Return *the **sum** of the total strengths of **all** contiguous groups of wizards*. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

A **subarray** is a contiguous **non-empty** sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> strength = [1,3,1,2]
<strong>Output:</strong> 44
<strong>Explanation:</strong> The following are all the contiguous groups of wizards:
- [1] from [1,3,1,2] has a total strength of min([1]) * sum([1]) = 1 * 1 = 1
- [3] from [1,3,1,2] has a total strength of min([3]) * sum([3]) = 3 * 3 = 9
- [1] from [1,3,1,2] has a total strength of min([1]) * sum([1]) = 1 * 1 = 1
- [2] from [1,3,1,2] has a total strength of min([2]) * sum([2]) = 2 * 2 = 4
- [1,3] from [1,3,1,2] has a total strength of min([1,3]) * sum([1,3]) = 1 * 4 = 4
- [3,1] from [1,3,1,2] has a total strength of min([3,1]) * sum([3,1]) = 1 * 4 = 4
- [1,2] from [1,3,1,2] has a total strength of min([1,2]) * sum([1,2]) = 1 * 3 = 3
- [1,3,1] from [1,3,1,2] has a total strength of min([1,3,1]) * sum([1,3,1]) = 1 * 5 = 5
- [3,1,2] from [1,3,1,2] has a total strength of min([3,1,2]) * sum([3,1,2]) = 1 * 6 = 6
- [1,3,1,2] from [1,3,1,2] has a total strength of min([1,3,1,2]) * sum([1,3,1,2]) = 1 * 7 = 7
The sum of all the total strengths is 1 + 9 + 1 + 4 + 4 + 4 + 3 + 5 + 6 + 7 = 44.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> strength = [5,4,6]
<strong>Output:</strong> 213
<strong>Explanation:</strong> The following are all the contiguous groups of wizards:
- [5] from [5,4,6] has a total strength of min([5]) * sum([5]) = 5 * 5 = 25
- [4] from [5,4,6] has a total strength of min([4]) * sum([4]) = 4 * 4 = 16
- [6] from [5,4,6] has a total strength of min([6]) * sum([6]) = 6 * 6 = 36
- [5,4] from [5,4,6] has a total strength of min([5,4]) * sum([5,4]) = 4 * 9 = 36
- [4,6] from [5,4,6] has a total strength of min([4,6]) * sum([4,6]) = 4 * 10 = 40
- [5,4,6] from [5,4,6] has a total strength of min([5,4,6]) * sum([5,4,6]) = 4 * 15 = 60
The sum of all the total strengths is 25 + 16 + 36 + 36 + 40 + 60 = 213.
</pre>

#### Constraints:
* <code>1 <= strength.length <= 10<sup>5</sup></code>
* <code>1 <= strength[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
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
```
