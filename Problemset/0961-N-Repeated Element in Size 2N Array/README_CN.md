# 961. 重复 N 次的元素
在大小为 ```2N``` 的数组 ```A``` 中有 ```N+1``` 个不同的元素，其中有一个元素重复了 ```N``` 次。

返回重复了 ```N``` 次的那个元素。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3,3]
<strong>输出:</strong> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [2,1,2,5,3,2]
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [5,1,5,2,5,3,5,4]
<strong>输出:</strong> 5
</pre>

#### 提示:
1. ```4 <= A.length <= 10000```
2. ```0 <= A[i] < 10000```
3. ```A.length``` 为偶数

## 题解 (Rust)

### 1. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for n in a {
            if set.contains(&n) {
                return n;
            } else {
                set.insert(n);
            }
        }

        -1
    }
}
```

### 2. 比较
```Rust
impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        for i in 1..4 {
            for j in i..a.len() {
                if a[j - i] == a[j] {
                    return a[j];
                }
            }
        }

        -1
    }
}
```
