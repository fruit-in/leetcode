# 763. 划分字母区间
字符串 ```S``` 由小写字母组成。我们要把这个字符串划分为尽可能多的片段，同一个字母只会出现在其中的一个片段。返回一个表示每个字符串片段的长度的列表。

#### 示例 1:
<pre>
<strong>输入:</strong> S = "ababcbacadefegdehijhklij"
<strong>输出:</strong> [9,7,8]
<strong>解释:</strong>
划分结果为 "ababcbaca", "defegde", "hijhklij"。
每个字母最多出现在一个片段中。
像 "ababcbacadefegde", "hijhklij" 的划分是错误的，因为划分的片段数较少。
</pre>

#### 注意:
1. ```S```的长度在```[1, 500]```之间。
2. ```S```只包含小写字母```'a'```到```'z'```。

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];

        for (i, ch) in s.bytes().enumerate() {
            last[(ch - b'a') as usize] = i;
        }

        let mut l = 0;
        let mut r = 0;
        let mut ret = Vec::new();

        for (i, ch) in s.bytes().enumerate() {
            if i > r {
                ret.push((r - l) as i32 + 1);
                l = i;
                r = last[(ch - b'a') as usize];
            } else if last[(ch - b'a') as usize] > r {
                r = last[(ch - b'a') as usize];
            }
        }

        ret.push((r - l) as i32 + 1);

        ret
    }
}
```
