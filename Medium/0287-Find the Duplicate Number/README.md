# 287. Find the Duplicate Number
Given an array *nums* containing *n* + 1 integers where each integer is between 1 and *n* (inclusive), prove that at least one duplicate number must exist. Assume that there is only one duplicate number, find the duplicate one.

#### Example 1:
<pre>
<strong>Input:</strong> [1,3,4,2,2]
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [3,1,3,4,2]
<strong>Output:</strong> 3
</pre>

#### Note:
1. You **must not** modify the array (assume the array is read only).
2. You must use only constant, *O*(1) extra space.
3. Your runtime complexity should be less than *O*(*n*<sup>2</sup>).
4. There is only one duplicate number in the array, but it could be repeated more than once.

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut p1 = 0;
        let mut p2 = 0;

        loop {
            p1 = nums[p1 as usize];
            p2 = nums[nums[p2 as usize] as usize];
            if p1 == p2 {
                break;
            }
        }

        p1 = 0;
        while p1 != p2 {
            p1 = nums[p1 as usize];
            p2 = nums[p2 as usize];
        }

        p1
    }
}
```
