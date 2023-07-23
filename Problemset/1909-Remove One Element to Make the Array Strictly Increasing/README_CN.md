# 1909. 删除一个元素使数组严格递增
给你一个下标从 **0** 开始的整数数组 `nums` ，如果 **恰好** 删除 **一个** 元素后，数组 **严格递增** ，那么请你返回 `true` ，否则返回 `false` 。如果数组本身已经是严格递增的，请你也返回 `true` 。

数组 `nums` 是 **严格递增** 的定义为：对于任意下标的 1 <= i < nums.length 都满足 nums[i - 1] < nums[i] 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,10,5,7]
<strong>输出:</strong> true
<strong>解释:</strong> 从 nums 中删除下标 2 处的 10 ，得到 [1,2,5,7] 。
[1,2,5,7] 是严格递增的，所以返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,1,2]
<strong>输出:</strong> false
<strong>解释:</strong>
[3,1,2] 是删除下标 0 处元素后得到的结果。
[2,1,2] 是删除下标 1 处元素后得到的结果。
[2,3,2] 是删除下标 2 处元素后得到的结果。
[2,3,1] 是删除下标 3 处元素后得到的结果。
没有任何结果数组是严格递增的，所以返回 false 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,1]
<strong>输出:</strong> false
<strong>解释:</strong> 删除任意元素后的结果都是 [1,1] 。
[1,1] 不是严格递增的，所以返回 false 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong> true
<strong>解释:</strong> [1,2,3] 已经是严格递增的，所以返回 true 。
</pre>

#### 提示:
* `2 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let mut removed = false;

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                if removed {
                    return false;
                }
                if i > 1 && nums[i] <= nums[i - 2] {
                    nums[i] = nums[i - 1];
                }
                removed = true;
            }
        }

        true
    }
}
```
