# 494. Target Sum
You are given an integer array `nums` and an integer `target`.

You want to build an **expression** out of nums by adding one of the symbols `'+'` and `'-'` before each integer in nums and then concatenate all the integers.
* For example, if `nums = [2, 1]`, you can add a `'+'` before `2` and a `'-'` before `1` and concatenate them to build the expression `"+2-1"`.

Return the number of different **expressions** that you can build, which evaluates to `target`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1], target = 3
<strong>Output:</strong> 5
<strong>Explanation:</strong> There are 5 ways to assign symbols to make the sum of nums be target 3.
-1 + 1 + 1 + 1 + 1 = 3
+1 - 1 + 1 + 1 + 1 = 3
+1 + 1 - 1 + 1 + 1 = 3
+1 + 1 + 1 - 1 + 1 = 3
+1 + 1 + 1 + 1 - 1 = 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1], target = 1
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= nums.length <= 20`
* `0 <= nums[i] <= 1000`
* `0 <= sum(nums[i]) <= 1000`
* `-1000 <= target <= 1000`

## Solutions (Ruby)

### 1. Dynamic Programming
```Ruby
# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer}
def find_target_sum_ways(nums, target)
  sum = nums.sum
  return 0 if sum < target.abs

  dp0 = [0] * (2 * sum + 1)
  dp0[sum] = 1

  nums.each do |num|
    dp1 = [0] * (2 * sum + 1)

    (0..2 * sum).each do |i|
      if dp0[i] > 0
        dp1[i + num] += dp0[i]
        dp1[i - num] += dp0[i]
      end
    end

    dp0 = dp1
  end

  dp0[sum + target]
end
```

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum < target.abs() as usize {
            return 0;
        }

        let mut dp0 = vec![0; 2 * sum + 1];
        dp0[sum] = 1;

        for num in nums {
            let mut dp1 = vec![0; 2 * sum + 1];

            for i in 0..2 * sum + 1 {
                if dp0[i] > 0 {
                    dp1[i + num as usize] += dp0[i];
                    dp1[i - num as usize] += dp0[i];
                }
            }

            dp0 = dp1;
        }

        dp0[(sum as i32 + target) as usize]
    }
}
```
