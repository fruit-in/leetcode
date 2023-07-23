# 2177. Find Three Consecutive Integers That Sum to a Given Number
Given an integer `num`, return *three consecutive integers (as a sorted array) that **sum** to* `num`. If `num` cannot be expressed as the sum of three consecutive integers, return *an **empty** array*.

#### Example 1:
<pre>
<strong>Input:</strong> num = 33
<strong>Output:</strong> [10,11,12]
<strong>Explanation:</strong> 33 can be expressed as 10 + 11 + 12 = 33.
10, 11, 12 are 3 consecutive integers, so we return [10, 11, 12].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 4
<strong>Output:</strong> []
<strong>Explanation:</strong> There is no way to express 4 as the sum of 3 consecutive integers.
</pre>

#### Constraints:
* <code>0 <= num <= 10<sup>15</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 == 0 {
            vec![num / 3 - 1, num / 3, num / 3 + 1]
        } else {
            vec![]
        }
    }
}
```
