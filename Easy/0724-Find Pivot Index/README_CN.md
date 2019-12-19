# 724. 寻找数组的中心索引
给定一个整数类型的数组 ```nums```，请编写一个能够返回数组**“中心索引”**的方法。

我们是这样定义数组**中心索引**的：数组中心索引的左侧所有元素相加的和等于右侧所有元素相加的和。

如果数组不存在中心索引，那么我们应该返回 -1。如果数组有多个中心索引，那么我们应该返回最靠近左边的那一个。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1, 7, 3, 6, 5, 6]
<strong>输出:</strong> 3
<strong>解释:</strong>
索引3 (nums[3] = 6) 的左侧数之和(1 + 7 + 3 = 11)，与右侧数之和(5 + 6 = 11)相等。
同时, 3 也是第一个符合要求的中心索引。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1, 2, 3]
<strong>输出:</strong> -1
<strong>解释:</strong>
数组中不存在满足此条件的中心索引。
</pre>

#### 说明:
* ```nums``` 的长度范围为 ```[0, 10000]```。
* 任何一个 ```nums[i]``` 将会是一个范围在 ```[-1000, 1000]```的整数。

## 题解 (Rust)

### 1. 线性扫描
```Rust
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for i in 0..nums.len() {
            if 2 * left_sum == total_sum - nums[i] {
                return i as i32;
            }
            left_sum += nums[i];
        }
        -1
    }
}
```
