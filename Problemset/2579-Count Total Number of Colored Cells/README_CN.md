# 2579. 统计染色格子数
有一个无穷大的二维网格图，一开始所有格子都未染色。给你一个正整数 `n` ，表示你需要执行以下步骤 `n` 分钟：

* 第一分钟，将 **任一** 格子染成蓝色。
* 之后的每一分钟，将与蓝色格子相邻的 **所有** 未染色格子染成蓝色。

下图分别是 1、2、3 分钟后的网格图。

![](https://assets.leetcode.com/uploads/2023/01/10/example-copy-2.png)

请你返回 `n` 分钟之后 **被染色的格子** 数目。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 1 分钟后，只有 1 个蓝色的格子，所以返回 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 5
<strong>解释:</strong> 2 分钟后，有 4 个在边缘的蓝色格子和 1 个在中间的蓝色格子，所以返回 5 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        (1..n as i64).fold(1, |acc, x| acc + x * 4)
    }
}
```
