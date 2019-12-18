# 796. Rotate String
We are given two strings, ```A``` and ```B```.

A *shift on ```A```* consists of taking string ```A``` and moving the leftmost character to the rightmost position. For example, if ```A = 'abcde'```, then it will be ```'bcdea'``` after one shift on ```A```. Return ```True``` if and only if ```A``` can become ```B``` after some number of shifts on ```A```.

<pre>
<strong>Example 1:</strong>
<strong>Input:</strong> A = 'abcde', B = 'cdeab'
<strong>Output:</strong> true

<strong>Example 2:</strong>
<strong>Input:</strong> A = 'abcde', B = 'abced'
<strong>Output:</strong> false
</pre>

#### Note:
* ```A``` and ```B``` will have length at most ```100```.

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        let mut a = a;

        if a.len() == b.len() {
            for _ in 0..=a.len() {
                if a == b {
                    return true;
                }

                let ch = a.remove(0);
                a.push(ch);
            }
        }

        false
    }
}
```

### 2. Check A + A
```Rust
impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        let mut c = a.clone();
        c.push_str(&a);
        c.len() == 2 * b.len() && c.contains(&b)
    }
}
```
