# 1492. n 的第 k 个因子
给你两个正整数 `n` 和 `k` 。

如果正整数 `i` 满足 `n % i == 0` ，那么我们就说正整数 `i` 是整数 `n` 的因子。

考虑整数 `n` 的所有因子，将它们 **升序排列** 。请你返回第 `k` 个因子。如果 `n` 的因子数少于 `k` ，请你返回 **-1** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 12, k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 因子列表包括 [1, 2, 3, 4, 6, 12]，第 3 个因子是 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 7, k = 2
<strong>输出:</strong> 7
<strong>解释:</strong> 因子列表包括 [1, 7] ，第 2 个因子是 7 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 4, k = 4
<strong>输出:</strong> -1
<strong>解释:</strong> 因子列表包括 [1, 2, 4] ，只有 3 个因子，所以我们应该返回 -1 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 1, k = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 因子列表包括 [1] ，第 1 个因子为 1 。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> n = 1000, k = 3
<strong>输出:</strong> 4
<strong>解释:</strong> 因子列表包括 [1, 2, 4, 5, 8, 10, 20, 25, 40, 50, 100, 125, 200, 250, 500, 1000] 。
</pre>

#### 提示:
* `1 <= k <= n <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def kthFactor(self, n: int, k: int) -> int:
        factor = 1
        i = 0

        while factor * factor <= n:
            if n % factor == 0:
                i += 1
                if i == k:
                    return factor
            factor += 1

        factor -= 1
        factor -= 1 if factor * factor == n else 0

        while factor > 0:
            if n % factor == 0:
                i += 1
                if i == k:
                    return n // factor
            factor -= 1

        return -1
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} n
# @param {Integer} k
# @return {Integer}
def kth_factor(n, k)
    factor = 1
    i = 0

    while factor * factor <= n
        if n % factor == 0
            i += 1
            return factor if i == k
        end
        factor += 1
    end

    factor -= 1
    factor -= 1 if factor * factor == n

    while factor > 0
        if n % factor == 0
            i += 1
            return n / factor if i == k
        end
        factor -= 1
    end

    return -1
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut factor = 1;
        let mut i = 0;

        while factor * factor <= n {
            if n % factor == 0 {
                i += 1;
                if i == k {
                    return factor;
                }
            }
            factor += 1;
        }

        factor -= 1;
        if factor * factor == n {
            factor -= 1;
        }

        while factor > 0 {
            if n % factor == 0 {
                i += 1;
                if i == k {
                    return n / factor;
                }
            }
            factor -= 1;
        }

        -1
    }
}
```
