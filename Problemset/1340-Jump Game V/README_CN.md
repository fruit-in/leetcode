# 1340. 跳跃游戏 V
给你一个整数数组 `arr` 和一个整数 `d` 。每一步你可以从下标 `i` 跳到：

* `i + x` ，其中 `i + x < arr.length` 且 `0 < x <= d` 。
* `i - x` ，其中 `i - x >= 0` 且 `0 < x <= d` 。

除此以外，你从下标 `i` 跳到下标 `j` 需要满足：`arr[i] > arr[j]` 且 `arr[i] > arr[k]` ，其中下标 `k` 是所有 `i` 到 `j` 之间的数字（更正式的，`min(i, j) < k < max(i, j)`）。

你可以选择数组的任意下标开始跳跃。请你返回你 **最多** 可以访问多少个下标。

请注意，任何时刻你都不能跳到数组的外面。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/01/23/meta-chart.jpeg)
<pre>
<strong>输入:</strong> arr = [6,4,14,6,8,13,9,7,10,6,12], d = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 你可以从下标 10 出发，然后如上图依次经过 10 --> 8 --> 6 --> 7 。
注意，如果你从下标 6 开始，你只能跳到下标 7 处。你不能跳到下标 5 处因为 13 > 9 。你也不能跳到下标 4 处，因为下标 5 在下标 4 和 6 之间且 13 > 9 。
类似的，你不能从下标 3 处跳到下标 2 或者下标 1 处。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [3,3,3,3,3], d = 3
<strong>输出:</strong> 1
<strong>解释:</strong> 你可以从任意下标处开始且你永远无法跳到任何其他坐标。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [7,6,5,4,3,2,1], d = 1
<strong>输出:</strong> 7
<strong>解释:</strong> 从下标 0 处开始，你可以按照数值从大到小，访问所有的下标。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [7,1,7,1,7,1], d = 2
<strong>输出:</strong> 2
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [66], d = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= arr.length <= 1000`
* <code>1 <= arr[i] <= 10<sup>5</sup></code>
* `1 <= d <= arr.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let mut indices = (0..arr.len()).collect::<Vec<_>>();
        let mut max_visit = vec![1; arr.len()];

        indices.sort_unstable_by_key(|&i| arr[i]);

        for i in indices {
            let mut max = arr[i];

            for j in (0.max(i.saturating_sub(d))..i).rev() {
                if arr[j] > max {
                    max_visit[j] = max_visit[j].max(max_visit[i] + 1);
                    max = arr[j];
                }
            }

            max = arr[i];

            for j in i + 1..arr.len().min(i + d + 1) {
                if arr[j] > max {
                    max_visit[j] = max_visit[j].max(max_visit[i] + 1);
                    max = arr[j];
                }
            }
        }

        max_visit.into_iter().max().unwrap()
    }
}
```
