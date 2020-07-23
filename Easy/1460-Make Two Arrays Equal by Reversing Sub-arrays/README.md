# 1460. Make Two Arrays Equal by Reversing Sub-arrays
Given two integer arrays of equal length `target` and `arr`.

In one step, you can select any **non-empty sub-array** of `arr` and reverse it. You are allowed to make any number of steps.

Return *True* if you can make `arr` equal to `target`, or *False* otherwise.

#### Example 1:
<pre>
<strong>Input:</strong> target = [1,2,3,4], arr = [2,4,1,3]
<strong>Output:</strong> true
<strong>Explanation:</strong> You can follow the next steps to convert arr to target:
1- Reverse sub-array [2,4,1], arr becomes [1,4,2,3]
2- Reverse sub-array [4,2], arr becomes [1,2,4,3]
3- Reverse sub-array [4,3], arr becomes [1,2,3,4]
There are multiple ways to convert arr to target, this is not the only way to do so.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = [7], arr = [7]
<strong>Output:</strong> true
<strong>Explanation:</strong> arr is equal to target without any reverses.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> target = [1,12], arr = [12,1]
<strong>Output:</strong> true
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> target = [3,7,9], arr = [3,7,11]
<strong>Output:</strong> false
<strong>Explanation:</strong> arr doesn't have value 9 and it can never be converted to target.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> target = [1,1,1,1,1], arr = [1,1,1,1,1]
<strong>Output:</strong> true
</pre>

#### Constraints:
* `target.length == arr.length`
* `1 <= target.length <= 1000`
* `1 <= target[i] <= 1000`
* `1 <= arr[i] <= 1000`

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut target = target;
        let mut arr = arr;
        target.sort_unstable();
        arr.sort_unstable();

        target == arr
    }
}
```
