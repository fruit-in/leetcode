# 643. 子数组最大平均数 I
给定 ```n``` 个整数，找出平均数最大且长度为 ```k``` 的连续子数组，并输出该最大平均数。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,12,-5,-6,50,3], k = 4
<strong>输出:</strong> 12.75
<strong>解释:</strong> 最大平均数 (12-5-6+50)/4 = 51/4 = 12.75
</pre>

#### 注意:
1. 1 <= ```k``` <= ```n``` <= 30,000。
2. 所给数据范围 [-10,000，10,000]。

## 题解 (Rust)

### 1. 滑动窗口
```Rust
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum: i32 = nums.iter().take(k).sum();
        let mut max_sum = sum;

        for i in k..nums.len() {
            sum += nums[i] - nums[i - k];
            max_sum = max_sum.max(sum);
        }

        max_sum as f64 / k as f64
    }
}
```
