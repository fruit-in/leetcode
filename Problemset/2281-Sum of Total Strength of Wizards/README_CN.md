# 2281. 巫师的总力量和
作为国王的统治者，你有一支巫师军队听你指挥。

给你一个下标从 **0** 开始的整数数组 `strength` ，其中 `strength[i]` 表示第 `i` 位巫师的力量值。对于连续的一组巫师（也就是这些巫师的力量值是 `strength` 的 **子数组**），**总力量** 定义为以下两个值的 **乘积** ：
* 巫师中 **最弱** 的能力值。
* 组中所有巫师的个人力量值 **之和** 。

请你返回 **所有** 巫师组的 **总** 力量之和。由于答案可能很大，请将答案对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

**子数组** 是一个数组里 **非空** 连续子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> strength = [1,3,1,2]
<strong>输出:</strong> 44
<strong>解释:</strong> 以下是所有连续巫师组：
- [1,3,1,2] 中 [1] ，总力量值为 min([1]) * sum([1]) = 1 * 1 = 1
- [1,3,1,2] 中 [3] ，总力量值为 min([3]) * sum([3]) = 3 * 3 = 9
- [1,3,1,2] 中 [1] ，总力量值为 min([1]) * sum([1]) = 1 * 1 = 1
- [1,3,1,2] 中 [2] ，总力量值为 min([2]) * sum([2]) = 2 * 2 = 4
- [1,3,1,2] 中 [1,3] ，总力量值为 min([1,3]) * sum([1,3]) = 1 * 4 = 4
- [1,3,1,2] 中 [3,1] ，总力量值为 min([3,1]) * sum([3,1]) = 1 * 4 = 4
- [1,3,1,2] 中 [1,2] ，总力量值为 min([1,2]) * sum([1,2]) = 1 * 3 = 3
- [1,3,1,2] 中 [1,3,1] ，总力量值为 min([1,3,1]) * sum([1,3,1]) = 1 * 5 = 5
- [1,3,1,2] 中 [3,1,2] ，总力量值为 min([3,1,2]) * sum([3,1,2]) = 1 * 6 = 6
- [1,3,1,2] 中 [1,3,1,2] ，总力量值为 min([1,3,1,2]) * sum([1,3,1,2]) = 1 * 7 = 7
所有力量值之和为 1 + 9 + 1 + 4 + 4 + 4 + 3 + 5 + 6 + 7 = 44 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> strength = [5,4,6]
<strong>输出:</strong> 213
<strong>解释:</strong> 以下是所有连续巫师组：
- [5,4,6] 中 [5] ，总力量值为 min([5]) * sum([5]) = 5 * 5 = 25
- [5,4,6] 中 [4] ，总力量值为 min([4]) * sum([4]) = 4 * 4 = 16
- [5,4,6] 中 [6] ，总力量值为 min([6]) * sum([6]) = 6 * 6 = 36
- [5,4,6] 中 [5,4] ，总力量值为 min([5,4]) * sum([5,4]) = 4 * 9 = 36
- [5,4,6] 中 [4,6] ，总力量值为 min([4,6]) * sum([4,6]) = 4 * 10 = 40
- [5,4,6] 中 [5,4,6] ，总力量值为 min([5,4,6]) * sum([5,4,6]) = 4 * 15 = 60
所有力量值之和为 25 + 16 + 36 + 36 + 40 + 60 = 213 。
</pre>

#### 提示:
* <code>1 <= strength.length <= 10<sup>5</sup></code>
* <code>1 <= strength[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
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
