# 435. 无重叠区间
给定一个区间的集合 `intervals` ，其中 <code>intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 。返回 *需要移除区间的最小数量，使剩余区间互不重叠* 。

#### 示例 1:
<pre>
<strong>输入:</strong> intervals = [[1,2],[2,3],[3,4],[1,3]]
<strong>输出:</strong> 1
<strong>解释:</strong> 移除 [1,3] 后，剩下的区间没有重叠。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> intervals = [[1,2],[1,2],[1,2]]
<strong>输出:</strong> 2
<strong>解释:</strong> 你需要移除两个 [1,2] 来使剩下的区间没有重叠。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> intervals = [[1,2],[2,3]]
<strong>输出:</strong> 0
<strong>解释:</strong> 你不需要移除任何区间，因为它们已经是无重叠的了。
</pre>

#### 提示:
* <code>1 <= intervals.length <= 105</sup></code>
* `intervals[i].length == 2`
* <code>-5 * 10<sup>4</sup> <= start<sub>i</sub> < end<sub>i</sub> <= 5 * 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let mut stack = vec![];

        intervals.sort_unstable();

        for i in 0..intervals.len() {
            let (start0, end0) = (intervals[i][0], intervals[i][1]);
            let &(start1, end1) = stack.last().unwrap_or(&(0, start0));

            if start0 >= end1 {
                stack.push((start0, end0));
            } else if end0 <= end1 {
                stack.pop();
                stack.push((start0, end0));
            }
        }

        (intervals.len() - stack.len()) as i32
    }
}
```
