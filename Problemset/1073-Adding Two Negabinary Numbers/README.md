# 1073. Adding Two Negabinary Numbers
Given two numbers `arr1` and `arr2` in base **-2**, return the result of adding them together.

Each number is given in *array format*:  as an array of 0s and 1s, from most significant bit to least significant bit.  For example, `arr = [1,1,0,1]` represents the number `(-2)^3 + (-2)^2 + (-2)^0 = -3`.  A number `arr` in *array, format* is also guaranteed to have no leading zeros: either `arr == [0]` or `arr[0] == 1`.

Return the result of adding `arr1` and `arr2` in the same format: as an array of 0s and 1s with no leading zeros.

#### Example 1:
<pre>
<strong>Input:</strong> arr1 = [1,1,1,1,1], arr2 = [1,0,1]
<strong>Output:</strong> [1,0,0,0,0]
<strong>Explanation:</strong> arr1 represents 11, arr2 represents 5, the output represents 16.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr1 = [0], arr2 = [0]
<strong>Output:</strong> [0]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr1 = [0], arr2 = [1]
<strong>Output:</strong> [1]
</pre>

#### Constraints:
* `1 <= arr1.length, arr2.length <= 1000`
* `arr1[i]` and `arr2[i]` are `0` or `1`
* `arr1` and `arr2` have no leading zeros

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1;
        let mut arr2 = arr2;
        let mut carry = 0;
        let mut ret = vec![];

        while !arr1.is_empty() || !arr2.is_empty() || carry != 0 {
            let x = arr1.pop().unwrap_or(0) + arr2.pop().unwrap_or(0) + carry;
            carry = 1 - (x + 2) / 2;
            ret.push(x.rem_euclid(2));
        }

        while ret.len() > 1 && *ret.last().unwrap() == 0 {
            ret.pop();
        }

        ret.reverse();

        ret
    }
}
```
