# 1969. Minimum Non-Zero Product of the Array Elements
You are given a positive integer `p`. Consider an array `nums` (**1-indexed**) that consists of the integers in the **inclusive** range <code>[1, 2<sup>p</sup> - 1]</code> in their binary representations. You are allowed to do the following operation **any** number of times:

* Choose two elements `x` and `y` from nums.
* Choose a bit in `x` and swap it with its corresponding bit in `y`. Corresponding bit refers to the bit that is in the **same position** in the other integer.

For example, if `x = 1101` and `y = 0011`, after swapping the <code>2<sup>nd</sup></code> bit from the right, we have `x = 1111` and `y = 0001`.

Find the **minimum non-zero** product of `nums` after performing the above operation **any** number of times. Return *this product **modulo*** <code>10<sup>9</sup> + 7</code>.

**Note:** The answer should be the minimum product **before** the modulo operation is done.

#### Example 1:
<pre>
<strong>Input:</strong> p = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> nums = [1].
There is only one element, so the product equals that element.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> p = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> nums = [01, 10, 11].
Any swap would either make the product 0 or stay the same.
Thus, the array product of 1 * 2 * 3 = 6 is already minimized.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> p = 3
<strong>Output:</strong> 1512
<strong>Explanation:</strong> nums = [001, 010, 011, 100, 101, 110, 111]
- In the first operation we can swap the leftmost bit of the second and fifth elements.
    - The resulting array is [001, 110, 011, 100, 001, 110, 111].
- In the second operation we can swap the middle bit of the third and fourth elements.
    - The resulting array is [001, 110, 001, 110, 001, 110, 111].
The array product is 1 * 6 * 1 * 6 * 1 * 6 * 7 = 1512, which is the minimum possible product.
</pre>

#### Constraints:
* `1 <= p <= 60`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        let x = 2_u64.pow(p as u32) % 1_000_000_007;

        ((x - 1) * Self::power(x - 2, 2_u64.pow(p as u32 - 1) - 1) % 1_000_000_007) as i32
    }

    fn power(x: u64, exp: u64) -> u64 {
        if exp == 0 {
            1
        } else if exp % 2 == 0 {
            let y = Self::power(x, exp / 2);
            y * y % 1_000_000_007
        } else {
            x * Self::power(x, exp - 1) % 1_000_000_007
        }
    }
}
```
