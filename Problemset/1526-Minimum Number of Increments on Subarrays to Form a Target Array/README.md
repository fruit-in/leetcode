# 1526. Minimum Number of Increments on Subarrays to Form a Target Array
You are given an integer array `target`. You have an integer array `initial` of the same size as `target` with all elements initially zeros.

In one operation you can choose **any** subarray from `initial` and increment each value by one.

Return *the minimum number of operations to form a* `target` *array from* `initial`.

The test cases are generated so that the answer fits in a 32-bit integer.

#### Example 1:
<pre>
<strong>Input:</strong> target = [1,2,3,2,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We need at least 3 operations to form the target array from the initial array.
[0,0,0,0,0] increment 1 from index 0 to 4 (inclusive).
[1,1,1,1,1] increment 1 from index 1 to 3 (inclusive).
[1,2,2,2,1] increment 1 at index 2.
[1,2,3,2,1] target array is formed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = [3,1,1,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong> [0,0,0,0] -> [1,1,1,1] -> [1,1,1,2] -> [2,1,1,2] -> [3,1,1,2]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> target = [3,1,5,4,2]
<strong>Output:</strong> 7
<strong>Explanation:</strong> [0,0,0,0,0] -> [1,1,1,1,1] -> [2,1,1,1,1] -> [3,1,1,1,1] -> [3,1,2,2,2] -> [3,1,3,3,2] -> [3,1,4,4,2] -> [3,1,5,4,2].
</pre>

#### Constraints:
* <code>1 <= target.length <= 10<sup>5</sup></code>
* <code>1 <= target[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut ret = target[0];

        for i in 1..target.len() {
            ret += 0.max(target[i] - target[i - 1]);
        }

        ret
    }
}
```
