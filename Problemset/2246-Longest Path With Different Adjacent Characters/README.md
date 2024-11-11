# 2246. Longest Path With Different Adjacent Characters
You are given a **tree** (i.e. a connected, undirected graph that has no cycles) **rooted** at node `0` consisting of `n` nodes numbered from `0` to `n - 1`. The tree is represented by a **0-indexed** array `parent` of size `n`, where `parent[i]` is the parent of node `i`. Since node `0` is the root, `parent[0] == -1`.

You are also given a string `s` of length `n`, where `s[i]` is the character assigned to node `i`.

Return *the length of the **longest path** in the tree such that no pair of **adjacent** nodes on the path have the same character assigned to them*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/25/testingdrawio.png)
<pre>
<strong>Input:</strong> parent = [-1,0,0,1,1,2], s = "abacbe"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest path where each two adjacent nodes have different characters in the tree is the path: 0 -> 1 -> 3. The length of this path is 3, so 3 is returned.
It can be proven that there is no longer path that satisfies the conditions.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/25/graph2drawio.png)
<pre>
<strong>Input:</strong> parent = [-1,0,0,0], s = "aabc"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest path where each two adjacent nodes have different characters is the path: 2 -> 0 -> 3. The length of this path is 3, so 3 is returned.
</pre>

#### Constraints:
* `n == parent.length == s.length`
* <code>1 <= n <= 105</sup></code>
* `0 <= parent[i] <= n - 1` for all `i >= 1`
* `parent[0] == -1`
* `parent` represents a valid tree.
* `s` consists of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut count = vec![0; n];
        let mut nodes = vec![];
        let mut max2 = vec![(0, 0); n];
        let mut ret = 1;

        for i in 1..n {
            count[parent[i] as usize] += 1;
        }
        for i in 0..n {
            if count[i] == 0 {
                nodes.push(i);
            }
        }

        while let Some(i) = nodes.pop() {
            ret = ret.max(max2[i].0 + max2[i].1 + 1);

            if i > 0 {
                if s[i] != s[parent[i] as usize] {
                    if max2[i].0 + 1 >= max2[parent[i] as usize].0 {
                        max2[parent[i] as usize] = (max2[i].0 + 1, max2[parent[i] as usize].0);
                    } else if max2[i].0 + 1 > max2[parent[i] as usize].1 {
                        max2[parent[i] as usize].1 = max2[i].0 + 1;
                    }
                }

                count[parent[i] as usize] -= 1;
                if count[parent[i] as usize] == 0 {
                    nodes.push(parent[i] as usize);
                }
            }
        }

        ret
    }
}
```
