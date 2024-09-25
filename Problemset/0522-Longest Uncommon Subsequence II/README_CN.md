# 522. 最长特殊序列 II
给定字符串列表 `strs` ，返回其中 **最长的特殊序列** 的长度。如果最长特殊序列不存在，返回 `-1` 。

**特殊序列** 定义如下：该序列为某字符串 **独有的子序列（即不能是其他字符串的子序列）**。

`s` 的 **子序列**可以通过删去字符串 `s` 中的某些字符实现。

* 例如，`"abc"` 是 `"aebdc"` 的子序列，因为您可以删除`"aebdc"`中的下划线字符来得到 `"abc"` 。`"aebdc"`的子序列还包括`"aebdc"`、 `"aeb"` 和 `""` (空字符串)。

#### 示例 1:
<pre>
<strong>输入:</strong> strs = ["aba","cdc","eae"]
<strong>输出:</strong> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> strs = ["aaa","aaa","aa"]
<strong>输出:</strong> -1
</pre>

#### 提示:
* `2 <= strs.length <= 50`
* `1 <= strs[i].length <= 10`
* `strs[i]` 只包含小写英文字母

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut count = HashMap::new();

        for s in &strs {
            let s = s.as_bytes();
            let mut subs = HashSet::new();

            for x in 1..2_i32.pow(s.len() as u32) {
                let mut sub = vec![];

                for i in 0..s.len() {
                    if x & (1 << i) != 0 {
                        sub.push(s[i]);
                    }
                }

                subs.insert(sub);
            }

            for sub in subs.into_iter() {
                *count.entry(sub).or_insert(0) += 1;
            }
        }

        count
            .iter()
            .filter(|&(_, c)| *c == 1)
            .map(|(sub, _)| sub.len() as i32)
            .max()
            .unwrap_or(-1)
    }
}
```
