# 1043. 分隔数组以得到最大和
给你一个整数数组 `arr`，请你将该数组分隔为长度 **最多** 为 k 的一些（连续）子数组。分隔完成后，每个子数组的中的所有值都会变为该子数组中的最大值。

返回将数组分隔变换后能够得到的元素最大和。本题所用到的测试用例会确保答案是一个 32 位整数。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,15,7,9,2,5,10], k = 3
<strong>输出:</strong> 84
<strong>解释:</strong> 数组变为 [15,15,15,9,10,10,10]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
<strong>输出:</strong> 83
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1], k = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= arr.length <= 500`
* <code>0 <= arr[i] <= 10<sup>9</sup></code>
* `1 <= k <= arr.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];

        for i in 0..dp.len() {
            let mut max_val = 0;

            for j in 0..(k as usize).min(dp.len() - i - 1) {
                max_val = max_val.max(arr[i + j]);
                dp[i + j + 1] = dp[i + j + 1].max(dp[i] + max_val * (j as i32 + 1));
            }
        }

        *dp.last().unwrap()
    }
}
```
