# 921. 使括号有效的最少添加
给定一个由 ```'('``` 和 ```')'``` 括号组成的字符串 ```S```，我们需要添加最少的括号（ ```'('``` 或是 ```')'```，可以在任何位置），以使得到的括号字符串有效。

从形式上讲，只有满足下面几点之一，括号字符串才是有效的：
* 它是一个空字符串，或者
* 它可以被写成 ```AB``` （```A``` 与 ```B``` 连接）, 其中 ```A``` 和 ```B``` 都是有效字符串，或者
* 它可以被写作 ```(A)```，其中 ```A``` 是有效字符串。

给定一个括号字符串，返回为使结果字符串有效而必须添加的最少括号数。

#### 示例 1:
<pre>
<strong>输入:</strong> "())"
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "((("
<strong>输出:</strong> 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "()"
<strong>输出:</strong> 0
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> "()))(("
<strong>输出:</strong> 4
</pre>

#### 提示:
1. <code>S.length <= 1000</code>
2. ```S``` 只包含 ```'('``` 和 ```')'``` 字符。

## 题解 (Rust)

### 1. 从字符串中移除有效括号
```Rust
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut s = s;
        while s.contains("()") {
            s = s.replace("()", "");
        }
        s.len() as i32
    }
}
```

### 2. 通过栈移除有效括号
```Rust
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            if ch == ')' && stack.ends_with(&['(']) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }
        stack.len() as i32
    }
}
```

### 3. 平衡
```Rust
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for ch in s.chars() {
            if ch == '(' {
                left += 1
            } else if left > 0{
                left -= 1
            } else {
                right += 1
            }
        }
        left + right
    }
}
```
