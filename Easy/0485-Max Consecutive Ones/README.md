# 485. Max Consecutive Ones
Given a binary array, find the maximum number of consecutive 1s in this array.

#### Example 1:
<pre>
<strong>Input:</strong> [1,1,0,1,1,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The first two digits or the last three digits are consecutive 1s.
The maximum number of consecutive 1s is 3.
</pre>

#### Note:
* The input array will only contain <code>0</code> and <code>1</code>.
* The length of input array is a positive integer and will not exceed 10,000

## Solutions (Rust)

### 1. Linear Scan
```Rust
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut m = 0;
        for n in nums {
            if n == 1 {
                i += 1;
            } else {
                m = m.max(i);
                i = 0;
            }
        }
        m.max(i)
    }
}
```
