# 1851. Minimum Interval to Include Each Query
You are given a 2D integer array `intervals`, where <code>intervals[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> describes the <code>i<sup>th</sup></code> interval starting at <code>left<sub>i</sub></code> and ending at <code>right<sub>i</sub></code> (**inclusive**). The **size** of an interval is defined as the number of integers it contains, or more formally <code>right<sub>i</sub> - left<sub>i</sub> + 1</code>.

You are also given an integer array `queries`. The answer to the <code>j<sup>th</sup></code> query is the **size of the smallest interval** `i` such that <code>left<sub>i</sub> <= queries[j] <= right<sub>i</sub></code>. If no such interval exists, the answer is `-1`.

Return *an array containing the answers to the queries*.

#### Example 1:
<pre>
<strong>Input:</strong> intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]
<strong>Output:</strong> [3,3,1,4]
<strong>Explanation:</strong> The queries are processed as follows:
- Query = 2: The interval [2,4] is the smallest interval containing 2. The answer is 4 - 2 + 1 = 3.
- Query = 3: The interval [2,4] is the smallest interval containing 3. The answer is 4 - 2 + 1 = 3.
- Query = 4: The interval [4,4] is the smallest interval containing 4. The answer is 4 - 4 + 1 = 1.
- Query = 5: The interval [3,6] is the smallest interval containing 5. The answer is 6 - 3 + 1 = 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> intervals = [[2,3],[2,5],[1,8],[20,25]], queries = [2,19,5,22]
<strong>Output:</strong> [2,-1,4,6]
<strong>Explanation:</strong> The queries are processed as follows:
- Query = 2: The interval [2,3] is the smallest interval containing 2. The answer is 3 - 2 + 1 = 2.
- Query = 19: None of the intervals contain 19. The answer is -1.
- Query = 5: The interval [2,5] is the smallest interval containing 5. The answer is 5 - 2 + 1 = 4.
- Query = 22: The interval [20,25] is the smallest interval containing 22. The answer is 25 - 20 + 1 = 6.
</pre>

#### Constraints:
* <code>1 <= intervals.length <= 10<sup>5</sup></code>
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* `intervals[i].length == 2`
* <code>1 <= left<sub>i</sub> <= right<sub>i</sub> <= 10<sup>7</sup></code>
* <code>1 <= queries[j] <= 10<sup>7</sup></code>

## Solutions (Rust)

### 1. Solution
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
