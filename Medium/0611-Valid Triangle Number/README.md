# 611. Valid Triangle Number
Given an array consists of non-negative integers, your task is to count the number of triplets chosen from the array that can make triangles if we take them as side lengths of a triangle.

#### Example 1:
<pre>
<strong>Input:</strong> [2,2,3,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Valid combinations are:
2,3,4 (using the first 2)
2,3,4 (using the second 2)
2,2,3
</pre>

#### Note:
1. The length of the given array won't exceed 1000.
2. The integers in the given array are in the range of [0, 1000].

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        nums.sort_unstable();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                for j in (i + 1)..nums.len() {
                    match nums[(j + 1)..].binary_search(&(nums[i] + nums[j] - 1)) {
                        Ok(k) => ret += k + 1,
                        Err(k) => ret += k,
                    }
                }
            }
        }

        ret as i32
    }
}
```
