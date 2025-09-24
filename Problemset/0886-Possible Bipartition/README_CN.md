# 886. 可能的二分法
给定一组 `n` 人（编号为 `1, 2, ..., n`）， 我们想把每个人分进**任意**大小的两组。每个人都可能不喜欢其他人，那么他们不应该属于同一组。

给定整数 `n` 和数组 `dislikes` ，其中 <code>dislikes[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> ，表示不允许将编号为 <code>a<sub>i</sub></code> 和  <code>b<sub>i</sub></code>的人归入同一组。当可以用这种方法将所有人分进两组时，返回 `true`；否则返回 `false`。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 4, dislikes = [[1,2],[1,3],[2,4]]
<strong>输出:</strong> true
<strong>解释:</strong> group1 [1,4], group2 [2,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, dislikes = [[1,2],[1,3],[2,3]]
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 5, dislikes = [[1,2],[2,3],[3,4],[4,5],[1,5]]
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= n <= 2000`
* <code>0 <= dislikes.length <= 10<sup>4</sup></code>
* `dislikes[i].length == 2`
* <code>1 <= a<sub>i</sub> < b<sub>i</sub> <= n</code>
* `dislikes` 中每一组都 **不同**

## 题解 (Rust)

### 1. 题解
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
