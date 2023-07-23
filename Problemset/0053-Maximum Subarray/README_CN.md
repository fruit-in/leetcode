# 53. 最大子序和
给定一个整数数组 ```nums``` ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

#### 示例:
<pre>
<strong>输入:</strong> [-2,1,-3,4,-1,2,1,-5,4],
<strong>输出:</strong> 6
<strong>解释:</strong> 连续子数组 [4,-1,2,1] 的和最大，为 6。
</pre>

#### 进阶:
如果你已经实现复杂度为 O(*n*) 的解法，尝试使用更为精妙的分治法求解。

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum > max {
                    max = sum;
                }
            }
        }
        max
    }
}
```

### 2. 单次遍历
```Rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = std::i32::MIN;
        let mut min_sum = 0;

        for n in nums {
            min_sum = min_sum.min(sum);
            sum += n;
            max_sum = max_sum.max(sum - min_sum);
        }
        max_sum
    }
}
```
