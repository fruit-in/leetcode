# 2154. Keep Multiplying Found Values by Two
You are given an array of integers `nums`. You are also given an integer `original` which is the first number that needs to be searched for in `nums`.

You then do the following steps:
1. If `original` is found in `nums`, **multiply** it by two (i.e., set `original = 2 * original`).
2. Otherwise, **stop** the process.
3. **Repeat** this process with the new number as long as you keep finding the number.

Return *the **final** value of* `original`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [5,3,6,1,12], original = 3
<strong>Output:</strong> 24
<strong>Explanation:</strong>
- 3 is found in nums. 3 is multiplied by 2 to obtain 6.
- 6 is found in nums. 6 is multiplied by 2 to obtain 12.
- 12 is found in nums. 12 is multiplied by 2 to obtain 24.
- 24 is not found in nums. Thus, 24 is returned.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,7,9], original = 4
<strong>Output:</strong> 4
<strong>Explanation:</strong>
- 4 is not found in nums. Thus, 4 is returned.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `1 <= nums[i], original <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut original = original;
        let mut flag = 0;

        for num in nums {
            if num % original == 0 && (num / original).count_ones() == 1 {
                flag |= num / original;
            }
        }

        while flag & 1 == 1 {
            original *= 2;
            flag >>= 1;
        }

        original
    }
}
```
