# 1996. 游戏中弱角色的数量
你正在参加一个多角色游戏，每个角色都有两个主要属性：攻击 和 防御 。给你一个二维整数数组 `properties` ，其中 <code>properties[i] = [attack<sub>i</sub>, defense<sub>i</sub>]</code> 表示游戏中第 `i` 个角色的属性。

如果存在一个其他角色的攻击和防御等级 都严格高于 该角色的攻击和防御等级，则认为该角色为 弱角色 。更正式地，如果认为角色 `i` 弱于 存在的另一个角色 `j` ，那么 <code>attack<sub>j</sub> > attack<sub>i</sub></code> 且 <code>defense<sub>j</sub> > defense<sub>i</sub></code> 。

返回 弱角色 的数量。

#### 示例 1:
<pre>
<strong>输入:</strong> properties = [[5,5],[6,3],[3,6]]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在攻击和防御都严格高于其他角色的角色。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> properties = [[2,2],[3,3]]
<strong>输出:</strong> 1
<strong>解释:</strong> 第一个角色是弱角色，因为第二个角色的攻击和防御严格大于该角色。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> properties = [[1,5],[10,4],[4,3]]
<strong>输出:</strong> 1
<strong>解释:</strong> 第三个角色是弱角色，因为第二个角色的攻击和防御严格大于该角色。
</pre>

#### 提示:
* <code>2 <= properties.length <= 10<sup>5</sup></code>
* `properties[i].length == 2`
* <code>1 <= attack<sub>i</sub>, defense<sub>i</sub> <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        let mut max_defense = 0;
        let mut ret = 0;

        properties.sort_unstable_by_key(|p| (-p[0], p[1]));

        for p in &properties {
            ret += (max_defense > p[1]) as i32;
            max_defense = max_defense.max(p[1]);
        }

        ret
    }
}
```
