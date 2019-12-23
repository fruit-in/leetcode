# 771. 宝石与石头
给定字符串```J``` 代表石头中宝石的类型，和字符串 ```S```代表你拥有的石头。 ```S``` 中每个字符代表了一种你拥有的石头的类型，你想知道你拥有的石头中有多少是宝石。

```J``` 中的字母不重复，```J``` 和 ```S```中的所有字符都是字母。字母区分大小写，因此```"a"```和```"A"```是不同类型的石头。

#### 示例 1:
<pre>
<strong>输入:</strong> J = "aA", S = "aAAbbbb"
<strong>输出:</strong> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> J = "z", S = "ZZ"
<strong>输出:</strong> 0
</pre>

#### 注意:
* ```S``` 和 ```J``` 最多含有50个字母。
* ```J``` 中的字符不重复。

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut jewels = HashMap::new();
        for ch_j in j.chars() {
            jewels.insert(ch_j, 0);
        }
        for ch_s in s.chars() {
            if let Some(i) = jewels.get(&ch_s) {
                jewels.insert(ch_s, i + 1);
            }
        }
        jewels.values().sum()
    }
}
```
