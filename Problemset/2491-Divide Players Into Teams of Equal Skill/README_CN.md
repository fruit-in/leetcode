# 2491. 划分技能点相等的团队
给你一个正整数数组 `skill` ，数组长度为 **偶数** `n` ，其中 `skill[i]` 表示第 `i` 个玩家的技能点。将所有玩家分成 `n / 2` 个 `2` 人团队，使每一个团队的技能点之和 **相等** 。

团队的 **化学反应** 等于团队中玩家的技能点 **乘积** 。

返回所有团队的 **化学反应** 之和，如果无法使每个团队的技能点之和相等，则返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> skill = [3,2,5,1,3,4]
<strong>输出:</strong> 22
<strong>解释:</strong>
将玩家分成 3 个团队 (1, 5), (2, 4), (3, 3) ，每个团队的技能点之和都是 6 。
所有团队的化学反应之和是 1 * 5 + 2 * 4 + 3 * 3 = 5 + 8 + 9 = 22 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> skill = [3,4]
<strong>输出:</strong> 12
<strong>解释:</strong>
两个玩家形成一个团队，技能点之和是 7 。
团队的化学反应是 3 * 4 = 12 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> skill = [1,1,2,3]
<strong>输出:</strong> -1
<strong>解释:</strong>
无法将玩家分成每个团队技能点都相等的若干个 2 人团队。
</pre>

#### 提示:
* <code>2 <= skill.length <= 10<sup>5</sup></code>
* `skill.length` 是偶数
* `1 <= skill[i] <= 1000`

## 题解 (Rust)

### 1. 题解
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
