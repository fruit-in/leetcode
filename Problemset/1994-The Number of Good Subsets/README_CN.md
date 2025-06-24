# 1994. 好子集的数目
给你一个整数数组 `nums` 。如果 `nums` 的一个子集中，所有元素的乘积可以表示为一个或多个 **互不相同的质数** 的乘积，那么我们称它为 **好子集** 。

* 比方说，如果 `nums = [1, 2, 3, 4]` ：
    * `[2, 3]` ，`[1, 2, 3]` 和 `[1, 3]` 是 **好** 子集，乘积分别为 `6 = 2*3` ，`6 = 2*3` 和 `3 = 3` 。
    * `[1, 4]` 和 `[4]` 不是 **好** 子集，因为乘积分别为 `4 = 2*2` 和 `4 = 2*2` 。

请你返回 `nums` 中不同的 **好** 子集的数目对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

`nums` 中的 **子集** 是通过删除 `nums` 中一些（可能一个都不删除，也可能全部都删除）元素后剩余元素组成的数组。如果两个子集删除的下标不同，那么它们被视为不同的子集。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> 6
<strong>解释:</strong> 好子集为：
- [1,2]：乘积为 2 ，可以表示为质数 2 的乘积。
- [1,2,3]：乘积为 6 ，可以表示为互不相同的质数 2 和 3 的乘积。
- [1,3]：乘积为 3 ，可以表示为质数 3 的乘积。
- [2]：乘积为 2 ，可以表示为质数 2 的乘积。
- [2,3]：乘积为 6 ，可以表示为互不相同的质数 2 和 3 的乘积。
- [3]：乘积为 3 ，可以表示为质数 3 的乘积。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,2,3,15]
<strong>输出:</strong> 5
<strong>解释:</strong> 好子集为：
- [2]：乘积为 2 ，可以表示为质数 2 的乘积。
- [2,3]：乘积为 6 ，可以表示为互不相同质数 2 和 3 的乘积。
- [2,15]：乘积为 30 ，可以表示为互不相同质数 2，3 和 5 的乘积。
- [3]：乘积为 3 ，可以表示为质数 3 的乘积。
- [15]：乘积为 15 ，可以表示为互不相同质数 3 和 5 的乘积。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `1 <= nums[i] <= 30`

## 题解 (Rust)

### 1. 题解
```Rust
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
```
