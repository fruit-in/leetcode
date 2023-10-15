# 2600. K 件物品的最大和
袋子中装有一些物品，每个物品上都标记着数字 `1` 、`0` 或 `-1` 。

给你四个非负整数 `numOnes` 、`numZeros` 、`numNegOnes` 和 `k` 。

袋子最初包含：

* `numOnes` 件标记为 `1` 的物品。
* `numZeros` 件标记为 `0` 的物品。
* `numNegOnes` 件标记为 `-1` 的物品。

现计划从这些物品中恰好选出 `k` 件物品。返回所有可行方案中，物品上所标记数字之和的最大值。

#### 示例 1:
<pre>
<strong>输入:</strong> numOnes = 3, numZeros = 2, numNegOnes = 0, k = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 袋子中的物品分别标记为 {1, 1, 1, 0, 0} 。取 2 件标记为 1 的物品，得到的数字之和为 2 。
可以证明 2 是所有可行方案中的最大值。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> numOnes = 3, numZeros = 2, numNegOnes = 0, k = 4
<strong>输出:</strong> 3
<strong>解释:</strong> 袋子中的物品分别标记为 {1, 1, 1, 0, 0} 。取 3 件标记为 1 的物品，1 件标记为 0 的物品，得到的数字之和为 3 。
可以证明 3 是所有可行方案中的最大值。
</pre>

#### 提示:
* `0 <= numOnes, numZeros, numNegOnes <= 50`
* `0 <= k <= numOnes + numZeros + numNegOnes`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        if num_ones + num_zeros >= k {
            num_ones.min(k)
        } else {
            num_ones * 2 - k + num_zeros
        }
    }
}
```
