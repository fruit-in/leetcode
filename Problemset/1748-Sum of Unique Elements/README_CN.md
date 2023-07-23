# 1748. 唯一元素的和
给你一个整数数组 `nums` 。数组中唯一元素是那些只出现 **恰好一次** 的元素。

请你返回 `nums` 中唯一元素的 **和** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,2]
<strong>输出:</strong> 4
<strong>解释:</strong> 唯一元素为 [1,3] ，和为 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 没有唯一元素，和为 0 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5]
<strong>输出:</strong> 15
<strong>解释:</strong> 唯一元素为 [1,2,3,4,5] ，和为 15 。
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut count = [0; 101];

        for num in nums {
            count[num as usize] += 1;
        }

        count
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 1)
            .map(|(n, _)| n as i32)
            .sum()
    }
}
```
