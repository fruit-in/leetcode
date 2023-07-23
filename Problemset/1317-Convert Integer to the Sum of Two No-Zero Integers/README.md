# 1317. Convert Integer to the Sum of Two No-Zero Integers
Given an integer ```n```. No-Zero integer is a positive integer which **doesn't contain any 0** in its decimal representation.

Return *a list of two integers* ```[A, B]``` where:
* ```A``` and ```B``` are No-Zero integers.
* ```A + B = n```

It's guarateed that there is at least one valid solution. If there are many valid solutions you can return any of them.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> [1,1]
<strong>Explanation:</strong> A = 1, B = 1. A + B = n and both A and B don't contain any 0 in their decimal representation.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 11
<strong>Output:</strong> [2,9]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 10000
<strong>Output:</strong> [1,9999]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 69
<strong>Output:</strong> [1,68]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> n = 1010
<strong>Output:</strong> [11,999]
</pre>

#### Constraints:
* ```2 <= n <= 10^4```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for a in 1..=(n / 2) {
            let b = n - a;
            let ab_str = a.to_string() + &b.to_string();

            if ab_str.bytes().all(|x| x != b'0') {
                return vec![a, b];
            }
        }

        Vec::new()
    }
}
```
