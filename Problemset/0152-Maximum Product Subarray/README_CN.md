# 152. 乘积最大子数组
给你一个整数数组 `nums` ，请你找出数组中乘积最大的非空连续子数组（该子数组中至少包含一个数字），并返回该子数组所对应的乘积。

测试用例的答案是一个 **32-位** 整数。

**子数组** 是数组的连续子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,-2,4]
<strong>输出:</strong> 6
<strong>解释:</strong> 子数组 [2,3] 有最大乘积 6。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-2,0,-1]
<strong>输出:</strong> 0
<strong>解释:</strong> 结果不能为 2, 因为 [-2,-1] 不是子数组。
</pre>

#### 提示:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* `-10 <= nums[i] <= 10`
* `nums` 的任何前缀或后缀的乘积都 **保证** 是一个 **32-位** 整数

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ret = *nums.iter().max().unwrap();

        for slice in nums.split(|&num| num == 0) {
            if slice.iter().filter(|&&num| num < 0).count() % 2 == 1 {
                let i = slice.iter().position(|&num| num < 0).unwrap();
                let j = slice.iter().rposition(|&num| num < 0).unwrap();

                if i + 1 < slice.len() {
                    ret = ret.max(slice.iter().skip(i + 1).product());
                }
                if j > 0 {
                    ret = ret.max(slice.iter().take(j).product());
                }
            } else if !slice.is_empty() {
                ret = ret.max(slice.iter().product());
            }
        }

        ret
    }
}
```
