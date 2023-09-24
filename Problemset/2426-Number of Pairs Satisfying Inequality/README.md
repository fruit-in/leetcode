# 2426. Number of Pairs Satisfying Inequality
You are given two **0-indexed** integer arrays `nums1` and `nums2`, each of size `n`, and an integer `diff`. Find the number of **pairs** `(i, j)` such that:

* `0 <= i < j <= n - 1` **and**
* `nums1[i] - nums1[j] <= nums2[i] - nums2[j] + diff`.

Return *the **number of pairs** that satisfy the conditions*.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [3,2,5], nums2 = [2,2,1], diff = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong>
There are 3 pairs that satisfy the conditions:
1. i = 0, j = 1: 3 - 2 <= 2 - 2 + 1. Since i < j and 1 <= 1, this pair satisfies the conditions.
2. i = 0, j = 2: 3 - 5 <= 2 - 1 + 1. Since i < j and -2 <= 2, this pair satisfies the conditions.
3. i = 1, j = 2: 2 - 5 <= 2 - 1 + 1. Since i < j and -3 <= 2, this pair satisfies the conditions.
Therefore, we return 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [3,-1], nums2 = [-2,2], diff = -1
<strong>Output:</strong> 0
<strong>Explanation:</strong>
Since there does not exist any pair that satisfies the conditions, we return 0.
</pre>

#### Constraints:
* `n == nums1.length == nums2.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums1[i], nums2[i] <= 10<sup>4</sup></code>
* <code>-10<sup>4</sup> <= diff <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let n = nums1.len();
        let nums = (0..n).map(|i| nums1[i] - nums2[i]).collect::<Vec<_>>();
        let mut nums_diff = nums.clone();
        let mut hashmap = HashMap::new();
        let mut tree = vec![0; n * 2 + 1];
        let mut ret = 0;

        for i in 0..n {
            nums_diff.push(nums[i] + diff);
        }
        nums_diff.sort_unstable();
        for i in 1..tree.len() {
            hashmap.insert(nums_diff[i - 1], i as i32);
        }

        for i in 0..n {
            let mut temp = hashmap[&(nums[i] + diff)];

            while temp > 0 {
                ret += tree[temp as usize];
                temp -= temp & (-temp);
            }

            temp = hashmap[&nums[i]];

            while temp < tree.len() as i32 {
                tree[temp as usize] += 1;
                temp += temp & (-temp);
            }
        }

        ret
    }
}
```
