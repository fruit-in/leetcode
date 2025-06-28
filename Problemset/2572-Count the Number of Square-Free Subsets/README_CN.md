# 2572. 无平方子集计数
给你一个正整数数组 `nums` 。

如果数组 `nums` 的子集中的元素乘积是一个 **无平方因子数** ，则认为该子集是一个 **无平方** 子集。

**无平方因子数** 是无法被除 `1` 之外任何平方数整除的数字。

返回数组 `nums` 中 **无平方** 且 **非空** 的子集数目。因为答案可能很大，返回对 <code>10<sup>9</sup> + 7</code> 取余的结果。

`nums` 的 **非空子集** 是可以由删除 `nums` 中一些元素（可以不删除，但不能全部删除）得到的一个数组。如果构成两个子集时选择删除的下标不同，则认为这两个子集不同。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,4,4,5]
<strong>输出:</strong> 3
<strong>解释:</strong> 示例中有 3 个无平方子集：
- 由第 0 个元素 [3] 组成的子集。其元素的乘积是 3 ，这是一个无平方因子数。
- 由第 3 个元素 [5] 组成的子集。其元素的乘积是 5 ，这是一个无平方因子数。
- 由第 0 个和第 3 个元素 [3,5] 组成的子集。其元素的乘积是 15 ，这是一个无平方因子数。
可以证明给定数组中不存在超过 3 个无平方子集。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1]
<strong>输出:</strong> 1
<strong>解释:</strong> 示例中有 1 个无平方子集：
- 由第 0 个元素 [1] 组成的子集。其元素的乘积是 1 ，这是一个无平方因子数。
可以证明给定数组中不存在超过 1 个无平方子集。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= 30`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
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
        ret = (ret + (0..count[1]).fold(1, |acc, _| acc * 2 % 1_000_000_007) - 1) % 1_000_000_007;

        ret as i32
    }
}
```
