# 2216. 美化数组的最少删除数
给你一个下标从 **0** 开始的整数数组 `nums` ，如果满足下述条件，则认为数组 `nums` 是一个 **美丽数组** ：

* `nums.length` 为偶数
* 对所有满足 `i % 2 == 0` 的下标 `i` ，`nums[i] != nums[i + 1]` 均成立

注意，空数组同样认为是美丽数组。

你可以从 `nums` 中删除任意数量的元素。当你删除一个元素时，被删除元素右侧的所有元素将会向左移动一个单位以填补空缺，而左侧的元素将会保持 **不变** 。

返回使 `nums` 变为美丽数组所需删除的 **最少** 元素数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,2,3,5]
<strong>输出:</strong> 1
<strong>解释:</strong> 可以删除 nums[0] 或 nums[1] ，这样得到的 nums = [1,2,3,5] 是一个美丽数组。可以证明，要想使 nums 变为美丽数组，至少需要删除 1 个元素。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,2,2,3,3]
<strong>输出:</strong> 2
<strong>解释:</strong> 可以删除 nums[0] 和 nums[5] ，这样得到的 nums = [1,2,2,3] 是一个美丽数组。可以证明，要想使 nums 变为美丽数组，至少需要删除 2 个元素。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
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
