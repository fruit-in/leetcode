# 1733. 需要教语言的最少人数
在一个由 `m` 个用户组成的社交网络里，我们获取到一些用户之间的好友关系。两个用户之间可以相互沟通的条件是他们都掌握同一门语言。

给你一个整数 `n` ，数组 `languages` 和数组 `friendships` ，它们的含义如下：

* 总共有 `n` 种语言，编号从 `1` 到 `n` 。
* `languages[i]` 是第 `i` 位用户掌握的语言集合。
* <code>friendships[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> 表示 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 为好友关系。

你可以选择 **一门** 语言并教会一些用户，使得所有好友之间都可以相互沟通。请返回你 **最少** 需要教会多少名用户。

请注意，好友关系没有传递性，也就是说如果 `x` 和 `y` 是好友，且 `y` 和 `z` 是好友， `x` 和 `z` 不一定是好友。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
<strong>输出:</strong> 1
<strong>解释:</strong> 你可以选择教用户 1 第二门语言，也可以选择教用户 2 第一门语言。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
<strong>输出:</strong> 2
<strong>解释:</strong> 教用户 1 和用户 3 第三门语言，需要教 2 名用户。
</pre>

#### 提示:
* `2 <= n <= 500`
* `languages.length == m`
* `1 <= m <= 500`
* `1 <= languages[i].length <= n`
* `1 <= languages[i][j] <= n`
* <code>1 <= u<sub>i</sub> < v<sub>i</sub> <= languages.length</code>
* `1 <= friendships.length <= 500`
* 所有的好友关系 <code>(u<sub>i</sub>, v<sub>i</sub>)</code> 都是唯一的。
* `languages[i]` 中包含的值互不相同。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut teach_users: HashMap<i32, HashSet<i32>> = HashMap::new();

        for friendship in &friendships {
            let u = friendship[0];
            let v = friendship[1];
            let languages_u = languages[u as usize - 1].iter().collect::<HashSet<_>>();
            let languages_v = languages[v as usize - 1].iter().collect::<HashSet<_>>();

            if languages_u.intersection(&languages_v).count() == 0 {
                for language in 1..=n {
                    if !languages_u.contains(&&language) {
                        teach_users
                            .entry(language)
                            .or_insert(HashSet::new())
                            .insert(u);
                    }
                    if !languages_v.contains(&&language) {
                        teach_users
                            .entry(language)
                            .or_insert(HashSet::new())
                            .insert(v);
                    }
                }
            }
        }

        teach_users.values().map(|u| u.len()).min().unwrap_or(0) as i32
    }
}
```
