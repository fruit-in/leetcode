# 2012. 数组美丽值求和
给你一个下标从 **0** 开始的整数数组 `nums` 。对于每个下标 `i`（`1 <= i <= nums.length - 2`），`nums[i]` 的 **美丽值** 等于：

* `2`，对于所有 `0 <= j < i` 且 `i < k <= nums.length - 1` ，满足 `nums[j] < nums[i] < nums[k]`
* `1`，如果满足 `nums[i - 1] < nums[i] < nums[i + 1]` ，且不满足前面的条件
* `0`，如果上述条件全部不满足

返回符合 `1 <= i <= nums.length - 2` 的所有 `nums[i]` 的 **美丽值的总和** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong> 2
<strong>解释:</strong> 对于每个符合范围 1 <= i <= 1 的下标 i :
- nums[1] 的美丽值等于 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,4,6,4]
<strong>输出:</strong> 1
<strong>解释:</strong> 对于每个符合范围 1 <= i <= 2 的下标 i :
- nums[1] 的美丽值等于 1
- nums[2] 的美丽值等于 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,2,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 对于每个符合范围 1 <= i <= 1 的下标 i :
- nums[1] 的美丽值等于 0
</pre>

#### 提示:
* <code>3 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = vec![i32::MIN; nums.len()];
        let mut min = vec![i32::MAX; nums.len()];
        let mut ret = 0;

        for i in 1..nums.len() {
            max[i] = max[i - 1].max(nums[i - 1]);
            min[n - 1 - i] = min[n - i].min(nums[n - i]);
        }

        for i in 1..nums.len() - 1 {
            if max[i] < nums[i] && nums[i] < min[i] {
                ret += 2;
            } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
                ret += 1;
            }
        }

        ret
    }
}
```
