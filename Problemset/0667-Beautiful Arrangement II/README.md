# 667. Beautiful Arrangement II
Given two integers `n` and `k`, construct a list `answer` that contains `n` different positive integers ranging from `1` to `n` and obeys the following requirement:
* Suppose this list is <code>answer = [a<sub>1</sub>, a<sub>2</sub>, a<sub>3</sub>, ... , a<sub>n</sub>]</code>, then the list <code>[|a<sub>1</sub> - a<sub>2</sub>|, |a<sub>2</sub> - a<sub>3</sub>|, |a<sub>3</sub> - a<sub>4</sub>|, ... , |a<sub>n-1</sub> - a<sub>n</sub>|]</code> has exactly `k` distinct integers.

Return *the list* `answer`. If there multiple valid answers, return **any of them**.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, k = 1
<strong>Output:</strong> [1,2,3]
<strong>Explanation:</strong> The [1,2,3] has three different positive integers ranging from 1 to 3, and the [1,1] has exactly 1 distinct integer: 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, k = 2
<strong>Output:</strong> [1,3,2]
<strong>Explanation:</strong> The [1,3,2] has three different positive integers ranging from 1 to 3, and the [2,1] has exactly 2 distinct integers: 1 and 2.
</pre>

#### Constraints:
* <code>1 <= k < n <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut answer = vec![];

        for x in 1..=k / 2 {
            answer.push(x);
            answer.push(n + 1 - x);
        }

        if k % 2 == 1 {
            for x in k / 2 + 1..=n - k / 2 {
                answer.push(x);
            }
        } else {
            for x in (k / 2 + 1..=n - k / 2).rev() {
                answer.push(x);
            }
        }

        answer
    }
}
```
