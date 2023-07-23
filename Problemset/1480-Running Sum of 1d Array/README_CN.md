# 1480. 一维数组的动态和
给你一个数组 `nums` 。数组「动态和」的计算公式为：`runningSum[i] = sum(nums[0]…nums[i])` 。

请返回 `nums` 的动态和。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> [1,3,6,10]
<strong>解释:</strong> 动态和计算过程为 [1, 1+2, 1+2+3, 1+2+3+4] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1]
<strong>输出:</strong> [1,2,3,4,5]
<strong>解释:</strong> 动态和计算过程为 [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,1,2,10,1]
<strong>输出:</strong> [3,4,6,16,17]
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `-10^6 <= nums[i] <= 10^6`

## 题解 (Ruby)

### 1. 前缀和
```Ruby
# @param {Integer[]} nums
# @return {Integer[]}
def running_sum(nums)
    for i in 1...nums.length
        nums[i] += nums[i - 1]
    end

    return nums
end
```

## 题解 (Rust)

### 1. 前缀和
```Rust
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut running_sums = nums;

        for i in 1..running_sums.len() {
            running_sums[i] += running_sums[i - 1];
        }

        running_sums
    }
}
```
