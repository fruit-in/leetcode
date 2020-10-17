# 1551. Minimum Operations to Make Array Equal
You have an array `arr` of length `n` where `arr[i] = (2 * i) + 1` for all valid values of `i` (i.e. `0 <= i < n`).

In one operation, you can select two indices `x` and `y` where `0 <= x, y < n` and subtract `1` from `arr[x]` and add `1` to `arr[y]` (i.e. perform `arr[x] -=1` and `arr[y] += 1`). The goal is to make all the elements of the array **equal**. It is **guaranteed** that all the elements of the array can be made equal using some operations.

Given an integer `n`, the length of the array. Return *the minimum number of operations* needed to make all the elements of arr equal.

#### Example 1:
<pre>
<b>Input:</b> n = 3
<b>Output:</b> 2
<b>Explanation:</b> arr = [1, 3, 5]
First operation choose x = 2 and y = 0, this leads arr to be [2, 3, 4]
In the second operation choose x = 2 and y = 0 again, thus arr = [3, 3, 3].
</pre>

#### Example 2:
<pre>
<b>Input:</b> n = 6
<b>Output:</b> 9
</pre>

#### Constraints:
* `1 <= n <= 10^4`

## Solutions (Ruby)

### 1. Mathematical
```Ruby
# @param {Integer} n
# @return {Integer}
def min_operations(n)
    return n * n / 4
end
```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        n * n / 4
    }
}
```
