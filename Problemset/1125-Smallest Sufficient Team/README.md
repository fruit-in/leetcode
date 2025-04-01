# 1125. Smallest Sufficient Team
In a project, you have a list of required skills `req_skills`, and a list of people. The <code>i<sup>th</sup></code> person `people[i]` contains a list of skills that the person has.

Consider a sufficient team: a set of people such that for every required skill in `req_skills`, there is at least one person in the team who has that skill. We can represent these teams by the index of each person.

* For example, `team = [0, 1, 3]` represents the people with skills `people[0]`, `people[1]`, and `people[3]`.

Return *any sufficient team of the smallest possible size, represented by the index of each person*. You may return the answer in **any order**.

It is **guaranteed** an answer exists.

#### Example 1:
<pre>
<strong>Input:</strong> req_skills = ["java","nodejs","reactjs"], people = [["java"],["nodejs"],["nodejs","reactjs"]]
<strong>Output:</strong> [0,2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> req_skills = ["algorithms","math","java","reactjs","csharp","aws"], people = [["algorithms","math","java"],["algorithms","math","reactjs"],["java","csharp","aws"],["reactjs","csharp"],["csharp","math"],["aws","java"]]
<strong>Output:</strong> [1,2]
</pre>

#### Constraints:
* `1 <= req_skills.length <= 16`
* `1 <= req_skills[i].length <= 16`
* `req_skills[i]` consists of lowercase English letters.
* All the strings of `req_skills` are **unique**.
* `1 <= people.length <= 60`
* `0 <= people[i].length <= 16`
* `1 <= people[i][j].length <= 16`
* `people[i][j]` consists of lowercase English letters.
* All the strings of `people[i]` are **unique**.
* Every skill in `people[i]` is a skill in `req_skills`.
* It is guaranteed a sufficient team exists.

## Solutions (Rust)

### 1. Solution
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
