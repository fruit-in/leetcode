# 2462. Total Cost to Hire K Workers
You are given a **0-indexed** integer array `costs` where `costs[i]` is the cost of hiring the <code>i<sup>th</sup></code> worker.

You are also given two integers `k` and `candidates`. We want to hire exactly `k` workers according to the following rules:

* You will run `k` sessions and hire exactly one worker in each session.
* In each hiring session, choose the worker with the lowest cost from either the first `candidates` workers or the last `candidates` workers. Break the tie by the smallest index.
    * For example, if `costs = [3,2,7,7,1,2]` and `candidates = 2`, then in the first hiring session, we will choose the <code>4<sup>th</sup></code> worker because they have the lowest cost `[3,2,7,7,1,2]`.
    * In the second hiring session, we will choose <code>1<sup>st</sup></code> worker because they have the same lowest cost as <code>4<sup>th</sup></code> worker but they have the smallest index `[3,2,7,7,2]`. Please note that the indexing may be changed in the process.
* If there are fewer than candidates workers remaining, choose the worker with the lowest cost among them. Break the tie by the smallest index.
* A worker can only be chosen once.

Return *the total cost to hire exactly* `k` *workers*.

#### Example 1:
<pre>
<strong>Input:</strong> costs = [17,12,10,2,7,2,11,20,8], k = 3, candidates = 4
<strong>Output:</strong> 11
<strong>Explanation:</strong> We hire 3 workers in total. The total cost is initially 0.
- In the first hiring round we choose the worker from [17,12,10,2,7,2,11,20,8]. The lowest cost is 2, and we break the tie by the smallest index, which is 3. The total cost = 0 + 2 = 2.
- In the second hiring round we choose the worker from [17,12,10,7,2,11,20,8]. The lowest cost is 2 (index 4). The total cost = 2 + 2 = 4.
- In the third hiring round we choose the worker from [17,12,10,7,11,20,8]. The lowest cost is 7 (index 3). The total cost = 4 + 7 = 11. Notice that the worker with index 3 was common in the first and last four workers.
The total hiring cost is 11.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> costs = [1,2,4,1], k = 3, candidates = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> We hire 3 workers in total. The total cost is initially 0.
- In the first hiring round we choose the worker from [1,2,4,1]. The lowest cost is 1, and we break the tie by the smallest index, which is 0. The total cost = 0 + 1 = 1. Notice that workers with index 1 and 2 are common in the first and last 3 workers.
- In the second hiring round we choose the worker from [2,4,1]. The lowest cost is 1 (index 2). The total cost = 1 + 1 = 2.
- In the third hiring round there are less than three candidates. We choose the worker from the remaining workers [2,4]. The lowest cost is 2 (index 0). The total cost = 2 + 2 = 4.
The total hiring cost is 4.
</pre>

#### Constraints:
* <code>1 <= costs.length <= 10<sup>5</sup></code>
* <code>1 <= costs[i] <= 10<sup>5</sup></code>
* `1 <= k, candidates <= costs.length`

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut heap = BinaryHeap::new();
        let mut left = 0;
        let mut right = costs.len() - 1;
        let mut ret = 0;

        for _ in 0..candidates {
            if left <= right {
                heap.push(Reverse((costs[left], left)));
                left += 1;
            }
            if right >= left {
                heap.push(Reverse((costs[right], right)));
                right -= 1;
            }
        }

        for _ in 0..k {
            let Reverse((cost, i)) = heap.pop().unwrap();

            if left <= right {
                if i < left {
                    heap.push(Reverse((costs[left], left)));
                    left += 1;
                } else {
                    heap.push(Reverse((costs[right], right)));
                    right -= 1;
                }
            }

            ret += cost as i64;
        }

        ret
    }
}
```
