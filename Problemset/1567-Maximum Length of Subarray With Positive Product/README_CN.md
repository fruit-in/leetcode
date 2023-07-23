# 1567. 乘积为正数的最长子数组长度
给你一个整数数组 `nums` ，请你求出乘积为正数的最长子数组的长度。

一个数组的子数组是由原数组中零个或者更多个连续数字组成的数组。

请你返回乘积为正数的最长子数组长度。

#### 示例 1:
<pre>
<b>输入:</b> nums = [1,-2,-3,4]
<b>输出:</b> 4
<b>解释:</b> 数组本身乘积就是正数，值为 24 。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> nums = [0,1,-2,-3,-4]
<b>输出:</b> 3
<b>解释:</b> 最长乘积为正数的子数组为 [1,-2,-3] ，乘积为 6 。
注意，我们不能把 0 也包括到子数组中，因为这样乘积为 0 ，不是正数。
</pre>

#### 示例 3:
<pre>
<b>输入:</b> nums = [-1,-2,-3,0,1]
<b>输出:</b> 2
<b>解释:</b> 乘积为正数的最长子数组是 [-1,-2] 或者 [-2,-3] 。
</pre>

#### 示例 4:
<pre>
<b>输入:</b> nums = [-1,2]
<b>输出:</b> 1
</pre>

#### 示例 5:
<pre>
<b>输入:</b> nums = [1,2,3,5,-6,4,0,10]
<b>输出:</b> 4
</pre>

#### 提示:
* `1 <= nums.length <= 10^5`
* `-10^9 <= nums[i] <= 10^9`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for v in nums.split(|&num| num == 0) {
            let mut tmp = v.len();

            if v.iter().filter(|&&x| x < 0).count() % 2 == 1 {
                let l_ne = v.iter().position(|&x| x < 0);
                let r_ne = v.iter().rev().position(|&x| x < 0);

                tmp -= l_ne.min(r_ne).unwrap() + 1;
            }

            ret = ret.max(tmp);
        }

        ret as i32
    }
}
```
