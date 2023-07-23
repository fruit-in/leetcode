# 1042. 不邻接植花
有 ```N``` 个花园，按从 ```1``` 到 ```N``` 标记。在每个花园中，你打算种下四种花之一。

```paths[i] = [x, y]``` 描述了花园 ```x``` 到花园 ```y``` 的双向路径。

另外，没有花园有 3 条以上的路径可以进入或者离开。

你需要为每个花园选择一种花，使得通过路径相连的任何两个花园中的花的种类互不相同。

以数组形式返回选择的方案作为答案 ```answer```，其中 ```answer[i]``` 为在第 ```(i+1)``` 个花园中种植的花的种类。花的种类用  1, 2, 3, 4 表示。保证存在答案。

#### 示例 1:
<pre>
<strong>输入:</strong> N = 3, paths = [[1,2],[2,3],[3,1]]
<strong>输出:</strong> [1,2,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> N = 4, paths = [[1,2],[3,4]]
<strong>输出:</strong> [1,2,1,2]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> N = 4, paths = [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]]
<strong>输出:</strong> [1,2,3,4]
</pre>

#### 提示:
* ```1 <= N <= 10000```
* ```0 <= paths.size <= 20000```
* 不存在花园有 4 条或者更多路径可以进入或离开。
* 保证存在答案。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = vec![0; n as usize];
        let mut graph = vec![Vec::new(); n as usize];

        for path in paths {
            graph[path[0] as usize - 1].push(path[1] as usize - 1);
            graph[path[1] as usize - 1].push(path[0] as usize - 1);
        }

        for i in 0..answer.len() {
            let mut choice = 0;
            for &neighbor in &graph[i] {
                if neighbor < i {
                    choice |= 1 << (answer[neighbor] - 1);
                }
            }

            match choice {
                7 => answer[i] = 4,
                3|11 => answer[i] = 3,
                1|5|9|13 => answer[i] = 2,
                _ => answer[i] = 1,
            };
        }

        answer
    }
}
```
