# 1313. Decompress Run-Length Encoded List
We are given a list ```nums``` of integers representing a list compressed with run-length encoding.

Consider each adjacent pair of elements ```[a, b] = [nums[2*i], nums[2*i+1]]``` (with ```i >= 0```).  For each such pair, there are ```a``` elements with value ```b``` in the decompressed list.

Return the decompressed list.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> [2,4,4,4]
<strong>Explanation:</strong> The first pair [1,2] means we have freq = 1 and val = 2 so we generate the array [2].
The second pair [3,4] means we have freq = 3 and val = 4 so we generate [4,4,4].
At the end the concatenation [2] + [4,4,4,4] is [2,4,4,4].
</pre>

#### Constraints:
* ```2 <= nums.length <= 100```
* ```nums.length % 2 == 0```
* ```1 <= nums[i] <= 100```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        for i in (0..nums.len()).step_by(2) {
            ret.append(&mut vec![nums[i + 1]; nums[i] as usize]);
        }

        ret
    }
}
```
