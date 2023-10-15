# 2600. K Items With the Maximum Sum
There is a bag that consists of items, each item has a number `1`, `0`, or `-1` written on it.

You are given four **non-negative** integers `numOnes`, `numZeros`, `numNegOnes`, and `k`.

The bag initially contains:

* `numOnes` items with `1`s written on them.
* `numZeroes` items with `0`s written on them.
* `numNegOnes` items with `-1`s written on them.

We want to pick exactly `k` items among the available items. Return *the **maximum** possible sum of numbers written on the items*.

#### Example 1:
<pre>
<strong>Input:</strong> numOnes = 3, numZeros = 2, numNegOnes = 0, k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> We have a bag of items with numbers written on them {1, 1, 1, 0, 0}. We take 2 items with 1 written on them and get a sum in a total of 2.
It can be proven that 2 is the maximum possible sum.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> numOnes = 3, numZeros = 2, numNegOnes = 0, k = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> We have a bag of items with numbers written on them {1, 1, 1, 0, 0}. We take 3 items with 1 written on them, and 1 item with 0 written on it, and get a sum in a total of 3.
It can be proven that 3 is the maximum possible sum.
</pre>

#### Constraints:
* `0 <= numOnes, numZeros, numNegOnes <= 50`
* `0 <= k <= numOnes + numZeros + numNegOnes`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        if num_ones + num_zeros >= k {
            num_ones.min(k)
        } else {
            num_ones * 2 - k + num_zeros
        }
    }
}
```
