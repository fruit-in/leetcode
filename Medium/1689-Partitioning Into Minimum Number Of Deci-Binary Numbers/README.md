# 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
A decimal number is called **deci-binary** if each of its digits is either `0` or `1` without any leading zeros. For example, `101` and `1100` are **deci-binary**, while `112` and `3001` are not.

Given a string `n` that represents a positive decimal integer, return *the **minimum** number of positive **deci-binary** numbers needed so that they sum up to* `n`.

#### Example 1:
<pre>
<strong>Input:</strong> n = "32"
<strong>Output:</strong> 3
<strong>Explanation:</strong> 10 + 11 + 11 = 32
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = "82734"
<strong>Output:</strong> 8
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = "27346209830709182346"
<strong>Output:</strong> 9
</pre>

#### Constraints:
* <code>1 <= n.length <= 10<sup>5</sup></code>
* `n` consists of only digits.
* `n` does not contain any leading zeros and represents a positive integer.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} n
# @return {Integer}
def min_partitions(n)
  n.chars.max.to_i
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.bytes().max().unwrap() as i32 - 48
    }
}
```
