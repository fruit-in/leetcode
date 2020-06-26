# 56. 合并区间
给出一个区间的集合，请合并所有重叠的区间。

#### 示例 1:
<pre>
<strong>输入:</strong> [[1,3],[2,6],[8,10],[15,18]]
<strong>输出:</strong> [[1,6],[8,10],[15,18]]
<strong>解释:</strong> 区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [[1,4],[4,5]]
<strong>输出:</strong> [[1,5]]
<strong>解释:</strong> 区间 [1,4] 和 [4,5] 可被视为重叠区间。
</pre>

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| b.cmp(a));
        let mut ret = vec![];
        let mut prev = intervals.pop().unwrap();

        while let Some(curr) = intervals.pop() {
            if curr[0] > prev[1] {
                ret.push(prev);
                prev = curr;
            } else {
                prev[1] = prev[1].max(curr[1]);
            }
        }
        ret.push(prev);

        ret
    }
}
```
