# 941. 有效的山脉数组
给定一个整数数组 ```A```，如果它是有效的山脉数组就返回 ```true```，否则返回 ```false```。

让我们回顾一下，如果 A 满足下述条件，那么它是一个山脉数组：
* ```A.length >= 3```
* 在 ```0 < i < A.length - 1``` 条件下，存在 ```i``` 使得：
    * ```A[0] < A[1] < ... A[i-1] < A[i]```
    * ```A[i] > A[i+1] > ... > A[B.length - 1]```

#### 示例 1:
<pre>
<strong>Input:</strong> [2,1]
<strong>Output:</strong> false
</pre>

#### 示例 2:
<pre>
<strong>Input:</strong> [3,5,5]
<strong>Output:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>Input:</strong> [0,3,2,1]
<strong>Output:</strong> true
</pre>

#### 提示:
1. ```0 <= A.length <= 10000```
2. ```0 <= A[i] <= 10000 ```

## 题解 (Rust)

### 1. 单次遍历
```Rust
impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let length = a.len();
        let mut i = 0;
        while i + 1 < length && a[i] < a[i + 1] {
            i += 1;
        }
        if i == 0 || i + 1 == length {
            return false;
        }
        while i + 1 < length && a[i] > a[i + 1] {
            i += 1;
        }
        i + 1 == length
    }
}
```
