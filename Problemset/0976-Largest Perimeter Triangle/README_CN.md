# 976. 三角形的最大周长
给定由一些正数（代表长度）组成的数组 ```A```，返回由其中三个长度组成的、**面积不为零**的三角形的最大周长。

如果不能形成任何面积不为零的三角形，返回 ```0```。

#### 示例 1:
<pre>
<strong>输入:</strong> [2,1,2]
<strong>输出:</strong> 5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,1]
<strong>输出:</strong> 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [3,2,3,4]
<strong>输出:</strong> 10
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> [3,6,2,3]
<strong>输出:</strong> 8
</pre>

#### 提示:
1. ```3 <= A.length <= 10000```
2. ```1 <= A[i] <= 10^6```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let mut a = a;
        a.sort_unstable_by(|a, b| b.cmp(a));

        for i in 2..a.len() {
            if a[i] + a[i - 1] > a[i - 2] {
                return a[i] + a[i - 1] + a[i - 2];
            }
        }

        0
    }
}
```
