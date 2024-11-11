# 2246. 相邻字符不同的最长路径
给你一棵 **树**（即一个连通、无向、无环图），根节点是节点 `0` ，这棵树由编号从 `0` 到 `n - 1` 的 `n` 个节点组成。用下标从 **0** 开始、长度为 `n` 的数组 `parent` 来表示这棵树，其中 `parent[i]` 是节点 `i` 的父节点，由于节点 `0` 是根节点，所以 `parent[0] == -1` 。

另给你一个字符串 `s` ，长度也是 `n` ，其中 `s[i]` 表示分配给节点 `i` 的字符。

请你找出路径上任意一对相邻节点都没有分配到相同字符的 **最长路径** ，并返回该路径的长度。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/25/testingdrawio.png)
<pre>
<strong>输入:</strong> parent = [-1,0,0,1,1,2], s = "abacbe"
<strong>输出:</strong> 3
<strong>解释:</strong> 任意一对相邻节点字符都不同的最长路径是：0 -> 1 -> 3 。该路径的长度是 3 ，所以返回 3 。
可以证明不存在满足上述条件且比 3 更长的路径。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/25/graph2drawio.png)
<pre>
<strong>输入:</strong> parent = [-1,0,0,0], s = "aabc"
<strong>输出:</strong> 3
<strong>解释:</strong> 任意一对相邻节点字符都不同的最长路径是：2 -> 0 -> 3 。该路径的长度为 3 ，所以返回 3 。
</pre>

#### 提示:
* `n == parent.length == s.length`
* <code>1 <= n <= 105</sup></code>
* 对所有 `i >= 1` ，`0 <= parent[i] <= n - 1` 均成立
* `parent[0] == -1`
* `parent` 表示一棵有效的树
* `s` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
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
