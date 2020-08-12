# 1480. Running Sum of 1d Array
Given an array `nums`. We define a running sum of an array as `runningSum[i] = sum(nums[0]…nums[i])`.

Return the running sum of `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> [1,3,6,10]
<strong>Explanation:</strong> Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1]
<strong>Output:</strong> [1,2,3,4,5]
<strong>Explanation:</strong> Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,1,2,10,1]
<strong>Output:</strong> [3,4,6,16,17]
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `-10^6 <= nums[i] <= 10^6`

## Solutions (Ruby)

### 1. Prefix Sum
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

## Solutions (Rust)

### 1. Prefix Sum
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
