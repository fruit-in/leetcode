# 908. 最小差值 I
给定一个整数数组 ```A```，对于每个整数 ```A[i]```，我们可以选择任意 ```x``` 满足 ```-K <= x <= K```，并将 ```x``` 加到 ```A[i]``` 中。

在此过程之后，我们得到一些数组 ```B```。

返回 ```B``` 的最大值和 ```B``` 的最小值之间可能存在的最小差值。

#### 示例 1:
<pre>
<strong>输入:</strong> A = [1], K = 0
<strong>输出:</strong> 0
<strong>解释:</strong> B = [1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = [0,10], K = 2
<strong>输出:</strong> 6
<strong>解释:</strong> B = [2,8]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> A = [1,3,6], K = 3
<strong>输出:</strong> 0
<strong>解释:</strong> B = [3,3,3] or B = [4,4,4]
</pre>

#### 提示:
1. ```1 <= A.length <= 10000```
2. ```0 <= A[i] <= 10000```
3. ```0 <= K <= 10000```

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let mut min = a.iter().min().unwrap();
        let mut max = a.iter().max().unwrap();
        if max - min < 2 * k {
            0
        } else {
            max - min - 2 * k
        }
    }
}
```
