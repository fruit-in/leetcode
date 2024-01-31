# 2216. Minimum Deletions to Make Array Beautiful
You are given a **0-indexed** integer array `nums`. The array `nums` is **beautiful** if:

* `nums.length` is even.
* `nums[i] != nums[i + 1]` for all `i % 2 == 0`.

Note that an empty array is considered beautiful.

You can delete any number of elements from `nums`. When you delete an element, all the elements to the right of the deleted element will be **shifted one unit to the left** to fill the gap created and all the elements to the left of the deleted element will remain **unchanged**.

Return *the **minimum** number of elements to delete from* `nums` *to make it beautiful*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,2,3,5]
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can delete either nums[0] or nums[1] to make nums = [1,2,3,5] which is beautiful. It can be proven you need at least 1 deletion to make nums beautiful.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,2,2,3,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can delete nums[0] and nums[5] to make nums = [1,2,2,3] which is beautiful. It can be proven you need at least 2 deletions to make nums beautiful.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut beautiful = vec![];

        for i in 0..nums.len() {
            if beautiful.len() % 2 == 0 || *beautiful.last().unwrap_or(&-1) != nums[i] {
                beautiful.push(nums[i]);
            }
        }

        if beautiful.len() % 2 == 1 {
            beautiful.pop();
        }

        (nums.len() - beautiful.len()) as i32
    }
}
```
