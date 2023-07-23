# 2554. Maximum Number of Integers to Choose From a Range I
You are given an integer array `banned` and two integers `n` and `maxSum`. You are choosing some number of integers following the below rules:

* The chosen integers have to be in the range `[1, n]`.
* Each integer can be chosen **at most once**.
* The chosen integers should not be in the array `banned`.
* The sum of the chosen integers should not exceed `maxSum`.

Return *the **maximum** number of integers you can choose following the mentioned rules*.

#### Example 1:
<pre>
<strong>Input:</strong> banned = [1,6,5], n = 5, maxSum = 6
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can choose the integers 2 and 4.
2 and 4 are from the range [1, 5], both did not appear in banned, and their sum is 6, which did not exceed maxSum.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> banned = [1,2,3,4,5,6,7], n = 8, maxSum = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> You cannot choose any integer while following the mentioned conditions.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> banned = [11], n = 7, maxSum = 50
<strong>Output:</strong> 7
<strong>Explanation:</strong> You can choose the integers 1, 2, 3, 4, 5, 6, and 7.
They are from the range [1, 7], all did not appear in banned, and their sum is 28, which did not exceed maxSum.
</pre>

#### Constraints:
* <code>1 <= banned.length <= 10<sup>4</sup></code>
* <code>1 <= banned[i], n <= 10<sup>4</sup></code>
* <code>1 <= maxSum <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned = banned.into_iter().collect::<HashSet<_>>();
        let mut sum = 0;
        let mut ret = 0;

        for x in 1..=n {
            if !banned.contains(&x) && sum + x <= max_sum {
                sum += x;
                ret += 1;
            } else if sum + x > max_sum {
                break;
            }
        }

        ret
    }
}
```
