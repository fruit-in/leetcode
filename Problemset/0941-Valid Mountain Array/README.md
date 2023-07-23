# 941. Valid Mountain Array
Given an array <code>A</code> of integers, return <code>true</code> if and only if it is a *valid mountain array*.

Recall that A is a mountain array if and only if:
* <code>A.length >= 3</code>
* There exists some <code>i</code> with <code>0 < i < A.length - 1</code> such that:
    * <code>A[0] < A[1] < ... A[i-1] < A[i]</code>
    * <code>A[i] > A[i+1] > ... > A[A.length - 1]</code>

#### Example 1:
<pre>
<strong>Input:</strong> [2,1]
<strong>Output:</strong> false
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [3,5,5]
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [0,3,2,1]
<strong>Output:</strong> true
</pre>

#### Note:
1. <code>0 <= A.length <= 10000</code>
2. <code>0 <= A[i] <= 10000</code>

## Solutions (Rust)

### 1. One Pass
```Rust
impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let length = a.len();
        let mut i = 0;
        while i + 1 < length && a[i] < a[i + 1] {
            i += 1;
        }
        if i == 0 || i + 1 == length {
            return false;
        }
        while i + 1 < length && a[i] > a[i + 1] {
            i += 1;
        }
        i + 1 == length
    }
}
```
