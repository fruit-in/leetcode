# 1851. 包含每个查询的最小区间
给你一个二维整数数组 `intervals` ，其中 <code>intervals[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> 表示第 `i` 个区间开始于 <code>left<sub>i</sub></code> 、结束于 <code>right<sub>i</sub></code>（包含两侧取值，**闭区间**）。区间的 **长度** 定义为区间中包含的整数数目，更正式地表达是 <code>right<sub>i</sub> - left<sub>i</sub> + 1</code> 。

再给你一个整数数组 `queries` 。第 `j` 个查询的答案是满足 <code>left<sub>i</sub> <= queries[j] <= right<sub>i</sub></code> 的 **长度最小区间** `i` **的长度** 。如果不存在这样的区间，那么答案是 `-1` 。

以数组形式返回对应查询的所有答案。

#### 示例 1:
<pre>
<strong>输入:</strong> intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]
<strong>输出:</strong> [3,3,1,4]
<strong>解释:</strong> 查询处理如下：
- Query = 2 ：区间 [2,4] 是包含 2 的最小区间，答案为 4 - 2 + 1 = 3 。
- Query = 3 ：区间 [2,4] 是包含 3 的最小区间，答案为 4 - 2 + 1 = 3 。
- Query = 4 ：区间 [4,4] 是包含 4 的最小区间，答案为 4 - 4 + 1 = 1 。
- Query = 5 ：区间 [3,6] 是包含 5 的最小区间，答案为 6 - 3 + 1 = 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> intervals = [[2,3],[2,5],[1,8],[20,25]], queries = [2,19,5,22]
<strong>输出:</strong> [2,-1,4,6]
<strong>解释:</strong> 查询处理如下：
- Query = 2 ：区间 [2,3] 是包含 2 的最小区间，答案为 3 - 2 + 1 = 2 。
- Query = 19：不存在包含 19 的区间，答案为 -1 。
- Query = 5 ：区间 [2,5] 是包含 5 的最小区间，答案为 5 - 2 + 1 = 4 。
- Query = 22：区间 [20,25] 是包含 22 的最小区间，答案为 25 - 20 + 1 = 6 。
</pre>

#### 提示:
* <code>1 <= intervals.length <= 10<sup>5</sup></code>
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* `intervals[i].length == 2`
* <code>1 <= left<sub>i</sub> <= right<sub>i</sub> <= 10<sup>7</sup></code>
* <code>1 <= queries[j] <= 10<sup>7</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut queries = (0..queries.len())
            .map(|i| (queries[i], i))
            .collect::<Vec<_>>();
        let mut i = 0;
        let mut heap = BinaryHeap::new();
        let mut ret = vec![-1; queries.len()];

        intervals.sort_unstable();
        queries.sort_unstable();

        for &(query, j) in &queries {
            while i < intervals.len() && intervals[i][0] <= query {
                heap.push(Reverse((
                    intervals[i][1] - intervals[i][0] + 1,
                    intervals[i][1],
                )));
                i += 1;
            }

            while let Some(&Reverse((size, right))) = heap.peek() {
                if right < query {
                    heap.pop();
                } else {
                    ret[j] = size;
                    break;
                }
            }
        }

        ret
    }
}
```
