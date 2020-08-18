# 1492. The kth Factor of n
Given two positive integers `n` and `k`.

A factor of an integer `n` is defined as an integer `i` where `n % i == 0`.

Consider a list of all factors of `n` sorted in **ascending order**, return *the* `kth` *factor* in this list or return **-1** if `n` has less than `k` factors.

#### Example 1:
<pre>
<strong>Input:</strong> n = 12, k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> Factors list is [1, 2, 3, 4, 6, 12], the 3rd factor is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 7, k = 2
<strong>Output:</strong> 7
<strong>Explanation:</strong> Factors list is [1, 7], the 2nd factor is 7.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 4, k = 4
<strong>Output:</strong> -1
<strong>Explanation:</strong> Factors list is [1, 2, 4], there is only 3 factors. We should return -1.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 1, k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> Factors list is [1], the 1st factor is 1.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> n = 1000, k = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> Factors list is [1, 2, 4, 5, 8, 10, 20, 25, 40, 50, 100, 125, 200, 250, 500, 1000].
</pre>

#### Constraints:
* `1 <= k <= n <= 1000`

## Solutions (Python)

### 1. Solution
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

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
