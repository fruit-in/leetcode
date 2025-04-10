# 2448. Minimum Cost to Make Array Equal
You are given two **0-indexed** arrays `nums` and `cost` consisting each of `n` **positive** integers.

You can do the following operation **any** number of times:
* Increase or decrease **any** element of the array `nums` by `1`.

The cost of doing one operation on the <code>i<sup>th</sup></code> element is `cost[i]`.

Return *the **minimum** total cost such that all the elements of the array* `nums` *become **equal***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,5,2], cost = [2,3,1,14]
<strong>Output:</strong> 8
<strong>Explanation:</strong> We can make all the elements equal to 2 in the following way:
- Increase the 0th element one time. The cost is 2.
- Decrease the 1st element one time. The cost is 3.
- Decrease the 2nd element three times. The cost is 1 + 1 + 1 = 3.
The total cost is 2 + 3 + 3 = 8.
It can be shown that we cannot make the array equal with a smaller cost.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,2,2,2,2], cost = [4,2,8,1,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All the elements are already equal, so no operations are needed.
</pre>

#### Constraints:
* `n == nums.length == cost.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i], cost[i] <= 10<sup>6</sup></code>
* Test cases are generated in a way that the output doesn't exceed 2<sup>53</sup>-1

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut nums = (0..nums.len())
            .map(|i| (nums[i] as i64, cost[i] as i64))
            .collect::<Vec<_>>();
        let mut prefix_cost = 0;
        let mut suffix_cost = 0;
        let mut equal_cost = 0;
        let mut total_cost = 0;
        let mut i = 0;
        let mut ret = i64::MAX;

        nums.sort_unstable();

        for j in 0..nums.len() {
            suffix_cost += nums[j].1;
            total_cost += nums[j].1 * (nums[j].0 - nums[0].0 + 1);
        }

        ret = ret.min(total_cost);

        for x in nums[0].0..=nums[nums.len() - 1].0 {
            prefix_cost += equal_cost;
            equal_cost = 0;
            total_cost += prefix_cost - suffix_cost;
            ret = ret.min(total_cost);
            while i < nums.len() && nums[i].0 == x {
                suffix_cost -= nums[i].1;
                equal_cost += nums[i].1;
                i += 1;
            }
        }

        ret
    }
}
```
