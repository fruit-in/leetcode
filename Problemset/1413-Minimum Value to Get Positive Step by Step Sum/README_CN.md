# 1413. 逐步求和得到正数的最小值
给你一个整数数组 `nums` 。你可以选定任意的 **正数** startValue 作为初始值。

你需要从左到右遍历 `nums` 数组，并将 startValue 依次累加上 `nums` 数组中的值。

请你在确保累加和始终大于等于 1 的前提下，选出一个最小的 **正数** 作为 startValue 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [-3,2,-3,4,2]
<strong>输出:</strong> 5
<strong>解释:</strong> 如果你选择 startValue = 4，在第三次累加时，和小于 1 。
                <strong>累加求和
                startValue = 4 | startValue = 5 | nums</strong>
                  (4 <strong>-3</strong> ) = 1  | (5 <strong>-3</strong> ) = 2    |  -3
                  (1 <strong>+2</strong> ) = 3  | (2 <strong>+2</strong> ) = 4    |   2
                  (3 <strong>-3</strong> ) = 0  | (4 <strong>-3</strong> ) = 1    |  -3
                  (0 <strong>+4</strong> ) = 4  | (1 <strong>+4</strong> ) = 5    |   4
                  (4 <strong>+2</strong> ) = 6  | (5 <strong>+2</strong> ) = 7    |   2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 最小的 startValue 需要是正数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,-2,-3]
<strong>输出:</strong> 5
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `-100 <= nums[i] <= 100`

## 题解 (Rust)

### 1. 前缀和
```Rust
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        1 - nums.into_iter().min().unwrap().min(0)
    }
}
```
