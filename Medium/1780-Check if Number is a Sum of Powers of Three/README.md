# 1780. Check if Number is a Sum of Powers of Three
Given an integer `n`, return `true` *if it is possible to represent* `n` *as the sum of distinct powers of three*. Otherwise, return `false`.

An integer `y` is a power of three if there exists an integer `x` such that <code>y == 3<sup>x</sup></code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 12
<strong>Output:</strong> true
<strong>Explanation:</strong> 12 = 3<sup>1</sup> + 3<sup>2</sup>
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 91
<strong>Output:</strong> true
<strong>Explanation:</strong> 91 = 3<sup>0</sup> + 3<sup>2</sup> + 3<sup>4</sup>
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 21
<strong>Output:</strong> false
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>7</sup></code>

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
