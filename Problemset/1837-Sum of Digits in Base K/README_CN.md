# 1837. K 进制表示下的各位数字总和
给你一个整数 `n`（`10` 进制）和一个基数 `k` ，请你将 `n` 从 `10` 进制表示转换为 `k` 进制表示，计算并返回转换后各位数字的 **总和** 。

转换后，各位数字应当视作是 `10` 进制数字，且它们的总和也应当按 `10` 进制表示返回。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 34, k = 6
<strong>输出:</strong> 9
<strong>解释:</strong> 34 (10 进制) 在 6 进制下表示为 54 。5 + 4 = 9 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 10, k = 10
<strong>输出:</strong> 1
<strong>解释:</strong> n 本身就是 10 进制。 1 + 0 = 1 。
</pre>

#### 提示:
* `1 <= n <= 100`
* `2 <= k <= 10`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} n
# @param {Integer} k
# @return {Integer}
def sum_base(n, k)
  ret = 0

  while n > 0
    ret += n % k
    n /= k
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut ret = 0;

        while n > 0 {
            ret += n % k;
            n /= k;
        }

        ret
    }
}
```
