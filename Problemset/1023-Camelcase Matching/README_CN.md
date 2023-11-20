# 1023. 驼峰式匹配
给你一个字符串数组 `queries`，和一个表示模式的字符串 `pattern`，请你返回一个布尔数组 `answer` 。只有在待查项 `queries[i]` 与模式串 `pattern` 匹配时， `answer[i]` 才为 `true`，否则为 `false`。

如果可以将**小写字母**插入模式串 `pattern` 得到待查询项 `query`，那么待查询项与给定模式串匹配。可以在任何位置插入每个字符，也可以不插入字符。

#### 示例 1:
<pre>
<strong>输入:</strong> queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FB"
<strong>输出:</strong> [true,false,true,true,false]
<strong>解释:</strong>
"FooBar" 可以这样生成："F" + "oo" + "B" + "ar"。
"FootBall" 可以这样生成："F" + "oot" + "B" + "all".
"FrameBuffer" 可以这样生成："F" + "rame" + "B" + "uffer".
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBa"
<strong>输出:</strong> [true,false,true,false,false]
<strong>解释:</strong>
"FooBar" 可以这样生成："Fo" + "o" + "Ba" + "r".
"FootBall" 可以这样生成："Fo" + "ot" + "Ba" + "ll".
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBaT"
<strong>输出:</strong> [false,true,false,false,false]
<strong>解释:</strong>
"FooBarTest" 可以这样生成："Fo" + "o" + "Ba" + "r" + "T" + "est".
</pre>

#### 提示:
* `1 <= pattern.length, queries.length <= 100`
* `1 <= queries[i].length <= 100`
* `queries[i]` 和 `pattern` 由英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pattern = pattern.as_bytes();
        let mut ret = vec![true; queries.len()];

        for i in 0..queries.len() {
            let query = queries[i].as_bytes();
            let mut j = 0;

            for k in 0..query.len() {
                if j < pattern.len() && query[k] == pattern[j] {
                    j += 1;
                } else if query[k].is_ascii_uppercase() {
                    ret[i] = false;
                    break;
                }
            }

            ret[i] &= j == pattern.len();
        }

        ret
    }
}
```
