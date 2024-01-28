# 2509. 查询树中环的长度
给你一个整数 `n` ，表示你有一棵含有 <code>2<sup>n</sup> - 1</code> 个节点的 完全二叉树 。根节点的编号是 `1` ，树中编号在<code>[1, 2<sup>n - 1</sup> - 1]</code> 之间，编号为 `val` 的节点都有两个子节点，满足：

* 左子节点的编号为 `2 * val`
* 右子节点的编号为 `2 * val + 1`

给你一个长度为 `m` 的查询数组 `queries` ，它是一个二维整数数组，其中 <code>queries[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 。对于每个查询，求出以下问题的解：

1. 在节点编号为 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间添加一条边。
2. 求出图中环的长度。
3. 删除节点编号为 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间新添加的边。

**注意：**

* **环** 是开始和结束于同一节点的一条路径，路径中每条边都只会被访问一次。
* 环的长度是环中边的数目。
* 在树中添加额外的边后，两个点之间可能会有多条边。

请你返回一个长度为 `m` 的数组 `answer` ，其中 `answer[i]` 是第 `i` 个查询的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/10/25/bexample1.png)
<pre>
<strong>输入:</strong> n = 3, queries = [[5,3],[4,7],[2,3]]
<strong>输出:</strong> [4,5,3]
<strong>解释:</strong> 上图是一棵有 2<sup>3</sup> - 1 个节点的树。红色节点表示添加额外边后形成环的节点。
- 在节点 3 和节点 5 之间添加边后，环为 [5,2,1,3] ，所以第一个查询的结果是 4 。删掉添加的边后处理下一个查询。
- 在节点 4 和节点 7 之间添加边后，环为 [4,2,1,3,7] ，所以第二个查询的结果是 5 。删掉添加的边后处理下一个查询。
- 在节点 2 和节点 3 之间添加边后，环为 [2,1,3] ，所以第三个查询的结果是 3 。删掉添加的边。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/10/25/aexample2.png)
<pre>
<strong>输入:</strong> n = 2, queries = [[1,2]]
<strong>输出:</strong> [2]
<strong>解释:</strong> 上图是一棵有 2<sup>2</sup> - 1 个节点的树。红色节点表示添加额外边后形成环的节点。
- 在节点 1 和节点 2 之间添加边后，环为 [2,1] ，所以第一个查询的结果是 2 。删掉添加的边。
</pre>

#### 提示:
* `2 <= n <= 30`
* `m == queries.length`
* <code>1 <= m <= 10<sup>5</sup></code>
* `queries[i].length == 2`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub> <= 2<sup>n</sup> - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = vec![1; queries.len()];

        for i in 0..queries.len() {
            let mut a = queries[i][0];
            let mut b = queries[i][1];

            if a > b {
                std::mem::swap(&mut a, &mut b);
            }

            answer[i] += (a.leading_zeros() - b.leading_zeros()) as i32;
            b >>= a.leading_zeros() - b.leading_zeros();
            answer[i] += 2 * (32 - a.leading_zeros() as i32);

            for j in (0..(32 - a.leading_zeros())).rev() {
                if a & (1 << j) == b & (1 << j) {
                    answer[i] -= 2;
                } else {
                    break;
                }
            }
        }

        answer
    }
}
```
