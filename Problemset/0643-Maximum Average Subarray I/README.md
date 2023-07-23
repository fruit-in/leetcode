# 643. Maximum Average Subarray I
Given an array consisting of ```n``` integers, find the contiguous subarray of given length ```k``` that has the maximum average value. And you need to output the maximum average value.

#### Example 1:
<pre>
<strong>Input:</strong> [1,12,-5,-6,50,3], k = 4
<strong>Output:</strong> 12.75
<strong>Explanation:</strong> Maximum average is (12-5-6+50)/4 = 51/4 = 12.75
</pre>

#### Note:
1. 1 <= ```k``` <= ```n``` <= 30,000.
2. Elements of the given array will be in the range [-10,000, 10,000].

## Solutions (Rust)

### 1. Sliding Window
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
