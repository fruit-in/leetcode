# 1330. Reverse Subarray To Maximize Array Value
You are given an integer array `nums`. The *value* of this array is defined as the sum of `|nums[i] - nums[i + 1]|` for all `0 <= i < nums.length - 1`.

You are allowed to select any subarray of the given array and reverse it. You can perform this operation **only once**.

Find maximum possible value of the final array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,1,5,4]
<strong>Output:</strong> 10
<strong>Explanation:</strong> By reversing the subarray [3,1,5] the array becomes [2,5,1,3,4] whose value is 10.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,4,9,24,2,1,10]
<strong>Output:</strong> 68
</pre>

#### Constraints:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let origin = (1..nums.len())
            .map(|i| (nums[i] - nums[i - 1]).abs())
            .sum::<i32>();
        let mut max_num = nums[0].min(nums[1]);
        let mut min_num = nums[0].max(nums[1]);
        let mut ret = origin;

        for i in 1..nums.len() - 1 {
            let a = nums[i - 1];
            let b = nums[i];
            let c = nums[i + 1];

            ret = ret.max(origin + (c - nums[0]).abs() - (c - b).abs());
            ret = ret.max(origin + (a - nums.last().unwrap()).abs() - (b - a).abs());

            let x = b.max(c);
            let y = b.min(c);

            if x < max_num {
                ret = ret.max(origin + 2 * max_num - 2 * x);
            }
            if y > min_num {
                ret = ret.max(origin + 2 * y - 2 * min_num);
            }

            max_num = max_num.max(y);
            min_num = min_num.min(x);
        }

        ret
    }
}
```
