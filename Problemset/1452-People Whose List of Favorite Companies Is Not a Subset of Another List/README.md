# 1452. People Whose List of Favorite Companies Is Not a Subset of Another List
Given the array `favoriteCompanies` where `favoriteCompanies[i]` is the list of favorites companies for the `ith` person (**indexed from 0**).

*Return the indices of people whose list of favorite companies is not a **subset** of any other list of favorites companies*. You must return the indices in increasing order.

#### Example 1:
<pre>
<strong>Input:</strong> favoriteCompanies = [["leetcode","google","facebook"],["google","microsoft"],["google","facebook"],["google"],["amazon"]]
<strong>Output:</strong> [0,1,4]
<strong>Explanation:</strong>
Person with index=2 has favoriteCompanies[2]=["google","facebook"] which is a subset of favoriteCompanies[0]=["leetcode","google","facebook"] corresponding to the person with index 0.
Person with index=3 has favoriteCompanies[3]=["google"] which is a subset of favoriteCompanies[0]=["leetcode","google","facebook"] and favoriteCompanies[1]=["google","microsoft"].
Other lists of favorite companies are not a subset of another list, therefore, the answer is [0,1,4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> favoriteCompanies = [["leetcode","google","facebook"],["leetcode","amazon"],["facebook","google"]]
<strong>Output:</strong> [0,1]
<strong>Explanation:</strong> In this case favoriteCompanies[2]=["facebook","google"] is a subset of favoriteCompanies[0]=["leetcode","google","facebook"], therefore, the answer is [0,1].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> favoriteCompanies = [["leetcode"],["google"],["facebook"],["amazon"]]
<strong>Output:</strong> [0,1,2,3]
</pre>

#### Constraints:
* `1 <= favoriteCompanies.length <= 100`
* `1 <= favoriteCompanies[i].length <= 500`
* `1 <= favoriteCompanies[i][j].length <= 20`
* All strings in `favoriteCompanies[i]` are **distinct**.
* All lists of favorite companies are **distinct**, that is, If we sort alphabetically each list then `favoriteCompanies[i] != favoriteCompanies[j]`.
* All strings consist of lowercase English letters only.

## Solutions (Rust)

### 1. Solution
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
