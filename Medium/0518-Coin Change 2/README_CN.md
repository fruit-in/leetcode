# 518. 零钱兑换 II
给定不同面额的硬币和一个总金额。写出函数来计算可以凑成总金额的硬币组合数。假设每一种面额的硬币有无限个。

#### 示例 1:
<pre>
<strong>输入:</strong> amount = 5, coins = [1, 2, 5]
<strong>输出:</strong> 4
<strong>解释:</strong> 有四种方式可以凑成总金额:
5=5
5=2+2+1
5=2+1+1+1
5=1+1+1+1+1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> amount = 3, coins = [2]
<strong>输出:</strong> 0
<strong>解释:</strong> 只用面额2的硬币不能凑成总金额3。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> amount = 10, coins = [10]
<strong>输出:</strong> 1
</pre>

#### 注意:
你可以假设：
* 0 <= amount (总金额) <= 5000
* 1 <= coin (硬币面额) <= 5000
* 硬币种类不超过 500 种
* 结果符合 32 位符号整数

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for &coin in &coins {
            for i in (coin as usize)..=(amount as usize) {
                dp[i] += dp[i - coin as usize];
            }
        }

        dp[amount as usize]
    }
}
```
