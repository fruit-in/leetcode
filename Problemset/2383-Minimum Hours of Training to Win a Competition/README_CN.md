# 2383. 赢得比赛需要的最少训练时长
你正在参加一场比赛，给你两个 **正** 整数 `initialEnergy` 和 `initialExperience` 分别表示你的初始精力和初始经验。

另给你两个下标从 **0** 开始的整数数组 `energy` 和 `experience`，长度均为 `n` 。

你将会 **依次** 对上 `n` 个对手。第 `i` 个对手的精力和经验分别用 `energy[i]` 和 `experience[i]` 表示。当你对上对手时，需要在经验和精力上都 **严格** 超过对手才能击败他们，然后在可能的情况下继续对上下一个对手。

击败第 `i` 个对手会使你的经验 **增加** `experience[i]`，但会将你的精力 **减少**  `energy[i]` 。

在开始比赛前，你可以训练几个小时。每训练一个小时，你可以选择将增加经验增加 1 **或者** 将精力增加 1 。

返回击败全部 `n` 个对手需要训练的 **最少** 小时数目。

#### 示例 1:
<pre>
<strong>输入:</strong> initialEnergy = 5, initialExperience = 3, energy = [1,4,3,2], experience = [2,6,3,1]
<strong>输出:</strong> 8
<strong>解释:</strong> 在 6 小时训练后，你可以将精力提高到 11 ，并且再训练 2 个小时将经验提高到 5 。
按以下顺序与对手比赛：
- 你的精力与经验都超过第 0 个对手，所以获胜。
  精力变为：11 - 1 = 10 ，经验变为：5 + 2 = 7 。
- 你的精力与经验都超过第 1 个对手，所以获胜。
  精力变为：10 - 4 = 6 ，经验变为：7 + 6 = 13 。
- 你的精力与经验都超过第 2 个对手，所以获胜。
  精力变为：6 - 3 = 3 ，经验变为：13 + 3 = 16 。
- 你的精力与经验都超过第 3 个对手，所以获胜。
  精力变为：3 - 2 = 1 ，经验变为：16 + 1 = 17 。
在比赛前进行了 8 小时训练，所以返回 8 。
可以证明不存在更小的答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> initialEnergy = 2, initialExperience = 4, energy = [1], experience = [3]
<strong>输出:</strong> 0
<strong>解释:</strong> 你不需要额外的精力和经验就可以赢得比赛，所以返回 0 。
</pre>

#### 提示:
* `n == energy.length == experience.length`
* `1 <= n <= 100`
* `1 <= initialEnergy, initialExperience, energy[i], experience[i] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut initial_energy = initial_energy;
        let mut initial_experience = initial_experience;
        let mut ret = 0;

        for i in 0..energy.len() {
            if initial_energy <= energy[i] {
                ret += energy[i] + 1 - initial_energy;
                initial_energy = energy[i] + 1;
            }
            if initial_experience <= experience[i] {
                ret += experience[i] + 1 - initial_experience;
                initial_experience = experience[i] + 1;
            }

            initial_energy -= energy[i];
            initial_experience += experience[i];
        }

        ret
    }
}
```
