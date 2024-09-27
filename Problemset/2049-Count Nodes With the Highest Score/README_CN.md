# 2049. 统计最高分的节点数目
给你一棵根节点为 `0` 的 **二叉树** ，它总共有 `n` 个节点，节点编号为 `0` 到 `n - 1` 。同时给你一个下标从 **0** 开始的整数数组 `parents` 表示这棵树，其中 `parents[i]` 是节点 `i` 的父节点。由于节点 `0` 是根，所以 `parents[0] == -1` 。

一个子树的 **大小** 为这个子树内节点的数目。每个节点都有一个与之关联的 **分数** 。求出某个节点分数的方法是，将这个节点和与它相连的边全部 **删除** ，剩余部分是若干个 **非空** 子树，这个节点的 **分数** 为所有这些子树 **大小的乘积** 。

请你返回有 **最高得分** 节点的 **数目** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/03/example-1.png)
<pre>
<strong>输入:</strong> parents = [-1,2,0,2,0]
<strong>输出:</strong> 3
<strong>解释:</strong>
- 节点 0 的分数为：3 * 1 = 3
- 节点 1 的分数为：4 = 4
- 节点 2 的分数为：1 * 1 * 2 = 2
- 节点 3 的分数为：4 = 4
- 节点 4 的分数为：4 = 4
最高得分为 4 ，有三个节点得分为 4 （分别是节点 1，3 和 4 ）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/03/example-2.png)
<pre>
<strong>输入:</strong> parents = [-1,2,0]
<strong>输出:</strong> 2
<strong>解释:</strong>
- 节点 0 的分数为：2 = 2
- 节点 1 的分数为：2 = 2
- 节点 2 的分数为：1 * 1 = 1
最高分数为 2 ，有两个节点分数为 2 （分别为节点 0 和 1 ）。
</pre>

#### 提示:
* `n == parents.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `parents[0] == -1`
* 对于 `i != 0` ，有 `0 <= parents[i] <= n - 1`
* `parents` 表示一棵二叉树。

## 题解 (Rust)

### 1. 题解
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
