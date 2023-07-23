# 2091. 从数组中移除最大值和最小值
给你一个下标从 **0** 开始的数组 `nums` ，数组由若干 **互不相同** 的整数组成。

`nums` 中有一个值最小的元素和一个值最大的元素。分别称为 **最小值** 和 **最大值** 。你的目标是从数组中移除这两个元素。

一次 **删除** 操作定义为从数组的 **前面** 移除一个元素或从数组的 **后面** 移除一个元素。

返回将数组中最小值和最大值 都 移除需要的最小删除次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,10,7,5,4,1,8,6]
<strong>输出:</strong> 5
<strong>解释:</strong>
数组中的最小元素是 nums[5] ，值为 1 。
数组中的最大元素是 nums[1] ，值为 10 。
将最大值和最小值都移除需要从数组前面移除 2 个元素，从数组后面移除 3 个元素。
结果是 2 + 3 = 5 ，这是所有可能情况中的最小删除次数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,-4,19,1,8,-2,-3,5]
<strong>输出:</strong> 3
<strong>解释:</strong>
数组中的最小元素是 nums[1] ，值为 -4 。
数组中的最大元素是 nums[2] ，值为 19 。
将最大值和最小值都移除需要从数组前面移除 3 个元素。
结果是 3 ，这是所有可能情况中的最小删除次数。
</pre>

#### Example 3:
<pre>
<strong>输入:</strong> nums = [101]
<strong>输出:</strong> 1
<strong>解释:</strong>
数组中只有这一个元素，那么它既是数组中的最小值又是数组中的最大值。
移除它只需要 1 次删除操作。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>
* `nums` 中的整数 **互不相同**

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let min_index = nums.iter().enumerate().min_by_key(|(_, &x)| x).unwrap().0;
        let max_index = nums.iter().enumerate().max_by_key(|(_, &x)| x).unwrap().0;
        let left = min_index.min(max_index);
        let right = min_index.max(max_index);

        (right + 1)
            .min(nums.len() - left)
            .min(left + 1 + nums.len() - right) as i32
    }
}
```
