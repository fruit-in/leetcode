# 793. 阶乘函数后 K 个零
`f(x)` 是 `x!` 末尾是 0 的数量。回想一下 `x! = 1 * 2 * 3 * ... * x`，且 `0! = 1` 。

* 例如， `f(3) = 0` ，因为 `3! = 6` 的末尾没有 0 ；而 `f(11) = 2` ，因为 `11!= 39916800` 末端有 2 个 0 。

给定 `k`，找出返回能满足 `f(x) = k` 的非负整数 `x` 的数量。

#### 示例 1:
<pre>
<strong>输入:</strong> k = 0
<strong>输出:</strong> 5
<strong>解释:</strong> 0!, 1!, 2!, 3!, 和 4! 均符合 k = 0 的条件。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 5
<strong>输出:</strong> 0
<strong>解释:</strong> 没有匹配到这样的 x!，符合 k = 5 的条件。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> k = 3
<strong>输出:</strong> 5
</pre>

#### 提示:
* <code>0 <= k <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        fn f(x: i64) -> i32 {
            let mut pow5 = 5;
            let mut ret = 0;

            while pow5 <= x {
                ret += x / pow5;
                pow5 *= 5;
            }

            ret as i32
        }

        let mut lo = 0;
        let mut hi = 800000004;

        while lo < hi {
            let mid = (lo + hi) / 2;
            let zeros = f(mid * 5);

            if zeros <= k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        5 * (f((hi - 1) * 5) == k) as i32
    }
}
```
