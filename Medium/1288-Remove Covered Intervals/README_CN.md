# 1288. 删除被覆盖区间
给你一个区间列表，请你删除列表中被其他区间所覆盖的区间。

只有当 `c <= a` 且 `b <= d` 时，我们才认为区间 `[a,b)` 被区间 `[c,d)` 覆盖。

在完成所有删除操作后，请你返回列表中剩余区间的数目。

#### 示例:
<pre>
<strong>输入:</strong> intervals = [[1,4],[3,6],[2,8]]
<strong>输出:</strong> 2
<strong>解释:</strong> 区间 [3,6] 被区间 [2,8] 覆盖，所以它被删除了。
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> intervals = [[1,4],[2,3]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= intervals.length <= 1000`
* `0 <= intervals[i][0] < intervals[i][1] <= 10^5`
* 对于所有的 `i != j`：`intervals[i] != intervals[j]`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let mut max_r = 0;
        let mut ret = intervals.len() as i32;
        intervals.sort_unstable_by_key(|interval| (interval[0], -interval[1]));

        for interval in intervals {
            if interval[1] <= max_r {
                ret -= 1;
            } else {
                max_r = interval[1];
            }
        }

        ret
    }
}
```
