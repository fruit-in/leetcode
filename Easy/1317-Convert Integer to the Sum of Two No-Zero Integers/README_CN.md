# 1317. 将整数转换为两个无零整数的和
「无零整数」是十进制表示中 **不含任何 0** 的正整数。

给你一个整数 ```n```，请你返回一个 **由两个整数组成的列表** ```[A, B]```，满足：
* ```A``` 和 ```B``` 都是无零整数
* ```A + B = n```

题目数据保证至少有一个有效的解决方案。

如果存在多个有效解决方案，你可以返回其中任意一个。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> [1,1]
<strong>解释:</strong> A = 1, B = 1. A + B = n 并且 A 和 B 的十进制表示形式都不包含任何 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 11
<strong>输出:</strong> [2,9]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 10000
<strong>输出:</strong> [1,9999]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 69
<strong>输出:</strong> [1,68]
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> n = 1010
<strong>输出:</strong> [11,999]
</pre>

#### 提示:
* ```2 <= n <= 10^4```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for a in 1..=(n / 2) {
            let b = n - a;
            let ab_str = a.to_string() + &b.to_string();

            if ab_str.bytes().all(|x| x != b'0') {
                return vec![a, b];
            }
        }

        Vec::new()
    }
}
```
