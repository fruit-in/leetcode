# 796. 旋转字符串
给定两个字符串, ```A``` 和 ```B```。

```A``` 的旋转操作就是将 ```A``` 最左边的字符移动到最右边。 例如, 若 ```A = 'abcde'```，在移动一次之后结果就是```'bcdea'``` 。如果在若干次旋转操作之后，```A``` 能变成```B```，那么返回```True```。

<pre>
<strong>示例 1:</strong>
<strong>输入:</strong> A = 'abcde', B = 'cdeab'
<strong>输出:</strong> true

<strong>示例 2:</strong>
<strong>输入:</strong> A = 'abcde', B = 'abced'
<strong>输出:</strong> false
</pre>

#### 注意:
* ```A``` 和 ```B``` 长度不超过 ```100```。

## 题解 (Rust)

### 1. 暴力法
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

### 2. A + A
```Rust
impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        let mut c = a.clone();
        c.push_str(&a);
        c.len() == 2 * b.len() && c.contains(&b)
    }
}
```
