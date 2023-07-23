# 1006. Clumsy Factorial
Normally, the factorial of a positive integer ```n``` is the product of all positive integers less than or equal to ```n```.  For example, ```factorial(10) = 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1```.

We instead make a *clumsy factorial*: using the integers in decreasing order, we swap out the multiply operations for a fixed rotation of operations: multiply (*), divide (/), add (+) and subtract (-) in this order.

For example, ```clumsy(10) = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1```.  However, these operations are still applied using the usual order of operations of arithmetic: we do all multiplication and division steps before any addition or subtraction steps, and multiplication and division steps are processed left to right.

Additionally, the division that we use is *floor division* such that ```10 * 9 / 8``` equals ```11```.  This guarantees the result is an integer.

```Implement the clumsy``` function as defined above: given an integer ```N```, it returns the clumsy factorial of ```N```.

#### Example 1:
<pre>
<strong>Input:</strong> 4
<strong>Output:</strong> 7
<strong>Explanation:</strong> 7 = 4 * 3 / 2 + 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 10
<strong>Output:</strong> 12
<strong>Explanation:</strong> 12 = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1
</pre>

#### Note:
1. ```1 <= N <= 10000```
2. ```-2^31 <= answer <= 2^31 - 1```  (The answer is guaranteed to fit within a 32-bit integer.)

## Solutions (Ruby)

### 1. Mathematical
```Ruby
# @param {Integer} n
# @return {Integer}
def clumsy(n)
    if n % 4 == 0
        return [n + 1, 7].max
    elsif n < 3
        return n
    elsif n % 4 == 1 or n % 4 == 2
        return n + 2
    else
        return [n - 1, 6].max
    end
end
```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        match n % 4 {
            0 => (n + 1).max(7),
            1 | 2 if n < 4 => n,
            1 | 2 => n + 2,
            _ => (n - 1).max(6),
        }
    }
}
```
