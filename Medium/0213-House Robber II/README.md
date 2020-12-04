# 213. House Robber II
You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are **arranged in a circle**. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and **it will automatically contact the police if two adjacent houses were broken into on the same night**.

Given a list of non-negative integers `nums` representing the amount of money of each house, return *the maximum amount of money you can rob tonight **without alerting the police***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [0]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 1000`

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut rob_fst = (0, 0, nums[0]);
        let mut rob_lst = (0, 0, 0);

        for i in 1..nums.len() {
            if i != nums.len() - 1 {
                rob_fst = (rob_fst.1, rob_fst.2, nums[i] + rob_fst.0.max(rob_fst.1));
            }
            rob_lst = (rob_lst.1, rob_lst.2, nums[i] + rob_lst.0.max(rob_lst.1));
        }

        rob_fst.1.max(rob_fst.2).max(rob_lst.1).max(rob_lst.2)
    }
}
```
