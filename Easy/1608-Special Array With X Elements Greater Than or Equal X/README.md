# 1608. Special Array With X Elements Greater Than or Equal X
You are given an array `nums` of non-negative integers. `nums` is considered **special** if there exists a number `x` such that there are **exactly** `x` numbers in `nums` that are **greater than or equal to** `x`.

Notice that `x` **does not** have to be an element in `nums`.

Return `x` *if the array is **special**, otherwise, return* `-1`. It can be proven that if `nums` is special, the value for `x` is **unique**.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 values (3 and 5) that are greater than or equal to 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,0]
<strong>Output:</strong> -1
<strong>Explanation:</strong> No numbers fit the criteria for x.
If x = 0, there should be 0 numbers >= x, but there are 2.
If x = 1, there should be 1 number >= x, but there are 0.
If x = 2, there should be 2 numbers >= x, but there are 0.
x cannot be greater since there are only 2 numbers in nums.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [0,4,3,0,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 values that are greater than or equal to 3.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [3,6,7,7,0]
<strong>Output:</strong> -1
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 1000`

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));

        for x in 1..=nums.len() {
            if nums[x - 1] >= x as i32 && (x == nums.len() || nums[x] < x as i32) {
                return x as i32;
            }
        }

        -1
    }
}
```
