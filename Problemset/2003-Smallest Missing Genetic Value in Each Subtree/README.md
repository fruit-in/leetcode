# 2003. Smallest Missing Genetic Value in Each Subtree
There is a **family tree** rooted at `0` consisting of `n` nodes numbered `0` to `n - 1`. You are given a **0-indexed** integer array `parents`, where `parents[i]` is the parent for node `i`. Since node `0` is the **root**, `parents[0] == -1`.

There are <code>10<sup>5</sup></code> genetic values, each represented by an integer in the **inclusive** range <code>[1, 10<sup>5</sup>]</code>. You are given a **0-indexed** integer array `nums`, where `nums[i]` is a **distinct** genetic value for node `i`.

Return *an array* `ans` *of length* `n` *where* `ans[i]` *is the **smallest** genetic value that is **missing** from the subtree rooted at node* `i`.

The **subtree** rooted at a node `x` contains node `x` and all of its **descendant** nodes.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/08/23/case-1.png)
<pre>
<strong>Input:</strong> parents = [-1,0,0,2], nums = [1,2,3,4]
<strong>Output:</strong> [5,1,1,1]
<strong>Explanation:</strong> The answer for each subtree is calculated as follows:
- 0: The subtree contains nodes [0,1,2,3] with values [1,2,3,4]. 5 is the smallest missing value.
- 1: The subtree contains only node 1 with value 2. 1 is the smallest missing value.
- 2: The subtree contains nodes [2,3] with values [3,4]. 1 is the smallest missing value.
- 3: The subtree contains only node 3 with value 4. 1 is the smallest missing value.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/08/23/case-2.png)
<pre>
<strong>Input:</strong> parents = [-1,0,1,0,3,3], nums = [5,4,6,2,1,3]
<strong>Output:</strong> [7,1,1,4,2,1]
<strong>Explanation:</strong> The answer for each subtree is calculated as follows:
- 0: The subtree contains nodes [0,1,2,3,4,5] with values [5,4,6,2,1,3]. 7 is the smallest missing value.
- 1: The subtree contains nodes [1,2] with values [4,6]. 1 is the smallest missing value.
- 2: The subtree contains only node 2 with value 6. 1 is the smallest missing value.
- 3: The subtree contains nodes [3,4,5] with values [2,1,3]. 4 is the smallest missing value.
- 4: The subtree contains only node 4 with value 1. 2 is the smallest missing value.
- 5: The subtree contains only node 5 with value 3. 1 is the smallest missing value.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> parents = [-1,2,3,0,2,4,1], nums = [2,3,4,5,6,7,8]
<strong>Output:</strong> [1,1,1,1,1,1,1]
<strong>Explanation:</strong> The value 1 is missing from all the subtrees.
</pre>

#### Constraints:
* `n == parents.length == nums.length`
* <code>2 <= n <= 105</sup></code>
* `0 <= parents[i] <= n - 1` for `i != 0`
* `parents[0] == -1`
* `parents` represents a valid tree.
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* Each `nums[i]` is distinct.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut children = vec![vec![]; n];
        let mut sub_genetic = HashSet::new();
        let mut min_val = 1;
        let mut ans = vec![1; n];

        for i in 1..n {
            children[parents[i] as usize].push(i);
        }

        for i in 0..n {
            if nums[i] > 1 {
                continue;
            }

            let mut j = i as i32;

            while j != -1 {
                let mut stack = vec![j as usize];

                while let Some(k) = stack.pop() {
                    if !sub_genetic.contains(&nums[k]) {
                        sub_genetic.insert(nums[k]);
                        for &l in &children[k] {
                            stack.push(l);
                        }
                    }
                }

                while sub_genetic.contains(&min_val) {
                    min_val += 1;
                }
                ans[j as usize] = min_val;

                j = parents[j as usize];
            }

            break;
        }

        ans
    }
}
```
