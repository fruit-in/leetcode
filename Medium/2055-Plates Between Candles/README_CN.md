# 2055. 蜡烛之间的盘子
给你一个长桌子，桌子上盘子和蜡烛排成一列。给你一个下标从 **0** 开始的字符串 `s` ，它只包含字符 `'*'` 和 `'|'` ，其中 `'*'` 表示一个 **盘子** ，`'|'` 表示一支 **蜡烛** 。

同时给你一个下标从 **0** 开始的二维整数数组 `queries` ，其中 <code>queries[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> 表示 **子字符串** <code>s[left<sub>i</sub>...right<sub>i</sub>]</code> （**包含左右端点的字符**）。对于每个查询，你需要找到 **子字符串中** 在 **两支蜡烛之间** 的盘子的 **数目** 。如果一个盘子在 **子字符串中** 左边和右边 **都** 至少有一支蜡烛，那么这个盘子满足在 **两支蜡烛之间** 。

* 比方说，`s = "||**||**|*"` ，查询 `[3, 8]` ，表示的是子字符串 `"*||**|"` 。子字符串中在两支蜡烛之间的盘子数目为 `2` ，子字符串中右边两个盘子在它们左边和右边 都 至少有一支蜡烛。

请你返回一个整数数组 `answer` ，其中 `answer[i]` 是第 `i` 个查询的答案。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/04/ex-1.png)
<pre>
<strong>输入:</strong> s = "**|**|***|", queries = [[2,5],[5,9]]
<strong>输出:</strong> [2,3]
<strong>解释:</strong>
- queries[0] 有两个盘子在蜡烛之间。
- queries[1] 有三个盘子在蜡烛之间。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/04/ex-2.png)
<pre>
<strong>输入:</strong> s = "***|**|*****|**||**|*", queries = [[1,17],[4,5],[14,17],[5,11],[15,16]]
<strong>输出:</strong> [9,0,0,0,0]
<strong>解释:</strong>
- queries[0] 有 9 个盘子在蜡烛之间。
- 另一个查询没有盘子在蜡烛之间。
</pre>

#### 提示:
* <code>3 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含字符 `'*'` 和 `'|'` 。
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* `queries[i].length == 2`
* <code>0 <= left<sub>i</sub> <= right<sub>i</sub> < s.length</code>

## 题解 (Rust)

### 1. 题解
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
