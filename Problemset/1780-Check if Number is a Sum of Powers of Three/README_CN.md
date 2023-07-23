# 1780. 判断一个数字是否可以表示成三的幂的和
给你一个整数 `n` ，如果你可以将 `n` 表示成若干个不同的三的幂之和，请你返回 `true` ，否则请返回 `false` 。

对于一个整数 `y` ，如果存在整数 `x` 满足 <code>y == 3<sup>x</sup></code> ，我们称这个整数 `y` 是三的幂。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 12
<strong>输出:</strong> true
<strong>解释:</strong> 12 = 3<sup>1</sup> + 3<sup>2</sup>
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 91
<strong>输出:</strong> true
<strong>解释:</strong> 91 = 3<sup>0</sup> + 3<sup>2</sup> + 3<sup>4</sup>
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 21
<strong>输出:</strong> false
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>7</sup></code>

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} n
# @return {Boolean}
def check_powers_of_three(n)
  while n > 0
    return false if n % 3 == 2

    n /= 3
  end

  true
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }

        true
    }
}
```
