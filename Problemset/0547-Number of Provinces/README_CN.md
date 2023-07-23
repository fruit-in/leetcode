# 547. 省份数量
有 `n` 个城市，其中一些彼此相连，另一些没有相连。如果城市 `a` 与城市 `b` 直接相连，且城市 `b` 与城市 `c` 直接相连，那么城市 `a` 与城市 `c` 间接相连。

**省份** 是一组直接或间接相连的城市，组内不含其他没有相连的城市。

给你一个 `n x n` 的矩阵 `isConnected` ，其中 `isConnected[i][j] = 1` 表示第 `i` 个城市和第 `j` 个城市直接相连，而 `isConnected[i][j] = 0` 表示二者不直接相连。

返回矩阵中 **省份** 的数量。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/12/24/graph1.jpg)
<pre>
<strong>输入:</strong> isConnected = [[1,1,0],[1,1,0],[0,0,1]]
<strong>输出:</strong> 2
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/12/24/graph2.jpg)
<pre>
<strong>输入:</strong> isConnected = [[1,0,0],[0,1,0],[0,0,1]]
<strong>输出:</strong> 3
</pre>

#### 提示:
* `1 <= n <= 200`
* `n == isConnected.length`
* `n == isConnected[i].length`
* `isConnected[i][j]` 为 `1` 或 `0`
* `isConnected[i][i] == 1`
* `isConnected[i][j] == isConnected[j][i]`

## 题解 (Ruby)

### 1. 深度优先搜索
```Ruby
# @param {Integer[][]} is_connected
# @return {Integer}
def find_circle_num(is_connected)
  seen = [false] * is_connected.size
  stack = []
  ret = 0

  (0...is_connected.size).each do |i|
    next if seen[i]

    ret += 1
    stack.push(i)

    until stack.empty?
      c = stack.pop
      seen[c] = true
      (i + 1...is_connected.size).each do |j|
        stack.push(j) if !seen[j] && is_connected[c][j] == 1
      end
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 深度优先搜索
```Rust
impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![false; m.len()];
        let mut stack = vec![];
        let mut ret = 0;

        for i in 0..m.len() {
            if !seen[i] {
                ret += 1;
                stack.push(i);

                while let Some(c) = stack.pop() {
                    seen[c] = true;
                    for j in i + 1..m.len() {
                        if !seen[j] && m[c][j] == 1 {
                            stack.push(j);
                        }
                    }
                }
            }
        }

        ret
    }
}
```
