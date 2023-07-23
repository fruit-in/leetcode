# 1791. 找出星型图的中心节点
有一个无向的 **星型** 图，由 `n` 个编号从 `1` 到 `n` 的节点组成。星型图有一个 **中心** 节点，并且恰有 `n - 1` 条边将中心节点与其他每个节点连接起来。

给你一个二维整数数组 `edges` ，其中 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> 表示在节点 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间存在一条边。请你找出并返回 `edges` 所表示星型图的中心节点。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/03/14/star_graph.png)
<pre>
<strong>输入:</strong> edges = [[1,2],[2,3],[4,2]]
<strong>输出:</strong> 2
<strong>解释:</strong> 如上图所示，节点 2 与其他每个节点都相连，所以节点 2 是中心节点。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> edges = [[1,2],[5,1],[1,3],[1,4]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>3 <= n <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* 题目数据给出的 `edges` 表示一个有效的星型图

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[][]} edges
# @return {Integer}
def find_center(edges)
  edges[0].include?(edges[1][0]) ? edges[1][0] : edges[1][1]
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0].contains(&edges[1][0]) {
            edges[1][0]
        } else {
            edges[1][1]
        }
    }
}
```
