# 2491. Divide Players Into Teams of Equal Skill
You are given a positive integer array `skill` of **even** length `n` where `skill[i]` denotes the skill of the <code>i<sup>th</sup></code> player. Divide the players into `n / 2` teams of size `2` such that the total skill of each team is **equal**.

The **chemistry** of a team is equal to the **product** of the skills of the players on that team.

Return *the sum of the **chemistry** of all the teams, or return* `-1` *if there is no way to divide the players into teams such that the total skill of each team is equal*.

#### Example 1:
<pre>
<strong>Input:</strong> skill = [3,2,5,1,3,4]
<strong>Output:</strong> 22
<strong>Explanation:</strong>
Divide the players into the following teams: (1, 5), (2, 4), (3, 3), where each team has a total skill of 6.
The sum of the chemistry of all the teams is: 1 * 5 + 2 * 4 + 3 * 3 = 5 + 8 + 9 = 22.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> skill = [3,4]
<strong>Output:</strong> 12
<strong>Explanation:</strong>
The two players form a team with a total skill of 7.
The chemistry of the team is 3 * 4 = 12.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> skill = [1,1,2,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong>
There is no way to divide the players into teams such that the total skill of each team is equal.
</pre>

#### Constraints:
* <code>2 <= skill.length <= 10<sup>5</sup></code>
* `skill.length` is even.
* `1 <= skill[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let n = skill.len();
        let mut skill = skill;
        let mut ret = 0;

        skill.sort_unstable();

        for i in 0..n / 2 {
            if skill[i] + skill[n - 1 - i] != skill[0] + skill[n - 1] {
                return -1;
            }

            ret += skill[i] as i64 * skill[n - 1 - i] as i64;
        }

        ret
    }
}
```
