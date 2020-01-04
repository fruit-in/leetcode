# 896. 单调数列
如果数组是单调递增或单调递减的，那么它是*单调的*。

如果对于所有 ```i <= j```，```A[i] <= A[j]```，那么数组 ```A``` 是单调递增的。 如果对于所有 ```i <= j```，```A[i]> = A[j]```，那么数组 ```A``` 是单调递减的。

当给定的数组 ```A``` 是单调数组时返回 ```true```，否则返回 ```false```。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,2,3]
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [6,5,4,4]
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [1,3,2]
<strong>输出:</strong> false
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> [1,2,4,5]
<strong>输出:</strong> true
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> [1,1,1]
<strong>输出:</strong> true
</pre>

#### 提示:
1. ```1 <= A.length <= 50000```
2. ```-100000 <= A[i] <= 100000```

## 题解 (Rust)

### 1. 线性扫描
```Rust
impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut flag = 0;

        for i in 1..a.len() {
            if a[i - 1] != a[i] {
                if flag * (a[i - 1] - a[i]) >= 0 {
                    flag = a[i - 1] - a[i];
                } else {
                    return false;
                }
            }
        }

        true
    }
}
```

### 2. 排序
```Rust
impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut sort_a = a.clone();
        sort_a.sort_unstable();

        if sort_a == a {
            true
        } else {
            sort_a.reverse();
            sort_a == a
        }
    }
}
```
