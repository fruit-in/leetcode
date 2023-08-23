# 1452. 收藏清单
给你一个数组 `favoriteCompanies` ，其中 `favoriteCompanies[i]` 是第 `i` 名用户收藏的公司清单（**下标从 0 开始**）。

请找出不是其他任何人收藏的公司清单的子集的收藏清单，并返回该清单下标。下标需要按升序排列。

#### 示例 1:
<pre>
<strong>输入:</strong> favoriteCompanies = [["leetcode","google","facebook"],["google","microsoft"],["google","facebook"],["google"],["amazon"]]
<strong>输出:</strong> [0,1,4]
<strong>解释:</strong>
favoriteCompanies[2]=["google","facebook"] 是 favoriteCompanies[0]=["leetcode","google","facebook"] 的子集。
favoriteCompanies[3]=["google"] 是 favoriteCompanies[0]=["leetcode","google","facebook"] 和 favoriteCompanies[1]=["google","microsoft"] 的子集。
其余的收藏清单均不是其他任何人收藏的公司清单的子集，因此，答案为 [0,1,4] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> favoriteCompanies = [["leetcode","google","facebook"],["leetcode","amazon"],["facebook","google"]]
<strong>输出:</strong> [0,1]
<strong>解释:</strong> favoriteCompanies[2]=["facebook","google"] 是 favoriteCompanies[0]=["leetcode","google","facebook"] 的子集，因此，答案为 [0,1] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> favoriteCompanies = [["leetcode"],["google"],["facebook"],["amazon"]]
<strong>输出:</strong> [0,1,2,3]
</pre>

#### 提示:
* `1 <= favoriteCompanies.length <= 100`
* `1 <= favoriteCompanies[i].length <= 500`
* `1 <= favoriteCompanies[i][j].length <= 20`
* `favoriteCompanies[i]` 中的所有字符串 **各不相同** 。
* 用户收藏的公司清单也 **各不相同** ，也就是说，即便我们按字母顺序排序每个清单， `favoriteCompanies[i] != favoriteCompanies[j]` 仍然成立。
* 所有字符串仅包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let favorite_companies = favorite_companies
            .iter()
            .map(|v| v.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let mut flag = true;
        let mut ret = vec![];

        for i in 0..favorite_companies.len() {
            flag = true;

            for j in (0..i).chain(i + 1..favorite_companies.len()) {
                if favorite_companies[i].is_subset(&favorite_companies[j]) {
                    flag = false;
                    break;
                }
            }

            if flag {
                ret.push(i as i32);
            }
        }

        ret
    }
}
```
