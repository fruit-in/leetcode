# 2438. 二的幂数组中查询范围内的乘积
给你一个正整数 `n` ，你需要找到一个下标从 **0** 开始的数组 `powers` ，它包含 **最少** 数目的 `2` 的幂，且它们的和为 `n` 。`powers` 数组是 **非递减** 顺序的。根据前面描述，构造 `powers` 数组的方法是唯一的。

同时给你一个下标从 **0** 开始的二维整数数组 `queries` ，其中 <code>queries[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> ，其中 `queries[i]` 表示请你求出满足 <code>left<sub>i</sub> <= j <= right<sub>i</sub></code> 的所有 `powers[j]` 的乘积。

请你返回一个数组 `answers` ，长度与 `queries` 的长度相同，其中 `answers[i]`是第 `i` 个查询的答案。由于查询的结果可能非常大，请你将每个 `answers[i]` 都对 <code>10<sup>9</sup> + 7</code> 取余 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 15, queries = [[0,1],[2,2],[0,3]]
<strong>输出:</strong> [2,4,64]
<strong>解释:</strong>
对于 n = 15 ，得到 powers = [1,2,4,8] 。没法得到元素数目更少的数组。
第 1 个查询的答案：powers[0] * powers[1] = 1 * 2 = 2 。
第 2 个查询的答案：powers[2] = 4 。
第 3 个查询的答案：powers[0] * powers[1] * powers[2] * powers[3] = 1 * 2 * 4 * 8 = 64 。
每个答案对 109 + 7 得到的结果都相同，所以返回 [2,4,64] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2, queries = [[0,0]]
<strong>输出:</strong> [2]
<strong>解释:</strong>
对于 n = 2, powers = [2] 。
唯一一个查询的答案是 powers[0] = 2 。答案对 109 + 7 取余后结果相同，所以返回 [2] 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* <code>0 <= start<sub>i</sub> <= end<sub>i</sub> < powers.length</code>

## 题解 (Rust)

### 1. 题解
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
