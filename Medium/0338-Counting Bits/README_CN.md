# 338. 比特位计数
给定一个非负整数 **num**。对于 **0 ≤ i ≤ num** 范围中的每个数字 **i** ，计算其二进制数中的 1 的数目并将它们作为数组返回。

#### 示例 1:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> [0,1,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> [0,1,1,2,1,2]
</pre>

#### 进阶:
* 给出时间复杂度为**O(n*sizeof(integer))**的解答非常容易。但你可以在线性时间**O(n)**内用一趟扫描做到吗？
* 要求算法的空间复杂度为**O(n)**。
* 你能进一步完善解法吗？要求在C++或任何其他语言中不使用任何内置函数（如 C++ 中的 **__builtin_popcount**）来执行此操作。

## 题解 (Rust)

### 1. i % n (n = 2<sup>⌊log<sub>2</sub>(n)⌋</sup>)
```Rust
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut n = 1;
        let mut result = vec![0];
        for i in 1..=num as usize{
            n *= (i / n);
            result.push(&result[i % n] + 1);
        }
        result
    }
}
```

### 2. i & (i - 1)
```Rust
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut result = vec![0];
        for i in 1..=num as usize{
            result.push(&result[i & (i - 1)] + 1);
        }
        result
    }
}
```
