# 2591. 将钱分给最多的儿童
给你一个整数 `money` ，表示你总共有的钱数（单位为美元）和另一个整数 `children` ，表示你要将钱分配给多少个儿童。

你需要按照如下规则分配：

* 所有的钱都必须被分配。
* 每个儿童至少获得 `1` 美元。
* 没有人获得 `4` 美元。

请你按照上述规则分配金钱，并返回 **最多** 有多少个儿童获得 **恰好** `8` 美元。如果没有任何分配方案，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> money = 20, children = 3
<strong>输出:</strong> 1
<strong>解释:</strong>
最多获得 8 美元的儿童数为 1 。一种分配方案为：
- 给第一个儿童分配 8 美元。
- 给第二个儿童分配 9 美元。
- 给第三个儿童分配 3 美元。
没有分配方案能让获得 8 美元的儿童数超过 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> money = 16, children = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 每个儿童都可以获得 8 美元。
</pre>

#### 提示:
* `1 <= money <= 200`
* `2 <= children <= 30`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        let money = money - children;

        if money < 0 {
            -1
        } else if money / 7 == children && money % 7 == 0 {
            children
        } else if money / 7 >= children {
            children - 1
        } else if money / 7 == children - 1 && money % 7 == 3 {
            children - 2
        } else {
            money / 7
        }
    }
}
```
