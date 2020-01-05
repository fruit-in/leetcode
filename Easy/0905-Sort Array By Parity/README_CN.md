# 905. 按奇偶排序数组
给定一个非负整数数组 ```A```，返回一个数组，在该数组中， ```A``` 的所有偶数元素之后跟着所有奇数元素。

你可以返回满足此条件的任何数组作为答案。

#### 示例:
<pre>
<strong>输入:</strong> [3,1,2,4]
<strong>输出:</strong> [2,4,3,1]
输出 [4,2,3,1]，[2,4,1,3] 和 [4,2,1,3] 也会被接受。
</pre>

#### 提示:
1. ```1 <= A.length <= 5000```
2. ```0 <= A[i] <= 5000```

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut i = 0;
        let mut j = a.len() - 1;
        while i < j {
            while i < j && a[i] % 2 == 0 {
                i += 1;
            }
            while i < j && a[j] % 2 == 1 {
                j -= 1;
            }
            a.swap(i, j);
        }
        a
    }
}
```

### 2. 单次遍历
```Rust
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut even = Vec::new();
        let mut odd = Vec::new();
        for n in a {
            match n % 2 == 0 {
                true => even.push(n),
                false => odd.push(n),
            };
        }
        even.append(&mut odd);
        even
    }
}
```

### 3. 排序
```Rust
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        a.sort_unstable_by_key(|k| k % 2 == 1);
        a
    }
}
```
