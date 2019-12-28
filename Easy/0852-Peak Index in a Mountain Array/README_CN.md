# 852. 山脉数组的峰顶索引
我们把符合下列属性的数组 ```A``` 称作山脉：
* ```A.length >= 3```
* 存在 ```0 < i < A.length - 1``` 使得```A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]```

给定一个确定为山脉的数组，返回任何满足 ```A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]``` 的 ```i``` 的值。

#### 示例 1:
<pre>
<strong>输入:</strong> [0,1,0]
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [0,2,1,0]
<strong>输出:</strong> 1
</pre>

#### 提示:
1. ```3 <= A.length <= 10000```
2. ```0 <= A[i] <= 10^6```
3. A 是如上定义的山脉

## 题解 (Rust)

### 1. 线性扫描
```Rust
impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut i = 1;
        while i + 1 < a.len() && a[i] <= a[i + 1] {
            i += 1;
        }
        i as i32
    }
}
```

### 2. 二分查找
```Rust
impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut head: usize = 0;
        let mut tail = a.len() - 1;
        let mut mid: usize;
        loop {
            mid = (head + tail) / 2;
            if a[mid - 1] < a[mid] && a[mid] > a[mid + 1] {
                return mid as i32;
            } else if a[mid] < a[mid + 1] {
                head = mid;
            } else if a[mid] > a[mid + 1] {
                tail = mid;
            }
        }
    }
}
```
