# 33. 搜索旋转排序数组
假设按照升序排序的数组在预先未知的某个点上进行了旋转。

( 例如，数组 `[0,1,2,4,5,6,7]` 可能变为 `[4,5,6,7,0,1,2]` )。

搜索一个给定的目标值，如果数组中存在这个目标值，则返回它的索引，否则返回 `-1` 。

你可以假设数组中不存在重复的元素。

你的算法时间复杂度必须是 *O*(log *n*) 级别。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,5,6,7,0,1,2], target = 0
<strong>输出:</strong> 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,5,6,7,0,1,2], target = 3
<strong>输出:</strong> -1
</pre>

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let m = (l + r) / 2;
            if target == nums[m] {
                return m as i32;
            }
            if nums[l] <= nums[m] && nums[m] <= nums[r - 1] {
                if target < nums[m] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else if nums[l] >= nums[m] {
                if target < nums[m] || target >= nums[l] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else if nums[m] >= nums[r - 1] {
                if target < nums[m] && target >= nums[l] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
        }

        -1
    }
}
```
