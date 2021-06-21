# 1743. Restore the Array From Adjacent Pairs
There is an integer array `nums` that consists of `n` **unique** elements, but you have forgotten it. However, you do remember every pair of adjacent elements in `nums`.

You are given a 2D integer array `adjacentPairs` of size `n - 1` where each <code>adjacentPairs[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that the elements <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> are adjacent in `nums`.

It is guaranteed that every adjacent pair of elements `nums[i]` and `nums[i+1]` will exist in `adjacentPairs`, either as `[nums[i], nums[i+1]]` or `[nums[i+1], nums[i]]`. The pairs can appear **in any order**.

Return *the original array* `nums`. *If there are multiple solutions, return* ***any of them***.

#### Example 1:
<pre>
<strong>Input:</strong> adjacentPairs = [[2,1],[3,4],[3,2]]
<strong>Output:</strong> [1,2,3,4]
<strong>Explanation:</strong> This array has all its adjacent pairs in adjacentPairs.
Notice that adjacentPairs[i] may not be in left-to-right order.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> adjacentPairs = [[4,-2],[1,4],[-3,1]]
<strong>Output:</strong> [-2,4,1,-3]
<strong>Explanation:</strong> There can be negative numbers.
Another solution is [-3,1,4,-2], which would also be accepted.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> adjacentPairs = [[100000,-100000]]
<strong>Output:</strong> [100000,-100000]
</pre>

#### Constraints:
* `nums.length == n`
* `adjacentPairs.length == n - 1`
* `adjacentPairs[i].length == 2`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= nums[i], u<sub>i</sub>, v<sub>i</sub> <= 10<sup>5</sup></code>
* There exists some `nums` that has `adjacentPairs` as its pairs.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adjacent = HashMap::new();
        let mut ret = vec![];

        for pair in &adjacent_pairs {
            adjacent.entry(pair[0]).or_insert(vec![]).push(pair[1]);
            adjacent.entry(pair[1]).or_insert(vec![]).push(pair[0]);
        }

        let mut curr = *adjacent.iter().find(|(_, v)| v.len() == 1).unwrap().0;
        ret.push(curr);

        for i in 0..adjacent_pairs.len() {
            curr = match &adjacent.get(&curr).unwrap()[..] {
                &[x] => x,
                &[x, y] if x != ret[i - 1] => x,
                &[x, y] => y,
                _ => 0,
            };
            ret.push(curr);
        }

        ret
    }
}
```
