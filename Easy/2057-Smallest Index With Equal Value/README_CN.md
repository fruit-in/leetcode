# 2057. 值相等的最小索引
给你一个下标从 0 开始的整数数组 `nums` ，返回 `nums` 中满足 `i mod 10 == nums[i]` 的最小下标 `i` ；如果不存在这样的下标，返回 `-1` 。

`x mod y` 表示 `x` 除以 `y` 的 **余数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,2]
<strong>输出:</strong> 0
<strong>解释:</strong>
i=0: 0 mod 10 = 0 == nums[0].
i=1: 1 mod 10 = 1 == nums[1].
i=2: 2 mod 10 = 2 == nums[2].
所有下标都满足 i mod 10 == nums[i] ，所以返回最小下标 0
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,3,2,1]
<strong>输出:</strong> 2
<strong>解释:</strong>
i=0: 0 mod 10 = 0 != nums[0].
i=1: 1 mod 10 = 1 != nums[1].
i=2: 2 mod 10 = 2 == nums[2].
i=3: 3 mod 10 = 3 != nums[3].
2 唯一一个满足 i mod 10 == nums[i] 的下标
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5,6,7,8,9,0]
<strong>输出:</strong> -1
<strong>解释:</strong> 不存在满足 i mod 10 == nums[i] 的下标
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [2,1,3,5,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 1 是唯一一个满足 i mod 10 == nums[i] 的下标
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 9`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .find(|&i| i as i32 % 10 == nums[i])
            .map_or(-1, |x| x as i32)
    }
}
```
