# 2379. 得到 K 个黑块的最少涂色次数
给你一个长度为 `n` 下标从 **0** 开始的字符串 `blocks` ，`blocks[i]` 要么是 `'W'` 要么是 `'B'` ，表示第 `i` 块的颜色。字符 `'W'` 和 `'B'` 分别表示白色和黑色。

给你一个整数 `k` ，表示想要 **连续** 黑色块的数目。

每一次操作中，你可以选择一个白色块将它 **涂成** 黑色块。

请你返回至少出现 一次 连续 `k` 个黑色块的 **最少** 操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> blocks = "WBBWWBBWBW", k = 7
<strong>输出:</strong> 3
<strong>解释:</strong>
一种得到 7 个连续黑色块的方法是把第 0 ，3 和 4 个块涂成黑色。
得到 blocks = "BBBBBBBWBW" 。
可以证明无法用少于 3 次操作得到 7 个连续的黑块。
所以我们返回 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> blocks = "WBWBBBW", k = 2
<strong>输出:</strong> 0
<strong>解释:</strong>
不需要任何操作，因为已经有 2 个连续的黑块。
所以我们返回 0 。
</pre>

#### 提示:
* `n == blocks.length`
* `1 <= n <= 100`
* `blocks[i]` 要么是 `'W'` ，要么是 `'B'` 。
* `1 <= k <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let mut count_w = blocks.chars().take(k).filter(|&c| c == 'W').count();
        let blocks = blocks.as_bytes();
        let mut ret = count_w;

        for i in k..blocks.len() {
            count_w += (blocks[i] == b'W') as usize - (blocks[i - k] == b'W') as usize;
            ret = ret.min(count_w);
        }

        ret as i32
    }
}
```
