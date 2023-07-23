# 209. 长度最小的子数组
给定一个含有 **n** 个正整数的数组和一个正整数 **s** ，找出该数组中满足其和 **≥ s** 的长度最小的 **连续** 子数组，并返回其长度。如果不存在符合条件的子数组，返回 0。

#### 示例:
<pre>
<strong>输入:</strong> s = 7, nums = [2,3,1,2,4,3]
<strong>输出:</strong> 2
<strong>解释:</strong> 子数组 [4,3] 是该条件下的长度最小的子数组。
</pre>

#### 进阶:
* 如果你已经完成了 *O*(*n*) 时间复杂度的解法, 请尝试 *O*(*n* log *n*) 时间复杂度的解法。

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() < s {
            return 0;
        }

        let mut i = 0;
        let mut sum = 0;
        let mut ret = std::i32::MAX;

        for j in 0..nums.len() {
            sum += nums[j];
            while sum >= s {
                ret = ret.min((j - i) as i32 + 1);
                sum -= nums[i];
                i += 1;
            }
        }

        ret
    }
}
```
