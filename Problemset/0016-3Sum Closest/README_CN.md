# 16. 最接近的三数之和
给定一个包括 *n* 个整数的数组 `nums` 和 一个目标值 `target`。找出 `nums` 中的三个整数，使得它们的和与 `target` 最接近。返回这三个数的和。假定每组输入只存在唯一答案。

#### 示例:
<pre>
<strong>输入:</strong> nums = [-1,2,1,-4], target = 1
<strong>输出:</strong> 2
<strong>解释:</strong> 与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
</pre>

#### 提示:
* `3 <= nums.length <= 10^3`
* `-10^3 <= nums[i] <= 10^3`
* `-10^4 <= target <= 10^4`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut diff = std::i32::MAX;
        nums.sort_unstable();

        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];

                if (target - sum).abs() < diff.abs() {
                    diff = target - sum;
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        target - diff
    }
}
```
