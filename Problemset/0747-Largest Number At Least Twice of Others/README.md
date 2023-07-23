# 747. Largest Number At Least Twice of Others
In a given integer array <code>nums</code>, there is always exactly one largest element.

Find whether the largest element in the array is at least twice as much as every other number in the array.

If it is, return the **index** of the largest element, otherwise return -1.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3, 6, 1, 0]
<strong>Output:</strong> 1
<strong>Explanation:</strong> 6 is the largest integer, and for every other number in the array x,
6 is more than twice as big as x.  The index of value 6 is 1, so we return 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1, 2, 3, 4]
<strong>Output:</strong> -1
<strong>Explanation:</strong> 4 isn't at least as big as twice the value of 3, so we return -1.
</pre>

#### Note:
1. <code>nums</code> will have a length in the range <code>[1, 50]</code>.
2. Every <code>nums[i]</code> will be an integer in the range <code>[0, 99]</code>.

## Solutions (Rust)

### 1. Linear Scan
```Rust
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap();
        if nums.iter().all(|&x| 2 * x <= m || x == m) {
            nums.iter().position(|&x| x == m).unwrap() as i32
        } else {
            -1
        }
    }
}
```
