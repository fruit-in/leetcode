# 1835. Find XOR Sum of All Pairs Bitwise AND
The **XOR sum** of a list is the bitwise `XOR` of all its elements. If the list only contains one element, then its **XOR sum** will be equal to this element.

* For example, the **XOR sum** of `[1,2,3,4]` is equal to `1 XOR 2 XOR 3 XOR 4 = 4`, and the **XOR sum** of `[3]` is equal to `3`.

You are given two **0-indexed** arrays `arr1` and `arr2` that consist only of non-negative integers.

Consider the list containing the result of `arr1[i] AND arr2[j]` (bitwise `AND`) for every `(i, j)` pair where `0 <= i < arr1.length` and `0 <= j < arr2.length`.

Return *the **XOR sum** of the aforementioned list*.

#### Example 1:
<pre>
<strong>Input:</strong> arr1 = [1,2,3], arr2 = [6,5]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The list = [1 AND 6, 1 AND 5, 2 AND 6, 2 AND 5, 3 AND 6, 3 AND 5] = [0,1,2,0,2,1].
The XOR sum = 0 XOR 1 XOR 2 XOR 0 XOR 2 XOR 1 = 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr1 = [12], arr2 = [4]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The list = [12 AND 4] = [4]. The XOR sum = 4.
</pre>

#### Constraints:
* <code>1 <= arr1.length, arr2.length <= 10<sup>5</sup></code>
* <code>0 <= arr1[i], arr2[j] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let xor1 = arr1.iter().fold(0, |acc, x| acc ^ x);
        arr2.iter().fold(0, |acc, x| acc ^ (x & xor1))
    }
}
```
