# 1712. Ways to Split Array Into Three Subarrays
A split of an integer array is **good** if:
* The array is split into three **non-empty** contiguous subarrays - named `left`, `mid`, `right` respectively from left to right.
* The sum of the elements in `left` is less than or equal to the sum of the elements in `mid`, and the sum of the elements in `mid` is less than or equal to the sum of the elements in `right`.

Given `nums`, an array of **non-negative** integers, return *the number of **good** ways to split* `nums`. As the number may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only good way to split nums is [1] [1] [1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,2,2,5,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three good ways of splitting nums:
[1] [2] [2,2,5,0]
[1] [2,2] [2,5,0]
[1,2] [2,2] [5,0]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,2,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no good way to split nums.
</pre>

#### Constraints:
* <code>3 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn ways_to_split(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        let sum = *nums.last().unwrap();

        for i in 0..nums.len() - 2 {
            let j = match nums[i + 1..].binary_search(&(2 * nums[i] - 1)) {
                Ok(a) => a + 1,
                Err(b) => b,
            };
            let k = match nums[i + 1..].binary_search(&((sum - nums[i]) / 2 + nums[i])) {
                Ok(a) if a == nums.len() - i - 2 => a,
                Ok(a) => a + 1,
                Err(b) if b == nums.len() - i - 1 => b - 1,
                Err(b) => b,
            };

            ret = (ret + k.saturating_sub(j) as i32) % 1_000_000_007;
        }

        ret
    }
}
```
