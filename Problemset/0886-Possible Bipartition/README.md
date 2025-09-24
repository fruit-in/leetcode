# 886. Possible Bipartition
We want to split a group of `n` people (labeled from `1` to `n`) into two groups of **any size**. Each person may dislike some other people, and they should not go into the same group.

Given the integer `n` and the array `dislikes` where <code>dislikes[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that the person labeled <code>a<sub>i</sub></code> does not like the person labeled <code>b<sub>i</sub></code>, return `true` *if it is possible to split everyone into two groups in this way*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4, dislikes = [[1,2],[1,3],[2,4]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The first group has [1,4], and the second group has [2,3].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, dislikes = [[1,2],[1,3],[2,3]]
<strong>Output:</strong> false
<strong>Explanation:</strong> We need at least 3 groups to divide them. We cannot put them in two groups.
</pre>

#### Constraints:
* `1 <= n <= 2000`
* <code>0 <= dislikes.length <= 10<sup>4</sup></code>
* `dislikes[i].length == 2`
* <code>1 <= a<sub>i</sub> < b<sub>i</sub> <= n</code>
* All the pairs of `dislikes` are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n + 1];
        let mut group = vec![0; n + 1];

        for edge in &dislikes {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        for i in 1..=n {
            if group[i] > 0 {
                continue;
            }

            group[i] = 1;
            let mut stack = vec![i];

            while let Some(a) = stack.pop() {
                for &b in &neighbors[a] {
                    if group[b] == 0 {
                        group[b] = 3 - group[a];
                        stack.push(b);
                    } else if group[b] == group[a] {
                        return false;
                    }
                }
            }
        }

        true
    }
}
```
