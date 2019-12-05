# 566. 重塑矩阵
在MATLAB中，有一个非常有用的函数 ```reshape```，它可以将一个矩阵重塑为另一个大小不同的新矩阵，但保留其原始数据。

给出一个由二维数组表示的矩阵，以及两个正整数```r```和```c```，分别表示想要的重构的矩阵的行数和列数。

重构后的矩阵需要将原始矩阵的所有元素以相同的**行遍历顺序**填充。

如果具有给定参数的```reshape```操作是可行且合理的，则输出新的重塑矩阵；否则，输出原始矩阵。

#### 示例 1:
<pre>
<strong>输入:</strong>
nums =
[[1,2],
 [3,4]]
r = 1, c = 4
<strong>输出:</strong>
[[1,2,3,4]]
<strong>解释:</strong>
行遍历nums的结果是 [1,2,3,4]。新的矩阵是 1 * 4 矩阵, 用之前的元素值一行一行填充新矩阵。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
nums =
[[1,2],
 [3,4]]
r = 2, c = 4
<strong>输出:</strong>
[[1,2],
 [3,4]]
<strong>解释:</strong>
没有办法将 2 * 2 矩阵转化为 2 * 4 矩阵。 所以输出原矩阵。
</pre>

#### 注意:
1. 给定矩阵的宽和高范围在 [1, 100]。
2. 给定的 r 和 c 都是正数。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if nums.len() * nums[0].len() != (r * c) as usize {
            return nums;
        }

        nums.concat()
            .chunks(c as usize)
            .map(|row| row.to_vec())
            .collect()
    }
}
```
