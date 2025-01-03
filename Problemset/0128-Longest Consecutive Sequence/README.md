# 128. Longest Consecutive Sequence
Given an unsorted array of integers `nums`, return *the length of the longest consecutive elements sequence*.

You must write an algorithm that runs in `O(n)` time.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [100,4,200,1,3,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,3,7,2,5,8,4,6,0,1]
<strong>Output:</strong> 9
</pre>

#### Constraints:
* <code>0 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut leftmost = HashMap::new();
        let mut rightmost = HashMap::new();
        let mut ret = 0;

        for &&x in nums.iter().collect::<HashSet<_>>().iter() {
            let lo = leftmost.remove(&(x - 1)).unwrap_or(x);
            let hi = rightmost.remove(&(x + 1)).unwrap_or(x);

            leftmost.insert(hi, lo);
            rightmost.insert(lo, hi);

            ret = ret.max(hi - lo + 1);
        }

        ret
    }
}
```
