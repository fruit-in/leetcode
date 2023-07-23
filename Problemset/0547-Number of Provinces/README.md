# 547. Number of Provinces
There are `n` cities. Some of them are connected, while some are not. If city `a` is connected directly with city `b`, and city `b` is connected directly with city `c`, then city `a` is connected indirectly with city `c`.

A **province** is a group of directly or indirectly connected cities and no other cities outside of the group.

You are given an `n x n` matrix `isConnected` where `isConnected[i][j] = 1` if the <code>i<sup>th</sup></code> city and the <code>j<sup>th</sup></code> city are directly connected, and `isConnected[i][j] = 0` otherwise.

Return *the total number of **provinces***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/12/24/graph1.jpg)
<pre>
<strong>Input:</strong> isConnected = [[1,1,0],[1,1,0],[0,0,1]]
<strong>Output:</strong> 2
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/12/24/graph2.jpg)
<pre>
<strong>Input:</strong> isConnected = [[1,0,0],[0,1,0],[0,0,1]]
<strong>Output:</strong> 3
</pre>

#### Constraints:
* `1 <= n <= 200`
* `n == isConnected.length`
* `n == isConnected[i].length`
* `isConnected[i][j]` is `1` or `0`.
* `isConnected[i][i] == 1`
* `isConnected[i][j] == isConnected[j][i]`

## Solutions (Ruby)

### 1. DFS
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

## Solutions (Rust)

### 1. DFS
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
