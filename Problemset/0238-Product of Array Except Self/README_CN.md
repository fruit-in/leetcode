# 238. 除自身以外数组的乘积
给你一个长度为 *n* 的整数数组 `nums`，其中 *n* > 1，返回输出数组 `output` ，其中 `output[i]` 等于 `nums` 中除 `nums[i]` 之外其余各元素的乘积。

#### 示例:
<pre>
<b>输入:</b> [1,2,3,4]
<b>输出:</b> [24,12,8,6]
</pre>

**提示:** 题目数据保证数组之中任意元素的全部前缀元素和后缀（甚至是整个数组）的乘积都在 32 位整数范围内。

**说明:** 请**不要使用除法**，且在 O(*n*) 时间复杂度内完成此题。

#### 进阶:
你可以在常数空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组**不被视为**额外空间。）

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product_l = 1;
        let mut product_r = 1;
        let mut ret = vec![1; nums.len()];

        for i in 0..nums.len() {
            let j = nums.len() - 1 - i;

            ret[i] *= product_l;
            ret[j] *= product_r;

            product_l *= nums[i];
            product_r *= nums[j];
        }

        ret
    }
}
```
