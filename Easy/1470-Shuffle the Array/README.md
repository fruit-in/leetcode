# 1470. Shuffle the Array
Given the array `nums` consisting of `2n` elements in the form <code>[x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>,y<sub>1</sub>,y<sub>2</sub>,...,y<sub>n</sub>]</code>.

*Return the array in the form* <code>[x<sub>1</sub>,y<sub>1</sub>,x<sub>2</sub>,y<sub>2</sub>,...,x<sub>n</sub>,y<sub>n</sub>]</code>.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,5,1,3,4,7], n = 3
<strong>Output:</strong> [2,3,5,4,1,7]
<strong>Explanation:</strong> Since x<sub>1</sub>=2, x<sub>2</sub>=5, x<sub>3</sub>=1, y<sub>1</sub>=3, y<sub>2</sub>=4, y<sub>3</sub>=7 then the answer is [2,3,5,4,1,7].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,4,3,2,1], n = 4
<strong>Output:</strong> [1,4,2,3,3,2,4,1]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,2,2], n = 2
<strong>Output:</strong> [1,2,1,2]
</pre>

#### Constraints:
* `1 <= n <= 500`
* `nums.length == 2n`
* `1 <= nums[i] <= 10^3`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ret = Vec::with_capacity(2 * n as usize);

        for i in 0..(n as usize) {
            ret.push(nums[i]);
            ret.push(nums[n as usize + i]);
        }

        ret
    }
}
```
