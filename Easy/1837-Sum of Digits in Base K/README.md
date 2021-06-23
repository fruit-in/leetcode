# 1837. Sum of Digits in Base K
Given an integer `n` (in base `10`) and a base `k`, return *the **sum** of the digits of* `n` ***after*** *converting* `n` *from base* `10` *to base* `k`.

After converting, each digit should be interpreted as a base `10` number, and the sum should be returned in base `10`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 34, k = 6
<strong>Output:</strong> 9
<strong>Explanation:</strong> 34 (base 10) expressed in base 6 is 54. 5 + 4 = 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 10, k = 10
<strong>Output:</strong> 1
<strong>Explanation:</strong> n is already in base 10. 1 + 0 = 1.
</pre>

#### Constraints:
* `1 <= n <= 100`
* `2 <= k <= 10`

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
