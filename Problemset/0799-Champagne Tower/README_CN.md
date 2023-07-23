# 799. 香槟塔
我们把玻璃杯摆成金字塔的形状，其中第一层有1个玻璃杯，第二层有2个，依次类推到第100层，每个玻璃杯(250ml)将盛有香槟。

从顶层的第一个玻璃杯开始倾倒一些香槟，当顶层的杯子满了，任何溢出的香槟都会立刻等流量的流向左右两侧的玻璃杯。当左右两边的杯子也满了，就会等流量的流向它们左右两边的杯子，依次类推。（当最底层的玻璃杯满了，香槟会流到地板上）

例如，在倾倒一杯香槟后，最顶层的玻璃杯满了。倾倒了两杯香槟后，第二层的两个玻璃杯各自盛放一半的香槟。在倒三杯香槟后，第二层的香槟满了 - 此时总共有三个满的玻璃杯。在倒第四杯后，第三层中间的玻璃杯盛放了一半的香槟，他两边的玻璃杯各自盛放了四分之一的香槟，如下图所示。

![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/03/09/tower.png)

现在当倾倒了非负整数杯香槟后，返回第 i 行 j 个玻璃杯所盛放的香槟占玻璃杯容积的比例（i 和 j都从0开始）。

#### 示例 1:
<pre>
<strong>输入:</strong> poured(倾倒香槟总杯数) = 1, query_glass(杯子的位置数) = 1, query_row(行数) = 1
<strong>输出:</strong> 0.0
<strong>解释:</strong> 我们在顶层（下标是（0，0））倒了一杯香槟后，没有溢出，因此所有在顶层以下的玻璃杯都是空的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> poured(倾倒香槟总杯数) = 2, query_glass(杯子的位置数) = 1, query_row(行数) = 1
<strong>输出:</strong> 0.5
<strong>解释:</strong> 我们在顶层（下标是（0，0）倒了两杯香槟后，有一杯量的香槟将从顶层溢出，位于（1，0）的玻璃杯和（1，1）的玻璃杯平分了这一杯香槟，所以每个玻璃杯有一半的香槟。
</pre>

#### 注意:
* `poured` 的范围`[0, 10 ^ 9]`。
* `query_glass` 和`query_row` 的范围 `[0, 99]`。

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = vec![poured as f64];

        for r in 0..query_row as usize {
            dp.push((dp[r] - 1.0).max(0.0) / 2.0);
            for i in (1..=r).rev() {
                dp[i] = (dp[i] - 1.0).max(0.0) / 2.0 + (dp[i - 1] - 1.0).max(0.0) / 2.0;
            }
            dp[0] = (dp[0] - 1.0).max(0.0) / 2.0;
        }

        dp[query_glass as usize].min(1.0)
    }
}
```
