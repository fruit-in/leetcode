# 789. 逃脱阻碍者
你在进行一个简化版的吃豆人游戏。你从 `(0, 0)` 点开始出发，你的目的地是 `(target[0], target[1])` 。地图上有一些阻碍者，第 i 个阻碍者从 `(ghosts[i][0], ghosts[i][1])` 出发。

每一回合，你和阻碍者们*可以*同时向东，西，南，北四个方向移动，每次可以移动到距离原位置1个单位的新位置。

如果你可以在任何阻碍者抓住你之前到达目的地（阻碍者可以采取任意行动方式），则被视为逃脱成功。如果你和阻碍者同时到达了一个位置（包括目的地）都不算是逃脱成功。

当且仅当你有可能成功逃脱时，输出 True。

#### 示例 1:
<pre>
<strong>输入:</strong>
ghosts = [[1, 0], [0, 3]]
target = [0, 1]
<strong>输出:</strong> true
<strong>解释:</strong>
你可以直接一步到达目的地(0,1)，在(1, 0)或者(0, 3)位置的阻碍者都不可能抓住你。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
ghosts = [[1, 0]]
target = [2, 0]
<strong>输出:</strong> false
<strong>解释:</strong>
你需要走到位于(2, 0)的目的地，但是在(1, 0)的阻碍者位于你和目的地之间。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong>
ghosts = [[2, 0]]
target = [1, 0]
<strong>输出:</strong> false
<strong>解释:</strong>
阻碍者可以和你同时达到目的地。
</pre>

#### 说明:
* 所有的点的坐标值的绝对值 <= `10000`。
* 阻碍者的数量不会超过 `100`。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[][]} ghosts
# @param {Integer[]} target
# @return {Boolean}
def escape_ghosts(ghosts, target)
    min_distance = 20001

    ghosts.each do |ghost|
        distance = (ghost[0] - target[0]).abs + (ghost[1] - target[1]).abs
        min_distance = distance if distance < min_distance
    end

    return min_distance > target[0].abs + target[1].abs
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        ghosts
            .iter()
            .map(|v| (v[0] - target[0]).abs() + (v[1] - target[1]).abs())
            .min()
            .unwrap_or(20001)
            > (target[0].abs() + target[1].abs())
    }
}
```
