# 162. 寻找峰值
峰值元素是指其值大于左右相邻值的元素。

给定一个输入数组 ```nums```，其中 ```nums[i] ≠ nums[i+1]```，找到峰值元素并返回其索引。

数组可能包含多个峰值，在这种情况下，返回任何一个峰值所在位置即可。

你可以假设 ```nums[-1] = nums[n] = -∞```。

#### 示例 1:
<pre>
<strong>输入:</strong> <strong>nums</strong> = [1,2,3,1]
<strong>输出:</strong> 2
<strong>解释:</strong> 3 是峰值元素，你的函数应该返回其索引 2。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> <strong>nums</strong> = [1,2,1,3,5,6,4]
<strong>输出:</strong> 1 or 5
<strong>解释:</strong> 你的函数可以返回索引 1，其峰值元素为 2；
     或者返回索引 5， 其峰值元素为 6。
</pre>

#### 说明:
你的解法应该是 *O*(*logN*) 时间复杂度的。

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let m = (l + r) / 2;
            if nums[m] < nums[m + 1] {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l as i32
    }
}
```
