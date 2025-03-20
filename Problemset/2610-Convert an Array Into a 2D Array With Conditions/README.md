# 2610. Convert an Array Into a 2D Array With Conditions
You are given an integer array `nums`. You need to create a 2D array from `nums` satisfying the following conditions:
* The 2D array should contain **only** the elements of the array `nums`.
* Each row in the 2D array contains **distinct** integers.
* The number of rows in the 2D array should be **minimal**.

Return *the resulting array*. If there are multiple answers, return any of them.

**Note** that the 2D array can have a different number of elements on each row.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,4,1,2,3,1]
<strong>Output:</strong> [[1,3,4,2],[1,3],[1]]
<strong>Explanation:</strong> We can create a 2D array that contains the following rows:
- 1,3,4,2
- 1,3
- 1
All elements of nums were used, and each row of the 2D array contains distinct integers, so it is a valid answer.
It can be shown that we cannot have less than 3 rows in a valid array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> [[4,3,2,1]]
<strong>Explanation:</strong> All elements of the array are distinct, so we can keep all of them in the first row of the 2D array.
</pre>

#### Constraints:
* `1 <= nums.length <= 200`
* `1 <= nums[i] <= nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut prev = 0;
        let mut count = 0;
        let mut ret = vec![];

        nums.sort_unstable();

        for &x in &nums {
            if x != prev {
                prev = x;
                count = 0;
            }
            count += 1;
            if count > ret.len() {
                ret.push(vec![]);
            }
            ret[count - 1].push(x);
        }

        ret
    }
}
```
