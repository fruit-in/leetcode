# 866. 回文素数
求出大于或等于 `N` 的最小回文素数。

回顾一下，如果一个数大于 1，且其因数只有 1 和它自身，那么这个数是*素数*。

例如，2，3，5，7，11 以及 13 是素数。

回顾一下，如果一个数从左往右读与从右往左读是一样的，那么这个数是*回文数*。

例如，12321 是回文数。

#### 示例 1:
<pre>
<strong>输入:</strong> 6
<strong>输出:</strong> 7
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 8
<strong>输出:</strong> 11
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 13
<strong>输出:</strong> 101
</pre>

#### 提示:
* `1 <= N <= 10^8`
* 答案肯定存在，且小于 `2 * 10^8`。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn prime_palindrome(mut n: i32) -> i32 {
        loop {
            match n {
                999 | 99_999 | 9_999_999 => n = n * 10 + 11,
                n if Self::is_palindrome(n) && Self::is_prime(n) => return n,
                _ => n += 1,
            }
        }
    }

    fn is_palindrome(mut n: i32) -> bool {
        if n % 10 == 0 {
            return false;
        }

        let mut rev = 0;

        while n > rev {
            rev *= 10;
            rev += n % 10;
            n /= 10;
        }

        n == rev || n == rev / 10
    }

    fn is_prime(n: i32) -> bool {
        let mut i = 2;

        while i * i <= n && n % i != 0 {
            i += 1;
        }

        n > 1 && i * i > n
    }
}
```
