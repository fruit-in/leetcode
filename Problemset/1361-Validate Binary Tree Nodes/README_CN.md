# 1361. 验证二叉树
二叉树上有 `n` 个节点，按从 `0` 到 `n - 1` 编号，其中节点 `i` 的两个子节点分别是 `leftChild[i]` 和 `rightChild[i]`。

只有 **所有** 节点能够形成且 **只** 形成 **一颗** 有效的二叉树时，返回 `true`；否则返回 `false`。

如果节点 `i` 没有左子节点，那么 `leftChild[i]` 就等于 `-1`。右子节点也符合该规则。

注意：节点没有值，本问题中仅仅使用节点编号。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/08/23/1503_ex1.png)
<pre>
<strong>输入:</strong> n = 4, leftChild = [1,-1,3,-1], rightChild = [2,-1,-1,-1]
<strong>输出:</strong> true
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2019/08/23/1503_ex2.png)
<pre>
<strong>输入:</strong> n = 4, leftChild = [1,-1,3,-1], rightChild = [2,3,-1,-1]
<strong>输出:</strong> false
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2019/08/23/1503_ex3.png)
<pre>
<strong>输入:</strong> n = 2, leftChild = [1,0], rightChild = [-1,-1]
<strong>输出:</strong> false
</pre>

#### 提示:
* `n == leftChild.length == rightChild.length`
* <code>1 <= n <= 10<sup>4</sup></code>
* `-1 <= leftChild[i], rightChild[i] <= n - 1`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut candidates = (0..n).collect::<HashSet<_>>();
        let mut stack = vec![];
        let mut visited = HashSet::new();

        for i in 0..left_child.len() {
            if left_child[i] >= 0 && !candidates.remove(&left_child[i]) {
                return false;
            }
            if right_child[i] >= 0 && !candidates.remove(&right_child[i]) {
                return false;
            }
        }

        if candidates.len() != 1 {
            return false;
        }

        stack.push(candidates.into_iter().next().unwrap() as usize);
        visited.insert(stack[0]);

        while let Some(i) = stack.pop() {
            if left_child[i] >= 0 {
                if visited.insert(left_child[i] as usize) {
                    stack.push(left_child[i] as usize);
                } else {
                    return false;
                }
            }
            if right_child[i] >= 0 {
                if visited.insert(right_child[i] as usize) {
                    stack.push(right_child[i] as usize);
                } else {
                    return false;
                }
            }
        }

        visited.len() == n as usize
    }
}
```
