# 2438. Range Product Queries of Powers
Given a positive integer `n`, there exists a **0-indexed** array called `powers`, composed of the **minimum** number of powers of `2` that sum to `n`. The array is sorted in **non-decreasing** order, and there is **only one** way to form the array.

You are also given a **0-indexed** 2D integer array `queries`, where <code>queries[i] = [left<sub>i</sub>, right<sub>i</sub>]</code>. Each `queries[i]` represents a query where you have to find the product of all `powers[j]` with <code>left<sub>i</sub> <= j <= right<sub>i</sub></code>.

Return *an array* `answers`, *equal in length to* `queries`, *where* `answers[i]` *is the answer to the* <code>i<sup>th</sup></code> *query*. Since the answer to the <code>i<sup>th</sup></code> query may be too large, each `answers[i]` should be returned **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 15, queries = [[0,1],[2,2],[0,3]]
<strong>Output:</strong> [2,4,64]
<strong>Explanation:</strong>
For n = 15, powers = [1,2,4,8]. It can be shown that powers cannot be a smaller size.
Answer to 1st query: powers[0] * powers[1] = 1 * 2 = 2.
Answer to 2nd query: powers[2] = 4.
Answer to 3rd query: powers[0] * powers[1] * powers[2] * powers[3] = 1 * 2 * 4 * 8 = 64.
Each answer modulo 109 + 7 yields the same answer, so [2,4,64] is returned.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, queries = [[0,0]]
<strong>Output:</strong> [2]
<strong>Explanation:</strong>
For n = 2, powers = [2].
The answer to the only query is powers[0] = 2. The answer modulo 109 + 7 is the same, so [2] is returned.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* <code>0 <= start<sub>i</sub> <= end<sub>i</sub> < powers.length</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = vec![];
        let mut answers = vec![1_i64; queries.len()];

        for i in 0..30 {
            if n & (1 << i) != 0 {
                powers.push(1 << i);
            }
        }

        for i in 0..queries.len() {
            for j in queries[i][0] as usize..=queries[i][1] as usize {
                answers[i] = (answers[i] * powers[j]) % 1_000_000_007;
            }
        }

        answers.into_iter().map(|x| x as i32).collect()
    }
}
```
