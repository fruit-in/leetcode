# 2145. 统计隐藏数组数目
给你一个下标从 **0** 开始且长度为 `n` 的整数数组 `differences` ，它表示一个长度为 `n + 1` 的 **隐藏** 数组 **相邻** 元素之间的 **差值** 。更正式的表述为：我们将隐藏数组记作 `hidden` ，那么 `differences[i] = hidden[i + 1] - hidden[i]` 。

同时给你两个整数 `lower` 和 `upper` ，它们表示隐藏数组中所有数字的值都在 闭 区间 `[lower, upper]` 之间。

* 比方说，`differences = [1, -3, 4]` ，`lower = 1` ，`upper = 6` ，那么隐藏数组是一个长度为 `4` 且所有值都在 `1` 和 `6` （包含两者）之间的数组。
  * `[3, 4, 1, 5]` 和 `[4, 5, 2, 6]` 都是符合要求的隐藏数组。
  * `[5, 6, 3, 7]` 不符合要求，因为它包含大于 `6` 的元素。
  * `[1, 2, 3, 4]` 不符合要求，因为相邻元素的差值不符合给定数据。

请你返回 **符合** 要求的隐藏数组的数目。如果没有符合要求的隐藏数组，请返回 `0` 。

#### 示例 1:
<pre>
<strong>输入:</strong> differences = [1,-3,4], lower = 1, upper = 6
<strong>输出:</strong> 2
<strong>解释:</strong> 符合要求的隐藏数组为：
- [3, 4, 1, 5]
- [4, 5, 2, 6]
所以返回 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> differences = [3,-4,5,1,-2], lower = -4, upper = 5
<strong>输出:</strong> 4
<strong>解释:</strong> 符合要求的隐藏数组为：
- [-3, 0, -4, 1, 2, 0]
- [-2, 1, -3, 2, 3, 1]
- [-1, 2, -2, 3, 4, 2]
- [0, 3, -1, 4, 5, 3]
所以返回 4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> differences = [4,-7,2], lower = 3, upper = 6
<strong>输出:</strong> 0
<strong>解释:</strong> 没有符合要求的隐藏数组，所以返回 0 。
</pre>

#### 提示:
* `n == differences.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= differences[i] <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= lower <= upper <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prefix_sum = 0;
        let mut max_num = 0;
        let mut min_num = 0;

        for &x in &differences {
            prefix_sum += x as i64;
            max_num = max_num.max(prefix_sum);
            min_num = min_num.min(prefix_sum);
        }

        ((upper - lower) as i64 + min_num - max_num + 1).max(0) as i32
    }
}
```
