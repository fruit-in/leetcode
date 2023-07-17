# 2217. Find Palindrome With Fixed Length
Given an integer array `queries` and a **positive** integer `intLength`, return *an array* `answer` *where* `answer[i]` *is either the* <code>queries[i]<sup>th</sup></code> *smallest **positive palindrome** of length* `intLength` *or* `-1` *if no such palindrome exists*.

A **palindrome** is a number that reads the same backwards and forwards. Palindromes cannot have leading zeros.

#### Example 1:
<pre>
<strong>Input:</strong> queries = [1,2,3,4,5,90], intLength = 3
<strong>Output:</strong> [101,111,121,131,141,999]
<strong>Explanation:</strong>
The first few palindromes of length 3 are:
101, 111, 121, 131, 141, 151, 161, 171, 181, 191, 202, ...
The 90<sup>th</sup> palindrome of length 3 is 999.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> queries = [2,4,6], intLength = 4
<strong>Output:</strong> [1111,1331,1551]
<strong>Explanation:</strong>
The first six palindromes of length 4 are:
1001, 1111, 1221, 1331, 1441, and 1551.
</pre>

#### Constraints:
* <code>1 <= queries.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= queries[i] <= 10<sup>9</sup></code>
* `1 <= intLength <= 15`

## Solutions (Rust)

### 1. Solution
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
