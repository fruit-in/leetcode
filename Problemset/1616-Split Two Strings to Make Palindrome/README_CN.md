# 1616. 分割两个字符串得到回文串
给你两个字符串 `a` 和 `b` ，它们长度相同。请你选择一个下标，将两个字符串都在 **相同的下标** 分割开。由 `a` 可以得到两个字符串： <code>a<sub>prefix</sub></code> 和 <code>a<sub>suffix</sub></code> ，满足 <code>a = a<sub>prefix</sub> + a<sub>suffix</sub></code> ，同理，由 `b` 可以得到两个字符串 <code>b<sub>prefix</sub></code> 和 <code>b<sub>suffix</sub></code> ，满足 <code>b = b<sub>prefix</sub> + b<sub>suffix</sub></code> 。请你判断 <code>a<sub>prefix</sub> + b<sub>suffix</sub></code> 或者 <code>b<sub>prefix</sub> + a<sub>suffix</sub></code> 能否构成回文串。

当你将一个字符串 `s` 分割成 <code>s<sub>prefix</sub></code> 和 <code>s<sub>suffix</sub></code> 时， <code>s<sub>suffix</sub></code> 或者 <code>s<sub>prefix</sub></code> 可以为空。比方说， `s = "abc"` 那么 `"" + "abc"` ， `"a" + "bc"` ， `"ab" + "c"` 和 `"abc" + ""` 都是合法分割。

如果 **能构成回文字符串** ，那么请返回 `true`，否则返回 `false` 。

**注意**， `x + y` 表示连接字符串 `x` 和 `y` 。

#### 示例 1:
<pre>
<strong>输入:</strong> a = "x", b = "y"
<strong>输出:</strong> true
<strong>解释:</strong> 如果 a 或者 b 是回文串，那么答案一定为 true ，因为你可以如下分割：
aprefix = "", asuffix = "x"
bprefix = "", bsuffix = "y"
那么 aprefix + bsuffix = "" + "y" = "y" 是回文串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = "xbdef", b = "xecab"
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> a = "ulacfd", b = "jizalu"
<strong>输出:</strong> true
<strong>解释:</strong> 在下标为 3 处分割：
aprefix = "ula", asuffix = "cfd"
bprefix = "jiz", bsuffix = "alu"
那么 aprefix + bsuffix = "ula" + "alu" = "ulaalu" 是回文串。
</pre>

#### 提示:
* <code>1 <= a.length, b.length <= 10<sup>5</sup></code>
* `a.length == b.length`
* `a` 和 `b` 都只包含小写英文字母

## 题解 (Rust)

### 1. 题解
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
