# 2406. 将区间分为最少组数
给你一个二维整数数组 `intervals` ，其中 <code>intervals[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> 表示 **闭** 区间 <code>[left<sub>i</sub>, right<sub>i</sub>]</code> 。

你需要将 `intervals` 划分为一个或者多个区间 **组** ，每个区间 **只** 属于一个组，且同一个组中任意两个区间 **不相交** 。

请你返回 **最少** 需要划分成多少个组。

如果两个区间覆盖的范围有重叠（即至少有一个公共数字），那么我们称这两个区间是 **相交** 的。比方说区间 `[1, 5]` 和 `[5, 8]` 相交。

#### 示例 1:
<pre>
<strong>输入:</strong> intervals = [[5,10],[6,8],[1,5],[2,3],[1,10]]
<strong>输出:</strong> 3
<strong>解释:</strong> 我们可以将区间划分为如下的区间组：
- 第 1 组：[1, 5] ，[6, 8] 。
- 第 2 组：[2, 3] ，[5, 10] 。
- 第 3 组：[1, 10] 。
可以证明无法将区间划分为少于 3 个组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> intervals = [[1,3],[5,6],[8,10],[11,13]]
<strong>输出:</strong> 1
<strong>解释:</strong> 所有区间互不相交，所以我们可以把它们全部放在一个组内。
</pre>

#### 提示:
* <code>1 <= intervals.length <= 10<sup>5</sup></code>
* `intervals[i].length == 2`
* <code>1 <= left<sub>i</sub> <= right<sub>i</sub> <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut groups = BinaryHeap::new();

        intervals.sort_unstable();

        for interval in &intervals {
            let (left, right) = (interval[0], interval[1]);

            if *groups.peek().unwrap_or(&Reverse(i32::MAX)) > Reverse(left) {
                groups.pop();
            }
            groups.push(Reverse(right));
        }

        groups.len() as i32
    }
}
```
