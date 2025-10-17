# 2644. Find the Maximum Divisibility Score
You are given two integer arrays `nums` and `divisors`.

The **divisibility score** of `divisors[i]` is the number of indices `j` such that `nums[j]` is divisible by `divisors[i]`.

Return the integer `divisors[i]` with the **maximum** divisibility score. If multiple integers have the maximum score, return the smallest one.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,9,15,50], divisors = [5,3,7,2]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The divisibility score of divisors[0] is 2 since nums[2] and nums[3] are divisible by 5.
The divisibility score of divisors[1] is 2 since nums[1] and nums[2] are divisible by 3.
The divisibility score of divisors[2] is 0 since none of the numbers in nums is divisible by 7.
The divisibility score of divisors[3] is 2 since nums[0] and nums[3] are divisible by 2.
As divisors[0], divisors[1], and divisors[3] have the same divisibility score, we return the smaller one which is divisors[3].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,7,9,3,9], divisors = [5,2,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The divisibility score of divisors[0] is 0 since none of numbers in nums is divisible by 5.
The divisibility score of divisors[1] is 1 since only nums[0] is divisible by 2.
The divisibility score of divisors[2] is 3 since nums[2], nums[3] and nums[4] are divisible by 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [20,14,21,10], divisors = [10,16,20]
<strong>Output:</strong> 10
<strong>Explanation:</strong>
The divisibility score of divisors[0] is 2 since nums[0] and nums[3] are divisible by 10.
The divisibility score of divisors[1] is 0 since none of the numbers in nums is divisible by 16.
The divisibility score of divisors[2] is 1 since nums[0] is divisible by 20.
</pre>

#### Constraints:
* `1 <= nums.length, divisors.length <= 1000`
* <code>1 <= nums[i], divisors[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max_score = 0;
        let mut ret = i32::MAX;

        for i in 0..divisors.len() {
            let mut score = 0;

            for j in 0..nums.len() {
                if nums[j] % divisors[i] == 0 {
                    score += 1;
                }
            }

            if score > max_score {
                max_score = score;
                ret = divisors[i];
            } else if score == max_score && divisors[i] < ret {
                ret = divisors[i];
            }
        }

        ret
    }
}
```
