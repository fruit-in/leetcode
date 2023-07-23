# 1031. 两个非重叠子数组的最大和
给你一个整数数组 `nums` 和两个整数 `firstLen` 和 `secondLen`，请你找出并返回两个非重叠 **子数组** 中元素的最大和，长度分别为 `firstLen` 和 `secondLen` 。

长度为 `firstLen` 的子数组可以出现在长为 `secondLen` 的子数组之前或之后，但二者必须是不重叠的。

子数组是数组的一个 **连续** 部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,6,5,2,2,5,1,9,4], firstLen = 1, secondLen = 2
<strong>输出:</strong> 20
<strong>解释:</strong> 子数组的一种选择中，[9] 长度为 1，[6,5] 长度为 2。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,8,1,3,2,1,8,9,0], firstLen = 3, secondLen = 2
<strong>输出:</strong> 29
<strong>解释:</strong> 子数组的一种选择中，[3,8,1] 长度为 3，[8,9] 长度为 2。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,1,5,6,0,9,5,0,3,8], firstLen = 4, secondLen = 3
<strong>输出:</strong> 31
<strong>解释:</strong> 子数组的一种选择中，[5,6,0,9] 长度为 4，[0,3,8] 长度为 3。
</pre>

#### 提示:
* `1 <= firstLen, secondLen <= 1000`
* `2 <= firstLen + secondLen <= 1000`
* `firstLen + secondLen <= nums.length <= 1000`
* `0 <= nums[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut ret = i32::MIN;

        for i in 1..prefix_sum.len() {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
        }

        for i in 0..nums.len() - first_len + 1 {
            let first_sum = prefix_sum[i + first_len] - prefix_sum[i];
            for j in 0..(i + 1).saturating_sub(second_len) {
                let second_sum = prefix_sum[j + second_len] - prefix_sum[j];
                ret = ret.max(first_sum + second_sum);
            }
            for j in i + first_len..nums.len() - second_len + 1 {
                let second_sum = prefix_sum[j + second_len] - prefix_sum[j];
                ret = ret.max(first_sum + second_sum);
            }
        }

        ret
    }
}
```
