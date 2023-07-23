# 2022. 将一维数组转变成二维数组
给你一个下标从 **0** 开始的一维整数数组 `original` 和两个整数 `m` 和  `n` 。你需要使用 `original` 中 **所有** 元素创建一个 `m` 行 `n` 列的二维数组。

`original` 中下标从 `0` 到 `n - 1` （都 **包含** ）的元素构成二维数组的第一行，下标从 `n` 到 `2 * n - 1` （都 **包含** ）的元素构成二维数组的第二行，依此类推。

请你根据上述过程返回一个 `m x n` 的二维数组。如果无法构成这样的二维数组，请你返回一个空的二维数组。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/08/26/image-20210826114243-1.png)
<pre>
<strong>输入:</strong> original = [1,2,3,4], m = 2, n = 2
<strong>输出:</strong> [[1,2],[3,4]]
<strong>解释:</strong>
构造出的二维数组应该包含 2 行 2 列。
original 中第一个 n=2 的部分为 [1,2] ，构成二维数组的第一行。
original 中第二个 n=2 的部分为 [3,4] ，构成二维数组的第二行。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> original = [1,2,3], m = 1, n = 3
<strong>输出:</strong> [[1,2,3]]
<strong>解释:</strong>
构造出的二维数组应该包含 1 行 3 列。
将 original 中所有三个元素放入第一行中，构成要求的二维数组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> original = [1,2], m = 1, n = 1
<strong>输出:</strong> []
<strong>解释:</strong>
original 中有 2 个元素。
无法将 2 个元素放入到一个 1x1 的二维数组中，所以返回一个空的二维数组。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> original = [3], m = 1, n = 2
<strong>输出:</strong> []
<strong>解释:</strong>
original 中只有 1 个元素。
无法将 1 个元素放满一个 1x2 的二维数组，所以返回一个空的二维数组。
</pre>

#### 提示:
* <code>1 <= original.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= original[i] <= 10<sup>5</sup></code>
* <code>1 <= m, n <= 4 * 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if (m * n) as usize != original.len() {
            return vec![];
        }

        original.chunks(n as usize).map(|a| a.to_vec()).collect()
    }
}
```
