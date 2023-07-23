# 1318. 或运算的最小翻转次数
给你三个正整数 ```a```、```b``` 和 ```c```。

你可以对 ```a``` 和 ```b``` 的二进制表示进行位翻转操作，返回能够使按位或运算   ```a``` OR ```b``` == ```c```  成立的最小翻转次数。

「位翻转操作」是指将一个数的二进制表示任何单个位上的 1 变成 0 或者 0 变成 1 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/11/sample_3_1676.png)
<pre>
<strong>输入:</strong> a = 2, b = 6, c = 5
<strong>输出:</strong> 3
<strong>解释:</strong> 翻转后 a = 1 , b = 4 , c = 5 使得 a OR b == c
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = 4, b = 2, c = 7
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> a = 1, b = 2, c = 3
<strong>输出:</strong> 0
</pre>

#### 提示:
* ```1 <= a <= 10^9```
* ```1 <= b <= 10^9```
* ```1 <= c <= 10^9```

## 题解 (Rust)

### 1. 位操作
```Rust
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut ret = 0;

        while a > 0 || b > 0 || c > 0 {
            let d = a & 1;
            let e = b & 1;
            let f = c & 1;
            match (d << 2) + (e << 1) + f {
                0b001 | 0b010 | 0b100 => ret += 1,
                0b110 => ret += 2,
                _ => (),
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }

        ret
    }
}
```
