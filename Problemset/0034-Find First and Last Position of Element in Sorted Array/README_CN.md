# 34. 在排序数组中查找元素的第一个和最后一个位置
给你一个按照非递减顺序排列的整数数组 `nums`，和一个目标值 `target`。请你找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 `target`，返回 `[-1, -1]`。

你必须设计并实现时间复杂度为 `O(log n)` 的算法解决此问题。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [5,7,7,8,8,10], target = 8
<strong>输出:</strong> [3,4]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,7,7,8,8,10], target = 6
<strong>输出:</strong> [-1,-1]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [], target = 0
<strong>输出:</strong> [-1,-1]
</pre>

#### 提示:
* <code>0 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>
* `nums` 是一个非递减数组
* <code>-10<sup>9</sup> <= target <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let n = nums.len();
        let mut left = 0;
        let mut right = n - 1;
        let mut ret = vec![-1, -1];

        while left < right {
            let mid = (left + right) / 2;

            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if nums[left] != target {
            return vec![-1, -1];
        }

        ret[0] = left as i32;
        left = 0;
        right = n - 1;

        while left < right {
            let mid = (left + right + 1) / 2;

            if nums[mid] <= target {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        ret[1] = left as i32;

        ret
    }
}
```
