# 2094. Finding 3-Digit Even Numbers
You are given an integer array `digits`, where each element is a digit. The array may contain duplicates.

You need to find **all** the **unique** integers that follow the given requirements:
* The integer consists of the **concatenation** of **three** elements from `digits` in **any** arbitrary order.
* The integer does not have **leading zeros**.
* The integer is **even**.

For example, if the given `digits` were `[1, 2, 3]`, integers `132` and `312` follow the requirements.

Return *a **sorted** array of the unique integers*.

#### Example 1:
<pre>
<strong>Input:</strong> digits = [2,1,3,0]
<strong>Output:</strong> 102,120,130,132,210,230,302,310,312,320]
<strong>Explanation:</strong> All the possible integers that follow the requirements are in the output array.
Notice that there are no odd integers or integers with leading zeros.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> digits = [2,2,8,8,2]
<strong>Output:</strong> [222,228,282,288,822,828,882]
<strong>Explanation:</strong> The same digit can be used as many times as it appears in digits.
In this example, the digit 8 is used twice each time in 288, 828, and 882.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> digits = [3,7,5]
<strong>Output:</strong> []
<strong>Explanation:</strong> No even integers can be formed using the given digits.
</pre>

#### Constraints:
* `3 <= digits.length <= 100`
* `0 <= digits[i] <= 9`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut ret = vec![];
        digits.sort_unstable();

        for i in 0..digits.len() {
            for j in 0..digits.len() {
                for k in 0..digits.len() {
                    let x = digits[i] * 100 + digits[j] * 10 + digits[k];

                    if i != j && j != k && i != k && x % 2 == 0 && x > *ret.last().unwrap_or(&99) {
                        ret.push(x);
                    }
                }
            }
        }

        ret
    }
}
```
