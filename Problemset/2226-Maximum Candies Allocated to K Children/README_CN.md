# 2226. 每个小孩最多能分到多少糖果
给你一个 **下标从 0 开始** 的整数数组 `candies` 。数组中的每个元素表示大小为 `candies[i]` 的一堆糖果。你可以将每堆糖果分成任意数量的 **子堆** ，但 **无法** 再将两堆合并到一起。

另给你一个整数 `k` 。你需要将这些糖果分配给 `k` 个小孩，使每个小孩分到 **相同** 数量的糖果。每个小孩可以拿走 **至多一堆** 糖果，有些糖果可能会不被分配。

返回每个小孩可以拿走的 **最大糖果数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> candies = [5,8,6], k = 3
<strong>输出:</strong> 5
<strong>解释:</strong> 可以将 candies[1] 分成大小分别为 5 和 3 的两堆，然后把 candies[2] 分成大小分别为 5 和 1 的两堆。现在就有五堆大小分别为 5、5、3、5 和 1 的糖果。可以把 3 堆大小为 5 的糖果分给 3 个小孩。可以证明无法让每个小孩得到超过 5 颗糖果。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> candies = [2,5], k = 11
<strong>输出:</strong> 0
<strong>解释:</strong> 总共有 11 个小孩，但只有 7 颗糖果，但如果要分配糖果的话，必须保证每个小孩至少能得到 1 颗糖果。因此，最后每个小孩都没有得到糖果，答案是 0 。
</pre>

#### 提示:
* <code>1 <= candies.length <= 10<sup>5</sup></code>
* <code>1 <= candies[i] <= 10<sup>7</sup></code>
* <code>1 <= k <= 10<sup>12</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut l = 1;
        let mut r = 10_000_001;

        while l < r {
            let m = (l + r) / 2;
            let count = candies.iter().map(|&x| (x / m) as i64).sum::<i64>();

            if count < k {
                r = m;
            } else {
                l = m + 1;
            }
        }

        r - 1
    }
}
```
