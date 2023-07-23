# 2055. Plates Between Candles
There is a long table with a line of plates and candles arranged on top of it. You are given a **0-indexed** string `s` consisting of characters `'*'` and `'|'` only, where a `'*'` represents a **plate** and a `'|'` represents a **candle**.

You are also given a **0-indexed** 2D integer array `queries` where <code>queries[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> denotes the **substring** <code>s[left<sub>i</sub>...right<sub>i</sub>]</code> (**inclusive**). For each query, you need to find the **number** of plates **between candles** that are in **the substring**. A plate is considered **between candles** if there is at least one candle to its left **and** at least one candle to its right **in the substring**.

* For example, `s = "||**||**|*"`, and a query `[3, 8]` denotes the substring `"*||**|"`. The number of plates between candles in this substring is `2`, as each of the two plates has at least one candle **in the substring** to its left **and** right.

Return *an integer array* `answer` *where* `answer[i]` *is the answer to the* <code>i<sup>th</sup></code> *query*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/10/04/ex-1.png)
<pre>
<strong>Input:</strong> s = "**|**|***|", queries = [[2,5],[5,9]]
<strong>Output:</strong> [2,3]
<strong>Explanation:</strong>
- queries[0] has two plates between candles.
- queries[1] has three plates between candles.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/10/04/ex-2.png)
<pre>
<strong>Input:</strong> s = "***|**|*****|**||**|*", queries = [[1,17],[4,5],[14,17],[5,11],[15,16]]
<strong>Output:</strong> [9,0,0,0,0]
<strong>Explanation:</strong>
- queries[0] has nine plates between candles.
- The other queries have zero plates between candles.
</pre>

#### Constraints:
* <code>3 <= s.length <= 10<sup>5</sup></code>
* s consists of '*' and '|' characters.
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* `queries[i].length == 2`
* <code>0 <= left<sub>i</sub> <= right<sub>i</sub> < s.length</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let candles = s
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'|')
            .map(|(i, _)| i as i32)
            .collect::<Vec<_>>();
        let mut answer = vec![0; queries.len()];

        for i in 0..answer.len() {
            let j = match candles.binary_search(&queries[i][0]) {
                Ok(x) | Err(x) => x,
            };
            let k = match candles.binary_search(&(queries[i][1] + 1)) {
                Ok(x) | Err(x) => x.saturating_sub(1),
            };

            if j < k && k < candles.len() {
                answer[i] = candles[k] - candles[j] - (k - j) as i32;
            }
        }

        answer
    }
}
```
