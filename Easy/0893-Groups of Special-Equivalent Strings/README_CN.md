# 893. 特殊等价字符串组
你将得到一个字符串数组 ```A```。

如果经过任意次数的移动，S == T，那么两个字符串 ```S``` 和 ```T``` 是**特殊等价**的。

一次**移动**包括选择两个索引 ```i``` 和 ```j```，且 ```i ％ 2 == j ％ 2```，交换 ```S[j]``` 和 ```S [i]```。

现在规定，```A``` **中的特殊等价字符串组**是 ```A``` 的非空子集 ```S```，这样不在 ```S``` 中的任何字符串与 ```S``` 中的任何字符串都不是特殊等价的。

返回 ```A``` 中特殊等价字符串组的数量。

#### 示例 1:
<pre>
<strong>输入:</strong> ["a","b","c","a","c","c"]
<strong>输出:</strong> 3
<strong>解释:</strong> 3 组 ["a","a"]，["b"]，["c","c","c"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ["aa","bb","ab","ba"]
<strong>输出:</strong> 4
<strong>解释:</strong> 4 组 ["aa"]，["bb"]，["ab"]，["ba"]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> ["abc","acb","bac","bca","cab","cba"]
<strong>输出:</strong> 3
<strong>解释:</strong> 3 组 ["abc","cba"]，["acb","bca"]，["bac","cab"]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> ["abcd","cdab","adcb","cbad"]
<strong>输出:</strong> 1
<strong>解释:</strong> 1 组 ["abcd","cdab","adcb","cbad"]
</pre>

#### 提示:
* ```1 <= A.length <= 1000```
* ```1 <= A[i].length <= 20```
* 所有 ```A[i]``` 都具有相同的长度。
* 所有 ```A[i]``` 都只由小写字母组成。

## 题解 (Rust)

### 1. 计数
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        let mut set = HashSet::new();

        for s in a {
            let mut cnt = vec![0; 52];
            for (i, c) in s.bytes().enumerate() {
                cnt[(c - b'a') as usize + 26 * (i % 2)] += 1;
            }
            set.insert(cnt);
        }

        set.len() as i32
    }
}
```
