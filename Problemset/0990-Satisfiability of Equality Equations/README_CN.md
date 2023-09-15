# 990. 等式方程的可满足性
给定一个由表示变量之间关系的字符串方程组成的数组，每个字符串方程 `equations[i]` 的长度为 `4`，并采用两种不同的形式之一：`"a==b"` 或 `"a!=b"`。在这里，a 和 b 是小写字母（不一定不同），表示单字母变量名。

只有当可以将整数分配给变量名，以便满足所有给定的方程时才返回 `true`，否则返回 `false`。

#### 示例 1:
<pre>
<strong>输入:</strong> equations = ["a==b","b!=a"]
<strong>输出:</strong> false
<strong>解释:</strong> 如果我们指定，a = 1 且 b = 1，那么可以满足第一个方程，但无法满足第二个方程。没有办法分配变量同时满足这两个方程。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> equations = ["b==a","a==b"]
<strong>输出:</strong> true
<strong>解释:</strong> 我们可以指定 a = 1 且 b = 1 以满足满足这两个方程。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> ["a==b","b==c","a==c"]
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> ["a==b","b!=c","c==a"]
<strong>输出:</strong> false
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> ["c==c","b==d","x!=z"]
<strong>输出:</strong> true
</pre>

#### 提示:
1. `1 <= equations.length <= 500`
2. `equations[i].length == 4`
3. `equations[i][0]` 和 `equations[i][3]` 是小写字母
4. `equations[i][1]` 要么是 `'='`，要么是 `'!'`
5. `equations[i][2]` 是 `'='`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut parent = (0..26).collect::<Vec<usize>>();

        for i in 0..equations.len() {
            let equation = equations[i].as_bytes();
            let mut x = (equation[0] - b'a') as usize;
            let mut y = (equation[3] - b'a') as usize;

            if equation[1] == b'=' {
                while parent[x] != x {
                    x = parent[x];
                }
                while parent[y] != y {
                    y = parent[y];
                }
                if x > y {
                    parent[x] = y;
                } else {
                    parent[y] = x;
                }
            }
        }

        for i in 0..26 {
            while parent[i] != parent[parent[i]] {
                parent[i] = parent[parent[i]];
            }
        }

        for i in 0..equations.len() {
            let equation = equations[i].as_bytes();
            let x = (equation[0] - b'a') as usize;
            let y = (equation[3] - b'a') as usize;

            if equation[1] == b'!' && parent[x] == parent[y] {
                return false;
            }
        }

        true
    }
}
```
