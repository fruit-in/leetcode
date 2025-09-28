# 2097. Valid Arrangement of Pairs
You are given a **0-indexed** 2D integer array `pairs` where <code>pairs[i] = [start<sub>i</sub>, end<sub>i</sub>]</code>. An arrangement of `pairs` is **valid** if for every index `i` where `1 <= i < pairs.length`, we have <code>end<sub>i-1</sub> == start<sub>i</sub></code>.

Return ***any** valid arrangement of* `pairs`.

**Note:** The inputs will be generated such that there exists a valid arrangement of `pairs`.

#### Example 1:
<pre>
<strong>Input:</strong> pairs = [[5,1],[4,5],[11,9],[9,4]]
<strong>Output:</strong> [[11,9],[9,4],[4,5],[5,1]]
<strong>Explanation:</strong>
This is a valid arrangement since endi-1 always equals starti.
end0 = 9 == 9 = start1
end1 = 4 == 4 = start2
end2 = 5 == 5 = start3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> pairs = [[1,3],[3,2],[2,1]]
<strong>Output:</strong> [[1,3],[3,2],[2,1]]
<strong>Explanation:</strong>
This is a valid arrangement since endi-1 always equals starti.
end0 = 3 == 3 = start1
end1 = 2 == 2 = start2
The arrangements [[2,1],[1,3],[3,2]] and [[3,2],[2,1],[1,3]] are also valid.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> pairs = [[1,2],[1,3],[2,1]]
<strong>Output:</strong> [[1,2],[2,1],[1,3]]
<strong>Explanation:</strong>
This is a valid arrangement since endi-1 always equals starti.
end0 = 2 == 2 = start1
end1 = 1 == 1 = start2
</pre>

#### Constraints:
* <code>1 <= pairs.length <= 10<sup>5</sup></code>
* `pairs[i].length == 2`
* <code>0 <= start<sub>i</sub>, end<sub>i</sub> <= 10<sup>9</sup></code>
* <code>start<sub>i</sub> != end<sub>i</sub></code>
* No two pairs are exactly the same.
* There **exists** a valid arrangement of `pairs`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut outdegree = HashMap::new();
        let mut indegree = HashMap::new();
        let mut unused = HashMap::new();
        let mut stack = vec![];
        let mut curr = pairs[0][0];
        let mut path = vec![];
        let mut ret = vec![];

        for pair in &pairs {
            *outdegree.entry(pair[0]).or_insert(0) += 1;
            *indegree.entry(pair[1]).or_insert(0) += 1;
            unused.entry(pair[0]).or_insert(vec![]).push(pair[1]);
        }

        for pair in &pairs {
            if outdegree[&pair[0]] == *indegree.get(&pair[0]).unwrap_or(&0) + 1 {
                curr = pair[0];
                break;
            }
        }

        while curr != -1 {
            if !unused.get(&curr).unwrap_or(&vec![]).is_empty() {
                stack.push(curr);
                curr = unused.get_mut(&curr).unwrap().pop().unwrap();
            } else {
                path.push(curr);
                curr = stack.pop().unwrap_or(-1);
            }
        }

        for i in (1..path.len()).rev() {
            ret.push(vec![path[i], path[i - 1]]);
        }

        ret
    }
}
```
