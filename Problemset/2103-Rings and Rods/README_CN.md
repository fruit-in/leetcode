# 2103. 环和杆
总计有 `n` 个环，环的颜色可以是红、绿、蓝中的一种。这些环分布穿在 10 根编号为 `0` 到 `9` 的杆上。

给你一个长度为 `2n` 的字符串 `rings` ，表示这 `n` 个环在杆上的分布。`rings` 中每两个字符形成一个 **颜色位置对** ，用于描述每个环：
* 第 `i` 对中的 **第一个** 字符表示第 `i` 个环的 **颜色**（`'R'`、`'G'`、`'B'`）。
* 第 `i` 对中的 **第二个** 字符表示第 `i` 个环的 **位置**，也就是位于哪根杆上（`'0'` 到 `'9'`）。

例如，`"R3G2B1"` 表示：共有 `n == 3` 个环，红色的环在编号为 3 的杆上，绿色的环在编号为 2 的杆上，蓝色的环在编号为 1 的杆上。

找出所有集齐 **全部三种颜色** 环的杆，并返回这种杆的数量。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/11/23/ex1final.png)
<pre>
<strong>输入:</strong> rings = "B0B6G0R6R0R6G9"
<strong>输出:</strong> 1
<strong>解释:</strong>
- 编号 0 的杆上有 3 个环，集齐全部颜色：红、绿、蓝。
- 编号 6 的杆上有 3 个环，但只有红、蓝两种颜色。
- 编号 9 的杆上只有 1 个绿色环。
因此，集齐全部三种颜色环的杆的数目为 1 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/11/23/ex2final.png)
<pre>
<strong>输入:</strong> rings = "B0R0G0R9R0B0G0"
<strong>输出:</strong> 1
<strong>解释:</strong>
- 编号 0 的杆上有 6 个环，集齐全部颜色：红、绿、蓝。
- 编号 9 的杆上只有 1 个红色环。
因此，集齐全部三种颜色环的杆的数目为 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> rings = "G4"
<strong>输出:</strong> 0
<strong>解释:</strong>
只给了一个环，因此，不存在集齐全部三种颜色环的杆。
</pre>

#### 提示:
* `rings.length == 2 * n`
* `1 <= n <= 100`
* 如 `i` 是 **偶数** ，则 `rings[i]` 的值可以取 `'R'`、`'G'` 或 `'B'`（下标从 **0** 开始计数）
* 如 `i` 是 **奇数** ，则 `rings[i]` 的值可以取 `'0'` 到 `'9'` 中的一个数字（下标从 **0** 开始计数）

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let rings = rings.as_bytes();
        let mut rods = [0; 10];

        for i in (0..rings.len()).step_by(2) {
            match (rings[i], (rings[i + 1] - b'0') as usize) {
                (b'R', r) => rods[r] |= 1,
                (b'G', r) => rods[r] |= 2,
                (_, r) => rods[r] |= 4,
            }
        }

        rods.iter().filter(|&&r| r == 7).count() as i32
    }
}
```
