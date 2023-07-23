# 598. 范围求和 II
给定一个初始元素全部为 **0**，大小为 m*n 的矩阵 **M** 以及在 **M** 上的一系列更新操作。

操作用二维数组表示，其中的每个操作用一个含有两个**正整数 a** 和 **b** 的数组表示，含义是将所有符合 **0 <= i < a** 以及 **0 <= j < b** 的元素 **M[i][j]** 的值都**增加 1**。

在执行给定的一系列操作后，你需要返回矩阵中含有最大整数的元素个数。

#### 示例 1:
<pre>
<strong>输入:</strong>
m = 3, n = 3
operations = [[2,2],[3,3]]
<strong>输出:</strong> 4
<strong>解释:</strong>
初始状态, M =
[[0, 0, 0],
 [0, 0, 0],
 [0, 0, 0]]

执行完操作 [2,2] 后, M =
[[1, 1, 0],
 [1, 1, 0],
 [0, 0, 0]]

执行完操作 [3,3] 后, M =
[[2, 2, 1],
 [2, 2, 1],
 [1, 1, 1]]

M 中最大的整数是 2, 而且 M 中有4个值为2的元素。因此返回 4。
</pre>

#### 注意:
1. m 和 n 的范围是 [1,40000]。
2. a 的范围是 [1,m]，b 的范围是 [1,n]。
3. 操作数目不超过 10000。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} m
# @param {Integer} n
# @param {Integer[][]} ops
# @return {Integer}
def max_count(m, n, ops)
    min_a = ops.map {|op| op[0]}.min
    min_a = m if min_a.nil?
    min_b = ops.map {|op| op[1]}.min
    min_b = n if min_b.nil?

    return min_a * min_b
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let min_a = ops.iter().map(|op| op[0]).min().unwrap_or(m);
        let min_b = ops.iter().map(|op| op[1]).min().unwrap_or(n);

        min_a * min_b
    }
}
```
