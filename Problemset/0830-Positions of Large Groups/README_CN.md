# 830. 较大分组的位置
在一个由小写字母构成的字符串 ```S``` 中，包含由一些连续的相同字符所构成的分组。

例如，在字符串 ```S = "abbxxxxzyy"``` 中，就含有 ```"a"```, ```"bb"```, ```"xxxx"```, ```"z"``` 和 ```"yy"``` 这样的一些分组。

我们称所有包含大于或等于三个连续字符的分组为较大分组。找到每一个较大分组的起始和终止位置。

最终结果按照字典顺序输出。

#### 示例 1:
<pre>
<strong>输入:</strong> "abbxxxxzzy"
<strong>输出:</strong> [[3,6]]
<strong>解释:</strong> "xxxx" 是一个起始于 3 且终止于 6 的较大分组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "abc"
<strong>输出:</strong> []
<strong>解释:</strong> "a","b" 和 "c" 均不是符合要求的较大分组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "abcdddeeeeaabbbcd"
<strong>输出:</strong> [[3,5],[6,9],[12,14]]
</pre>

**说明:** ```1 <= S.length <= 1000```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut groups = Vec::new();
        let mut start = 0;
        let mut count = 0;
        let mut cur_ch = ' ';
        for ch in s.chars() {
            if ch == cur_ch {
                count += 1;
            } else if ch != cur_ch {
                if count >= 3 {
                    groups.push(vec![start, start + count - 1]);
                }
                start += count;
                count = 1;
                cur_ch = ch;
            }
        }
        if count >= 3 {
            groups.push(vec![start, start + count - 1]);
        }
        groups
    }
}
```
