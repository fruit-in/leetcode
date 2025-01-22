# 1616. Split Two Strings to Make Palindrome
You are given two strings `a` and `b` of the same length. Choose an index and split both strings **at the same index**, splitting `a` into two strings: <code>a<sub>prefix</sub></code> and <code>a<sub>suffix</sub></code> where <code>a = a<sub>prefix</sub> + a<sub>suffix</sub></code>, and splitting `b` into two strings: <code>b<sub>prefix</sub></code> and <code>b<sub>suffix</sub></code> where <code>b = b<sub>prefix</sub> + b<sub>suffix</sub></code>. Check if <code>a<sub>prefix</sub> + b<sub>suffix</sub></code> or <code>b<sub>prefix</sub> + a<sub>suffix</sub></code> forms a palindrome.

When you split a string `s` into <code>s<sub>prefix</sub></code> and <code>s<sub>suffix</sub></code>, either <code>s<sub>suffix</sub></code> or <code>s<sub>prefix</sub></code> is allowed to be empty. For example, if `s = "abc"`, then `"" + "abc"`, `"a" + "bc"`, `"ab" + "c"` , and `"abc" + ""` are valid splits.

Return `true` *if it is possible to form a palindrome string, otherwise return* `false`.

**Notice** that `x + y` denotes the concatenation of strings `x` and `y`.

#### Example 1:
<pre>
<strong>Input:</strong> a = "x", b = "y"
<strong>Output:</strong> true
<strong>Explanation:</strong> If either a or b are palindromes the answer is true since you can split in the following way:
aprefix = "", asuffix = "x"
bprefix = "", bsuffix = "y"
Then, aprefix + bsuffix = "" + "y" = "y", which is a palindrome.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = "xbdef", b = "xecab"
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> a = "ulacfd", b = "jizalu"
<strong>Output:</strong> true
<strong>Explanation:</strong> Split them at index 3:
aprefix = "ula", asuffix = "cfd"
bprefix = "jiz", bsuffix = "alu"
Then, aprefix + bsuffix = "ula" + "alu" = "ulaalu", which is a palindrome.
</pre>

#### Constraints:
* <code>1 <= a.length, b.length <= 10<sup>5</sup></code>
* `a.length == b.length`
* `a` and `b` consist of lowercase English letters

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let n = a.len();
        let mut prefix_i = -1;

        for i in 0..n / 2 {
            if a[i] == b[n - 1 - i] {
                prefix_i = i as i32;
            } else {
                break;
            }
        }
        for i in 0..n / 2 {
            if b[i] == a[n - 1 - i] {
                prefix_i = prefix_i.max(i as i32);
            } else {
                break;
            }
        }

        if prefix_i == n as i32 / 2 - 1 {
            return true;
        }

        for i in (0..=n / 2).rev() {
            if a[i] == a[n - 1 - i] && i as i32 - 1 <= prefix_i {
                return true;
            } else if a[i] != a[n - 1 - i] {
                break;
            }
        }
        for i in (0..=n / 2).rev() {
            if b[i] == b[n - 1 - i] && i as i32 - 1 <= prefix_i {
                return true;
            } else if b[i] != b[n - 1 - i] {
                break;
            }
        }

        false
    }
}
```
