# 977. 有序数组的平方
给定一个按非递减顺序排序的整数数组 ```A```，返回每个数字的平方组成的新数组，要求也按非递减顺序排序。

#### 示例 1:
<pre>
<strong>输入:</strong> [-4,-1,0,3,10]
<strong>输出:</strong> [0,1,9,16,100]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [-7,-3,2,3,11]
<strong>输出:</strong> [4,9,9,49,121]
</pre>

#### 提示:
1. ```1 <= A.length <= 10000```
2. ```-10000 <= A[i] <= 10000```
3. ```A``` 已按非递减顺序排序。

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut squares: Vec<i32> = a.iter().map(|&x| x * x).collect();
        squares.sort_unstable();
        squares
    }
}
```

### 2. 双指针
```Rust
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        let mut p = a.binary_search(&0).unwrap_or_else(|x| x);
        let mut n = p;
        while n > 0 || p < a.len() {
            if n == 0 || (p < a.len() && a[p] < -a[n - 1]) {
                ret.push(a[p] * a[p]);
                p += 1;
            } else {
                ret.push(a[n - 1] * a[n - 1]);
                n -= 1;
            }
        }

        ret
    }
}
```
