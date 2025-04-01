# 1125. 最小的必要团队
作为项目经理，你规划了一份需求的技能清单 `req_skills`，并打算从备选人员名单 `people` 中选出些人组成一个「必要团队」（ 编号为 `i` 的备选人员 `people[i]` 含有一份该备选人员掌握的技能列表）。

所谓「必要团队」，就是在这个团队中，对于所需求的技能列表 `req_skills` 中列出的每项技能，团队中至少有一名成员已经掌握。可以用每个人的编号来表示团队中的成员：

* 例如，团队 `team = [0, 1, 3]` 表示掌握技能分别为 `people[0]`，`people[1]`，和 `people[3]` 的备选人员。

请你返回 **任一** 规模最小的必要团队，团队成员用人员编号表示。你可以按 **任意顺序** 返回答案，题目数据保证答案存在。

#### 示例 1:
<pre>
<strong>输入:</strong> req_skills = ["java","nodejs","reactjs"], people = [["java"],["nodejs"],["nodejs","reactjs"]]
<strong>输出:</strong> [0,2]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> req_skills = ["algorithms","math","java","reactjs","csharp","aws"], people = [["algorithms","math","java"],["algorithms","math","reactjs"],["java","csharp","aws"],["reactjs","csharp"],["csharp","math"],["aws","java"]]
<strong>输出:</strong> [1,2]
</pre>

#### 提示:
* `1 <= req_skills.length <= 16`
* `1 <= req_skills[i].length <= 16`
* `req_skills[i]` 由小写英文字母组成
* `req_skills` 中的所有字符串 **互不相同**
* `1 <= people.length <= 60`
* `0 <= people[i].length <= 16`
* `1 <= people[i][j].length <= 16`
* `people[i][j]` 由小写英文字母组成
* `people[i]` 中的所有字符串 **互不相同**
* `people[i]` 中的每个技能是 `req_skills` 中的技能
* 题目数据保证「必要团队」一定存在

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let mut skill_indices = HashMap::new();
        let mut min_team = HashMap::from([(0, vec![])]);

        for i in 0..req_skills.len() {
            skill_indices.insert(&req_skills[i], i);
        }

        for i in 0..people.len() {
            let mut person_bin = 0;

            for skill in &people[i] {
                person_bin |= 1 << skill_indices[&skill];
            }

            for (k, mut v) in min_team.clone().into_iter() {
                match min_team.get_mut(&(k | person_bin)) {
                    Some(team) if team.len() <= v.len() + 1 => (),
                    Some(team) => {
                        v.push(i as i32);
                        *team = v;
                    }
                    None => {
                        v.push(i as i32);
                        min_team.insert(k | person_bin, v);
                    }
                }
            }
        }

        min_team[&((1 << req_skills.len()) - 1)].clone()
    }
}
```
