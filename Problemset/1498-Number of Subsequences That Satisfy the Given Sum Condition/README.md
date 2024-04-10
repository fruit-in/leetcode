# 1498. Number of Subsequences That Satisfy the Given Sum Condition
You are given an array of integers `nums` and an integer `target`.

Return *the number of **non-empty** subsequences of* `nums` *such that the sum of the minimum and maximum element on it is less or equal to* `target`. Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,5,6,7], target = 9
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 subsequences that satisfy the condition.
[3] -> Min value + max value <= target (3 + 3 <= 9)
[3,5] -> (3 + 5 <= 9)
[3,5,6] -> (3 + 6 <= 9)
[3,6] -> (3 + 6 <= 9)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,3,6,8], target = 10
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 6 subsequences that satisfy the condition. (nums can have repeated numbers).
[3] , [3] , [3,3], [3,6] , [3,6] , [3,3,6]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,3,3,4,6,7], target = 12
<strong>Output:</strong> 61
<strong>Explanation:</strong> There are 63 non-empty subsequences, two of them do not satisfy the condition ([6,7], [7]).
Number of valid subsequences (63 - 2 = 61).
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>
* <code>1 <= target <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut i = nums.len() - 1;
        let mut pow2 = vec![1];
        let mut ret = 0;

        nums.sort_unstable();

        for j in 0..nums.len() {
            if j > i || nums[j] * 2 > target {
                break;
            }

            while nums[i] + nums[j] > target {
                i -= 1;
            }

            while i - j >= pow2.len() {
                pow2.push(pow2.last().unwrap() * 2 % 1_000_000_007);
            }

            ret = (ret + pow2[i - j]) % 1_000_000_007;
        }

        ret
    }
}
```
