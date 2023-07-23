# 1262. Greatest Sum Divisible by Three
Given an array ```nums``` of integers, we need to find the maximum possible sum of elements of the array such that it is divisible by three.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,6,5,1,8]
<strong>Output:</strong> 18
<strong>Explanation:</strong> Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Since 4 is not divisible by 3, do not pick any number.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,4]
<strong>Output:</strong> 12
<strong>Explanation:</strong> Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3).
</pre>

#### Constraints:
* ```1 <= nums.length <= 4 * 10^4```
* ```1 <= nums[i] <= 10^4```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut min = vec![(19996, 10000), (20000, 10001)];
        let mut ret = 0;

        for num in nums {
            ret += num;
            match num as usize % 3 {
                0 => (),
                i => {
                    if num < min[i - 1].0 {
                        min[i - 1] = (num, min[i - 1].0);
                    } else if num < min[i - 1].1 {
                        min[i - 1].1 = num;
                    }
                },
            }
        }

        match ret % 3 {
            1 => ret - min[0].0.min(min[1].0 + min[1].1),
            2 => ret - min[1].0.min(min[0].0 + min[0].1),
            _ => ret,
        }
    }
}
```
