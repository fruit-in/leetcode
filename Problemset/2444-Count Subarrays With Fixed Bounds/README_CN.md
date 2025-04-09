# 2444. 统计定界子数组的数目
给你一个整数数组 `nums` 和两个整数 `minK` 以及 `maxK` 。

`nums` 的定界子数组是满足下述条件的一个子数组：
* 子数组中的 **最小值** 等于 `minK` 。
* 子数组中的 **最大值** 等于 `maxK` 。

返回定界子数组的数目。

子数组是数组中的一个连续部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,5,2,7,5], minK = 1, maxK = 5
<strong>输出:</strong> 2
<strong>解释:</strong> 定界子数组是 [1,3,5] 和 [1,3,5,2] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1,1], minK = 1, maxK = 1
<strong>输出:</strong> 10
<strong>解释:</strong> nums 的每个子数组都是一个定界子数组。共有 10 个子数组。
</pre>

#### 提示:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], minK, maxK <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut last_ban = -1;
        let mut last_min = -1;
        let mut last_max = -1;
        let mut ret = 0;

        for i in 0..nums.len() {
            if nums[i] < min_k || nums[i] > max_k {
                last_ban = i as i64;
            }
            if nums[i] == min_k {
                last_min = i as i64;
            }
            if nums[i] == max_k {
                last_max = i as i64;
            }

            if last_min > last_ban && last_max > last_ban {
                ret += last_min.min(last_max) - last_ban;
            }
        }

        ret
    }
}
```
