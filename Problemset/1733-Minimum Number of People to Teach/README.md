# 1733. Minimum Number of People to Teach
On a social network consisting of `m` users and some friendships between users, two users can communicate with each other if they know a common language.

You are given an integer `n`, an array `languages`, and an array `friendships` where:

* There are `n` languages numbered `1` through `n`,
* `languages[i]` is the set of languages the <code>i<sup>th</sup></code> user knows, and
* <code>friendships[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> denotes a friendship between the users <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.

You can choose **one** language and teach it to some users so that all friends can communicate with each other. Return *the **minimum** number of users you need to teach*.
Note that friendships are not transitive, meaning if `x` is a friend of `y` and `y` is a friend of `z`, this doesn't guarantee that `x` is a friend of `z`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can either teach user 1 the second language or user 2 the first language.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Teach the third language to users 1 and 3, yielding two users to teach.
</pre>

#### Constraints:
* `2 <= n <= 500`
* `languages.length == m`
* `1 <= m <= 500`
* `1 <= languages[i].length <= n`
* `1 <= languages[i][j] <= n`
* <code>1 <= u<sub>i</sub> < v<sub>i</sub> <= languages.length</code>
* `1 <= friendships.length <= 500`
* All tuples <code>(u<sub>i</sub>, v<sub>i</sub>)</code> are unique
* `languages[i]` contains only unique values

## Solutions (Rust)

### 1. Solution
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
