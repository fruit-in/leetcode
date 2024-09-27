# 2049. Count Nodes With the Highest Score
There is a **binary** tree rooted at `0` consisting of `n` nodes. The nodes are labeled from `0` to `n - 1`. You are given a **0-indexed** integer array `parents` representing the tree, where `parents[i]` is the parent of node `i`. Since node `0` is the root, `parents[0] == -1`.

Each node has a **score**. To find the score of a node, consider if the node and the edges connected to it were **removed**. The tree would become one or more **non-empty** subtrees. The **size** of a subtree is the number of the nodes in it. The **score** of the node is the **product of the sizes** of all those subtrees.

Return *the **number** of nodes that have the **highest score***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/10/03/example-1.png)
<pre>
<strong>Input:</strong> parents = [-1,2,0,2,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- The score of node 0 is: 3 * 1 = 3
- The score of node 1 is: 4 = 4
- The score of node 2 is: 1 * 1 * 2 = 2
- The score of node 3 is: 4 = 4
- The score of node 4 is: 4 = 4
The highest score is 4, and three nodes (node 1, node 3, and node 4) have the highest score.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/10/03/example-2.png)
<pre>
<strong>Input:</strong> parents = [-1,2,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- The score of node 0 is: 2 = 2
- The score of node 1 is: 2 = 2
- The score of node 2 is: 1 * 1 = 1
The highest score is 2, and two nodes (node 0 and node 1) have the highest score.
</pre>

#### Constraints:
* `n == parents.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `parents[0] == -1`
* `0 <= parents[i] <= n - 1` for `i != 0`
* `parents` represents a valid binary tree.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let mut children = vec![vec![]; parents.len()];
        let mut children_count = vec![0; parents.len()];
        let mut nodes = vec![];
        let mut size = vec![1_i64; parents.len()];
        let mut score_count = HashMap::new();

        for i in 1..parents.len() {
            children[parents[i] as usize].push(i);
            children_count[parents[i] as usize] += 1;
        }

        for i in 0..parents.len() {
            if children_count[i] == 0 {
                nodes.push(i);
            }
        }

        while let Some(i) = nodes.pop() {
            size[parents[i] as usize] += size[i];
            children_count[parents[i] as usize] -= 1;
            if parents[i] > 0 && children_count[parents[i] as usize] == 0 {
                nodes.push(parents[i] as usize);
            }
        }

        for i in 0..parents.len() {
            let mut score = 1;

            if i > 0 {
                score *= size[0] - size[i];
            }
            if let Some(&j) = children[i].get(0) {
                score *= size[j];
            }
            if let Some(&j) = children[i].get(1) {
                score *= size[j];
            }

            *score_count.entry(score).or_insert(0) += 1;
        }

        *score_count.iter().max().unwrap().1
    }
}
```
