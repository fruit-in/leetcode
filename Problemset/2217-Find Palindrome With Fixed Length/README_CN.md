# 2217. 找到指定长度的回文数
给你一个整数数组 `queries` 和一个 **正** 整数 `intLength` ，请你返回一个数组 `answer` ，其中 `answer[i]` 是长度为 `intLength` 的 **正回文数** 中第 `queries[i]` 小的数字，如果不存在这样的回文数，则为 `-1` 。

**回文数** 指的是从前往后和从后往前读一模一样的数字。回文数不能有前导 0 。

#### 示例 1:
<pre>
<strong>输入:</strong> queries = [1,2,3,4,5,90], intLength = 3
<strong>输出:</strong> [101,111,121,131,141,999]
<strong>解释:</strong>
长度为 3 的最小回文数依次是：
101, 111, 121, 131, 141, 151, 161, 171, 181, 191, 202, ...
第 90 个长度为 3 的回文数是 999 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> queries = [2,4,6], intLength = 4
<strong>输出:</strong> [1111,1331,1551]
<strong>解释:</strong>
长度为 4 的前 6 个回文数是：
1001, 1111, 1221, 1331, 1441 和 1551 。
</pre>

#### 提示:
* <code>1 <= queries.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= queries[i] <= 10<sup>9</sup></code>
* `1 <= intLength <= 15`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let mut ret = Vec::with_capacity(queries.len());

        for &query in &queries {
            let half_length = (int_length as u32 + 1) / 2;

            if query >= 9 * 10_i32.pow(half_length - 1) + 1 {
                ret.push(-1);
                continue;
            }

            let mut x = query as i64 + 10_i64.pow(half_length - 1) - 1;
            let mut y = x;

            if int_length % 2 == 1 {
                y /= 10;
            }

            for _ in 0..int_length / 2 {
                x = x * 10 + y % 10;
                y /= 10;
            }

            ret.push(x);
        }

        ret
    }
}
```
