# 788. Rotated Digits
X is a good number if after rotating each digit individually by 180 degrees, we get a valid number that is different from X.  Each digit must be rotated - we cannot choose to leave it alone.

A number is valid if each digit remains a digit after rotation. 0, 1, and 8 rotate to themselves; 2 and 5 rotate to each other; 6 and 9 rotate to each other, and the rest of the numbers do not rotate to any other number and become invalid.

Now given a positive number ```N```, how many numbers X from ```1``` to ```N``` are good?

<pre>
<strong>Example:</strong>
<strong>Input:</strong> 10
<strong>Output:</strong> 4
<strong>Explanation:</strong>
There are four good numbers in the range [1, 10] : 2, 5, 6, 9.
Note that 1 and 10 are not good numbers, since they remain unchanged after rotating.
</pre>

#### Note:
* N  will be in range ```[1, 10000]```.

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        (2..=n).map(|num| num.to_string())
               .filter(|s| "2569".chars().any(|c| s.contains(c)))
               .filter(|s| s.chars().all(|c| "0182569".contains(c)))
               .count() as i32
    }
}
```

### 2. Mathematical
```Rust
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        fn helper(n: i32, allow_same: bool) -> i32 {
            let x = (n as f64).log10() as u32;
            let a = n / 10_i32.pow(x);
            let b = n % 10_i32.pow(x);

            match (a, allow_same) {
                (1, true) => 7_i32.pow(x) + helper(b, true),
                (2, true) => 2 * 7_i32.pow(x) + helper(b, true),
                (3, true)|(4, true) => 3 * 7_i32.pow(x),
                (5, true) => 3 * 7_i32.pow(x) + helper(b, true),
                (6, true) => 4 * 7_i32.pow(x) + helper(b, true),
                (7, true) => 5 * 7_i32.pow(x),
                (8, true) => 5 * 7_i32.pow(x) + helper(b, true),
                (9, true) => 6 * 7_i32.pow(x) + helper(b, true),
                (_, true) => 1,

                (1, false) => 7_i32.pow(x) - 3_i32.pow(x) + helper(b, false),
                (2, false) => 2 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (3, false)|(4, false) => 3 * 7_i32.pow(x) - 2 * 3_i32.pow(x),
                (5, false) => 3 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (6, false) => 4 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (7, false) => 5 * 7_i32.pow(x) - 2 * 3_i32.pow(x),
                (8, false) => 5 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, false),
                (9, false) => 6 * 7_i32.pow(x) - 3 * 3_i32.pow(x) + helper(b, true),
                (_, false) => 0,
            }
        }

        helper(n, false)
    }
}
```
